use mio::*;
use mio::tcp::*;
use http::HttpParser;

pub struct ClientConnection {
  pub socket: TcpStream,
  pub interest: EventSet
}

impl ClientConnection {
  pub fn new(socket: TcpStream) -> ClientConnection {
    ClientConnection {
      socket: socket,
      interest: EventSet::readable()
    }
  }

  pub fn read(&mut self) -> bool {
    let mut buf = [0; 2048];
    match self.socket.try_read(&mut buf) {
      Err(e) => {
        println!("Error while reading socket: {:?}", e);
        false
      },
      Ok(None) => {
        println!("nothing to read???");
        false
      },
      Ok(Some(len)) => {
        match len {
          x if x > 0 => {
            let http_request = HttpParser::parse_http_request(&buf);
            println!("Client({}): Req Method: {:?}, Path: {}", 
              self.socket.peer_addr().unwrap().port(), 
              http_request.0,
              http_request.1);
            true
          }
          _ => {
            println!("Disconnecting Client({}) after receiving 0 length data", self.socket.peer_addr().unwrap().port());
            false
          }
        }
      }
    }
  }
}