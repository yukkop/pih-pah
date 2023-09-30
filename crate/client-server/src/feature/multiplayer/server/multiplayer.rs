use bevy::prelude::*;

use crate::feature::lobby::server::spawn_server_side_player;
use crate::feature::multiplayer::{
  Lobby, Player, PlayerData, PlayerInput, PlayerViewDirrection, ServerMessages, TransportData,
  PROTOCOL_ID, PlayerTransportData, Username,
};
use bevy_renet::{
  transport::NetcodeServerPlugin, RenetServerPlugin,
  renet::{
    transport::{ServerAuthentication, ServerConfig},
    ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
  }
};
use bevy_xpbd_3d::prelude::*;
use renet::transport::NetcodeServerTransport;

use std::net::UdpSocket;
use std::time::SystemTime;

pub struct MultiplayerPlugins{
  server_addr: String
}

impl MultiplayerPlugins {
  pub fn by_string(server_addr: String) -> Self {
    Self {
      server_addr
    }
  }
}

impl Plugin for MultiplayerPlugins {
  fn build(&self, app: &mut App) {
    app.init_resource::<Lobby>();
    app.init_resource::<TransportData>();

    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);

    let (server, transport) = new_renet_server(self.server_addr.as_str());

    app.insert_resource(server);
    app.insert_resource(transport);

    app.add_systems(
      Update,
      (server_update_system, server_sync_players).run_if(resource_exists::<RenetServer>()),
    );
  }
}

pub fn new_renet_server(addr: &str) -> (RenetServer, NetcodeServerTransport) {
  let server = RenetServer::new(ConnectionConfig::default());

  let public_addr = addr.parse().unwrap();
  let socket = UdpSocket::bind(public_addr).unwrap();
  let current_time = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap();
  let server_config = ServerConfig {
    current_time,
    max_clients: 64,
    protocol_id: PROTOCOL_ID,
    public_addresses: vec![public_addr],
    authentication: ServerAuthentication::Unsecure,
  };

  let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

  (server, transport)
}

pub fn generate_player_color(player_number: u32) -> Color {
  let golden_angle = 137.5;
  let hue = (golden_angle * player_number as f32) % 360.0;
  Color::hsl(hue, 1.0, 0.5)
}

pub fn server_update_system(
  mut server_events: EventReader<ServerEvent>,
  mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<StandardMaterial>>,
  mut lobby: ResMut<Lobby>,
  mut server: ResMut<RenetServer>,
  transport: Res<NetcodeServerTransport>,
) {
  for event in server_events.iter() {
    match event {
      ServerEvent::ClientConnected { client_id } => {
        log::info!("Player {} connected.", client_id);

        // Spawn player cube
        let player_entity = commands.spawn_server_side_player(*client_id).id();

        // TODO remove
        let message =
          bincode::serialize(&ServerMessages::InitConnection { id: *client_id }).unwrap();
        server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);

        lobby.players_seq += 1;
        let color = generate_player_color(lobby.players_seq as u32);


        // We could send an InitState with all the players id and positions for the client
        // but this is easier to do.
        for (player_id, player_data) in &lobby.players {
          let message =
            bincode::serialize(&ServerMessages::PlayerConnected { id: *player_id, color: player_data.color, username: player_data.username.clone() }).unwrap();
          server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
        }

        let data = transport.user_data(*client_id).unwrap();
        let username = match Username::from_user_data(&data) {
          Ok(name) => name,
          Err(_) => "@corapted@".to_string(),
        };
        // let username = "noname".to_string();

        lobby.players.insert(
          *client_id,
          PlayerData {
            entity: player_entity,
            color,
            username: username.clone()
          },
        );

        let message =
          bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id, color, username }).unwrap();
        server.broadcast_message(DefaultChannel::ReliableOrdered, message);
      }
      ServerEvent::ClientDisconnected { client_id, reason } => {
        log::info!("Player {} disconnected: {}", client_id, reason);
        if let Some(player_data) = lobby.players.remove(client_id) {
          commands.entity(player_data.entity).despawn();
        }

        let message =
          bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id }).unwrap();
        server.broadcast_message(DefaultChannel::ReliableOrdered, message);
      }
    }
  }

  for client_id in server.clients_id().into_iter() {
    while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
      let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
      if let Some(player_data) = lobby.players.get(&client_id) {
        commands.entity(player_data.entity).insert(player_input);
      }
    }
  }
}

pub fn server_sync_players(
  mut server: ResMut<RenetServer>,
  mut data: ResMut<TransportData>,
  query: Query<(&Position, &Rotation, &PlayerViewDirrection, &Player)>,
) {
  for (position, rotation, view_dirrection, player) in query.iter() {
    data
      .data
      .insert(player.id, PlayerTransportData {
        position: position.0.into(),
        rotation: rotation.0.into(),
        tied_camera_rotation: view_dirrection.0.into()
      });
  }

  let sync_message = bincode::serialize(&data.data).unwrap();
  server.broadcast_message(DefaultChannel::Unreliable, sync_message);

  data.data.clear();
}
