use dotenv::dotenv;
use renet::{
  transport::{
    NetcodeServerTransport, ServerAuthentication, ServerConfig, NETCODE_USER_DATA_BYTES,
  },
  ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};
use sqlx::postgres::PgPool;
use std::{
  collections::HashMap,
  net::{SocketAddr, UdpSocket},
  thread,
  time::{Duration, Instant, SystemTime},
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  env_logger::init();
  println!("Usage: load-balancer.rs [SERVER_PORT] ");

  // db
  dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = PgPool::connect(&database_url).await?;

  // get port
  let args: Vec<String> = std::env::args().collect();

  let server_addr: SocketAddr = format!("0.0.0.0:{}", args[1]).parse().unwrap();
  server(server_addr, pool).await?;

  Ok(())
}

const PROTOCOL_ID: u64 = 7;
async fn server(public_addr: SocketAddr, pool: PgPool) -> Result<(), sqlx::Error> {
  let connection_config = ConnectionConfig::default();
  let mut server: RenetServer = RenetServer::new(connection_config);

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
  let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();

  let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

  let mut addresses: HashMap<ClientId, String> = HashMap::new();
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
          let data = transport.user_data(client_id).unwrap();
          let addr = from_user_data(&data);
          addresses.insert(client_id, addr.clone());

          println!("{}", addr);
          sqlx::query("UPDATE server SET online = true WHERE address = $1;")
            .bind(addr)
            .execute(&pool)
            .await?;

          println!("Client {} connected.", client_id)
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
          println!("Client {} disconnected: {}", client_id, reason);

          sqlx::query("UPDATE server SET online = false WHERE address = $1")
            .bind(addresses.get(&client_id))
            .execute(&pool)
            .await?;

          addresses.remove_entry(&client_id);
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

fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> String {
  let mut buffer = [0u8; 8];
  buffer.copy_from_slice(&user_data[0..8]);
  let mut len = u64::from_le_bytes(buffer) as usize;
  len = len.min(NETCODE_USER_DATA_BYTES - 8);
  let data = user_data[8..len + 8].to_vec();
  String::from_utf8(data).unwrap()
}
