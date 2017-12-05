
#[allow(unused)]
use std::net::{SocketAddr, TcpStream, TcpListener};

fn main() {
  /*
  let addrs = [
      SocketAddr::from(([127, 0, 0, 1], 8080)),
      SocketAddr::from(([127, 0, 0, 1], 8081)),
  ];

  if let Ok(stream) = TcpStream::connect(&addrs[..]) {
      println!("Connected to the server!");
  } else {
      println!("Couldn't connect to server...");
  }
  */
  use std::net::{SocketAddr, TcpListener};

  let addrs = [
      SocketAddr::from(([127, 0, 0, 1], 3000)),
      SocketAddr::from(([127, 0, 0, 1], 3001)),
  ];
  let listener = TcpListener::bind(&addrs[..]).unwrap();
  match listener.accept() {
      Ok((_socket, addr)) => println!("new client: {:?}", addr),
      Err(e) => println!("couldn't get client: {:?}", e),
  }
}
