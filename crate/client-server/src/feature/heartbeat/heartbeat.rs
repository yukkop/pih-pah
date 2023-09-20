use renet::{
  RenetClient, ConnectionConfig,
  transport::{ClientAuthentication, NetcodeClientTransport},
};
use std::time::SystemTime;
use std::net::UdpSocket;
use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;

pub const PROTOCOL_ID: u64 = 7;

// connection to loader balancer

pub struct HeartbeatPlugins {
  server_addr: String,
}

impl Plugin for HeartbeatPlugins {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(RenetClientPlugin)
      .add_plugins(NetcodeClientPlugin);

    let (client, transport) = new_renet_client(self.server_addr.to_string());
    app.insert_resource(client);
    app.insert_resource(transport);
    // app.insert_resource(HeartbearConfig {
    //   timer: Timer::new(Duration::from_secs(30), TimerMode::Repeating)
    // });

    // let schedule = Schedule::default();

    // app.add_systems(
    //   Update, repost_status.run_if(bevy_renet::transport::client_connected()),
    // );
  }
}

impl HeartbeatPlugins {
  pub fn by_string(server_addr: String) -> Self {
    Self {
      server_addr
    }
  }
}

// #[derive(Resource)]
// struct HeartbearConfig {
//   timer: Timer,
// }
//
// fn repost_status(
//   time: Res<Time>,
//   mut config: ResMut<HeartbearConfig>,
//   mut client: ResMut<RenetClient>
// ) {
//   config.timer.tick(time.delta());
//
//   if config.timer.finished() {
//     let input_message = bincode::serialize("ok").unwrap();
//
//     client.send_message(DefaultChannel::ReliableOrdered, input_message);
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
