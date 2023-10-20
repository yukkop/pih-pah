use bevy::prelude::*;
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
use renet::{
  transport::{ClientAuthentication, NetcodeClientTransport, NETCODE_USER_DATA_BYTES},
  ConnectionConfig, RenetClient,
};
use std::net::UdpSocket;
use std::time::SystemTime;

pub const PROTOCOL_ID: u64 = 7;

// connection to loader balancer

pub struct HeartbeatPlugins {
  reciever_addr: String, // addres to repost heart beat
  server_addr: String,   // listening addres
}

impl Plugin for HeartbeatPlugins {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(RenetClientPlugin)
      .add_plugins(NetcodeClientPlugin);

    let (client, transport) =
      new_renet_client(self.reciever_addr.to_string(), self.server_addr.to_string());
    app.insert_resource(client);
    app.insert_resource(transport);
  }
}

impl HeartbeatPlugins {
  pub fn by_string(reciever_addr: String, server_addr: String) -> Self {
    Self {
      server_addr,
      reciever_addr,
    }
  }
}

fn addr_to_netcode_data(addr: &str) -> [u8; NETCODE_USER_DATA_BYTES] {
  let mut data = [0u8; NETCODE_USER_DATA_BYTES];
  if addr.len() > NETCODE_USER_DATA_BYTES - 8 {
    panic!("Client data to long, cringe this shouldn't have happened");
  }
  data[0..8].copy_from_slice(&(addr.len() as u64).to_le_bytes());
  data[8..addr.len() + 8].copy_from_slice(addr.as_bytes());

  data
}

pub fn new_renet_client(
  addr: String,
  listening_addr: String,
) -> (RenetClient, NetcodeClientTransport) {
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
    user_data: Some(addr_to_netcode_data(listening_addr.as_str())),
  };

  let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

  (client, transport)
}
