use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, Instant, SystemTime},
};
use std::net::{IpAddr, Ipv4Addr};
use renet::{
    transport::{
        ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig, NETCODE_USER_DATA_BYTES,
    },
    ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
};

fn main() {

  let mut server = RenetServer::new(ConnectionConfig::default());

  // Setup transport layer
  const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2007);
  let socket: UdpSocket = UdpSocket::bind(SERVER_ADDR).unwrap();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
  let server_config = ServerConfig {
      current_time, 
      max_clients: 64,
      protocol_id: 7,
      public_addresses: vec![SERVER_ADDR],
      authentication: ServerAuthentication::Unsecure
  };
  let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

  // Your gameplay loop
  loop {
      let delta_time = Duration::from_millis(16);
      // Receive new messages and update clients
      server.update(delta_time);
      let _ = transport.update(delta_time, &mut server);
      
      // Check for client connections/disconnections
      while let Some(event) = server.get_event() {
          match event {
              ServerEvent::ClientConnected { client_id } => {
                  println!("Client {client_id} connected");
              }
              ServerEvent::ClientDisconnected { client_id, reason } => {
                  println!("Client {client_id} disconnected: {reason}");
              }
          }
      }

      // Receive message from channel
      for &client_id in server.clients_id().iter() {
          // The enum DefaultChannel describe the channels used by the default configuration
          while let Some(message) = server
            .receive_message(client_id, DefaultChannel::ReliableOrdered) {
              println!("{:#?}", message);
          }
      }
      
      // Send a text message for all clients
      server.broadcast_message(DefaultChannel::ReliableOrdered, "server message".as_bytes().to_vec());
      
      // Send message to only one client
      // let client_id = 0; 
      // server.send_message(client_id, DefaultChannel::ReliableOrdered, "server message".as_bytes().to_vec());
   
      // Send packets to clients
      transport.send_packets(&mut server);
  }
}
