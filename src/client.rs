use mio::*;
use mio::tcp::*;

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
  
  pub fn read(&mut self) {
    loop {
      let mut buf = [0; 2048];
      match self.socket.try_read(&mut buf) {
        Err(e) => {
          println!("Error while reading socket: {:?}", e);
          return
        },
        Ok(None) => break,
        _ => {
          println!("Client({}): {}", self.socket.peer_addr().unwrap().port(), String::from_utf8_lossy(&buf));
          break;
        }
      }
    }
  }
}