use mio::*;
use mio::tcp::*;

use std::collections::HashMap;

use client::ClientConnection;

pub const SERVER_TOKEN: Token = Token(0);

pub struct ProxyServer {
  pub socket: TcpListener,
  pub clients: HashMap<Token, ClientConnection>,
  token_counter: usize
}

impl ProxyServer {
  pub fn new(socket: TcpListener) -> ProxyServer {
    ProxyServer {
      socket: socket,
      clients: HashMap::new(),
      token_counter: 1
    }
  }
}

impl Handler for ProxyServer {
  type Timeout = usize;
  type Message = ();

  fn ready(&mut self, event_loop: &mut EventLoop<ProxyServer>, token: Token, events: EventSet) {
    if events.is_readable() {
      match token {
        SERVER_TOKEN => {
          let client_socket = match self.socket.accept() {
            Ok(Some((sock, addr))) => {
              println!("Server: New client connected from 127.0.0.1:{}. Now {} clients connected", addr.port(), self.clients.len() + 1);
              sock
            },
            Ok(None) => unreachable!(),
            Err(e) => {
                println!("Accept error: {}", e);
                return;
            }
          };

          let new_token = Token(self.token_counter);
          self.clients.insert(new_token, ClientConnection::new(client_socket));
          self.token_counter += 1;

          event_loop.register(&self.clients[&new_token].socket, new_token, EventSet::readable(),
                              PollOpt::edge() | PollOpt::oneshot()).unwrap();
        },
        token => {
          if { self.clients.get_mut(&token).unwrap().read() } {
            let client = self.clients.get_mut(&token).unwrap();
            event_loop.reregister(&client.socket, token, client.interest, PollOpt::edge() | PollOpt::oneshot()).unwrap();
          } else {
            self.clients.remove(&token);
            println!("removing token {}", token.as_usize());
          }
        }
      }
    }
  }
}