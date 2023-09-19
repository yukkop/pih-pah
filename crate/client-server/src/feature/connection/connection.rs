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
