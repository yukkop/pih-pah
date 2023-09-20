use std::{
  collections::HashMap,
  net::{SocketAddr, UdpSocket},
  thread,
  time::{Duration, Instant, SystemTime},
};
use renet::{
  transport::{
    NetcodeServerTransport, ServerAuthentication, ServerConfig,
  },
  ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};

fn main() {
  env_logger::init();
  println!("Usage: load-balancer.rs [SERVER_PORT] ");
  let args: Vec<String> = std::env::args().collect();

  let server_addr: SocketAddr = format!("0.0.0.0:{}", args[1]).parse().unwrap();
  server(server_addr);
}

const PROTOCOL_ID: u64 = 7;

fn server(public_addr: SocketAddr) {
  let connection_config = ConnectionConfig::default();
  let mut server: RenetServer = RenetServer::new(connection_config);

  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
  let server_config = ServerConfig {
    current_time,
    max_clients: 64,
    protocol_id: PROTOCOL_ID,
    public_addresses: vec![public_addr],
    authentication: ServerAuthentication::Unsecure,
  };
  let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();

  let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

  let mut usernames: HashMap<ClientId, String> = HashMap::new();
  let mut last_updated = Instant::now();

  loop {
    let now = Instant::now();
    let duration = now - last_updated;
    last_updated = now;

    server.update(duration);
    transport.update(duration, &mut server).unwrap();

    while let Some(event) = server.get_event() {
      match event {
        ServerEvent::ClientConnected { client_id } => {
          println!("Client {} connected.", client_id)
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
          println!("Client {} disconnected: {}", client_id, reason);
          usernames.remove_entry(&client_id);
        }
      }
    }

    for client_id in server.clients_id() {
      while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
        let text = String::from_utf8(message.into()).unwrap();
        // let username = usernames.get(&client_id).unwrap();
        println!("{:?} | Server {} status: {}", now, client_id, text);
      }
    }

    transport.send_packets(&mut server);
    thread::sleep(Duration::from_millis(50));
  }
}
