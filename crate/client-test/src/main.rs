// connection to server balancer
use renet::{
  DefaultChannel, RenetClient, ConnectionConfig,
  transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError}
};
use std::time::{Duration, SystemTime};
use std::net::{UdpSocket, SocketAddr, Ipv4Addr, IpAddr};

fn main() {
  let mut client = RenetClient::new(ConnectionConfig::default());

  // Setup transport layer
  const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 2007);
  let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
  let client_id: u64 = 0;
  let authentication = ClientAuthentication::Unsecure {
      server_addr: SERVER_ADDR,
      client_id,
      user_data: None,
      protocol_id: 0,
  };

  let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

  // Your gameplay loop
  loop {
      let delta_time = Duration::from_millis(16);
      // Receive new messages and update client
      client.update(delta_time);
      transport.update(delta_time, &mut client).unwrap();
      
      if transport.is_connected() {
          // Receive message from server
          while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
            println!("{:#?}", message);
          }
          
          // Send message
          client.send_message(DefaultChannel::ReliableOrdered, "client text".as_bytes().to_vec());
      }
   
      // Send packets to server
      let _ = transport.send_packets(&mut client);
  }
}

