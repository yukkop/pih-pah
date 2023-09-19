use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, Instant, SystemTime},
};
const PROTOCOL_ID: u64 = 7;
use renet::{
    transport::{
        ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig, NETCODE_USER_DATA_BYTES as NETCODE_CLIENT_DATA_BYTES,
    },
    ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
};

#[derive(Resource)]
struct ConnectionTransport(NetcodeClientTransport);

#[derive(Resource)]
struct ConnectionClient(RenetClient);

// Helper struct to pass an username in the user data
struct ServerName(String);

impl ServerName {
    fn to_netcode(&self) -> [u8; NETCODE_CLIENT_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_CLIENT_DATA_BYTES];
        if self.0.len() > NETCODE_CLIENT_DATA_BYTES - 8 {
            panic!("Username is too big");
        }
        user_data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
        user_data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());

        user_data
    }
}

pub struct ConnectionPlugins {
  server_addr: String,
}

impl Plugin for ConnectionPlugins {
  fn build(&self, app: &mut App) {
    let (client, transport) = init_connection();
    app
      .insert_resource(ConnectionTransport(transport))
      .insert_resource(ConnectionClient(client))
      .add_systems(Update, report_server_status);
  }
}

impl MultiplayerPlugins {
  pub fn by_string(server_addr: String) -> Self {
    Self {
      server_addr
    }
  }
}

fn main() {
    let server_addr: SocketAddr = args[1].parse().unwrap();
    let username = Username(args[2].clone());

    let connection_config = ConnectionConfig::default();
    let mut client = RenetClient::new(connection_config);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: Some(username.to_netcode()),
        protocol_id: PROTOCOL_ID,
    };

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let stdin_channel: Receiver<String> = spawn_stdin_channel();

    let mut last_updated = Instant::now();
}

fn client(server_addr: SocketAddr, username: Username) {
    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        client.update(duration);
        transport.update(duration, &mut client).unwrap();

        if transport.is_connected() {
            match stdin_channel.try_recv() {
                Ok(text) => client.send_message(DefaultChannel::ReliableOrdered, text.as_bytes().to_vec()),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            while let Some(text) = client.receive_message(DefaultChannel::ReliableOrdered) {
                let text = String::from_utf8(text.into()).unwrap();
                println!("{}", text);
            }
        }

        transport.send_packets(&mut client).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
}
