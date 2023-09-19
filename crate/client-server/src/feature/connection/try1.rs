// connection to server balancer
use renet::{
  DefaultChannel, RenetClient, ConnectionConfig,
  transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError}
};
use std::time::{Duration, SystemTime};
use std::net::{UdpSocket, SocketAddr, Ipv4Addr, IpAddr};
use bevy::prelude::*;

pub struct ConnectionPlugins;

#[derive(Resource)]
struct ConnectionTransport(NetcodeClientTransport);

#[derive(Resource)]
struct ConnectionClient(RenetClient);

impl Plugin for ConnectionPlugins {
  fn build(&self, app: &mut App) {
    let (client, transport) = init_connection();
    app
      .insert_resource(ConnectionTransport(transport))
      .insert_resource(ConnectionClient(client))
      .add_systems(Update, report_server_status);
  }
}

// impl Plugin for MultiplayerPlugins {
//   fn build(&self, app: &mut App) {
//     app.init_resource::<Lobby>();
//     app.init_resource::<TransportData>();
//     app.add_plugins(RenetClientPlugin);
//     app.add_plugins(NetcodeClientPlugin);
//     app.init_resource::<PlayerInput>();
//     app.init_resource::<OwnId>();
//
//     let (client, transport) = new_renet_client(self.server_addr.to_string());
//     app.insert_resource(client);
//     app.insert_resource(transport);
//
//     app.add_systems(
//       Update,
//       (
//         player_input,
//         camera_switch, // TODO maybe separate
//         client_send_input,
//         client_sync_players,
//       )
//         .run_if(bevy_renet::transport::client_connected()),
//     );
//   }
// }

pub fn new_renet_client(addr: String) -> (RenetClient, NetcodeClientTransport) {
  let client = RenetClient::new(ConnectionConfig::default());
  let server_addr = addr.parse().unwrap();
  let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
  let current_time = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap();
  let client_id = current_time.as_millis() as u64;
  let authentication = ClientAuthentication::Unsecure {
    client_id,
    protocol_id: PROTOCOL_ID,
    server_addr,
    user_data: None,
  };

  let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

  (client, transport)
}

pub fn client_sync_players(
  mut client: ResMut<RenetClient>,
) {
}
