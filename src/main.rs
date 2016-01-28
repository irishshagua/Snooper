extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::io::{BufRead, BufReader};

const USAGE: &'static str = "
Snooper

Usage:
  snooper [--ip <ip> --port <port>]
  
Options:
  -h --help          Show this screen.
  --version          Show version.
  --ip=<addr>   Network address to bind to [default: 127.0.0.1]
  --port=<port>  Port to bind to [default: 3128]
";

#[derive(RustcDecodable)]
struct Args {
  flag_port: u16,
  flag_ip: String,
}

fn main() {
  
  let args: Args = Docopt::new(USAGE)
    .and_then(|dopt| dopt.decode())
    .unwrap_or_else(|e| e.exit());

  let bind_address = format!("{}:{}", args.flag_ip, args.flag_port);
  let listener = TcpListener::bind(&*bind_address).unwrap();

  println!("Snooper has started and is bound to: {}", bind_address);

  fn handle_client(stream: TcpStream) {
    let lines = BufReader::new(stream.try_clone().unwrap()).lines();
    for line in lines {
      match line {
        Ok(text) => { println!("Received: {}", text) }
        Err(_) => { println!("Ohhhhhhh SHIT!!!!") }
      }
    }
	}

	// accept connections and process them, spawning a new thread for each one TODO: Update to metal IO event notifier
	for stream in listener.incoming() {
	  match stream {
      Ok(stream) => {
        match stream.peer_addr() {
          Ok(socket_addr) => {
            match socket_addr {
              SocketAddr::V4(ipv_4_socket) => {
                println!("New connection using IPv4 from: ip: {}, port: {}", ipv_4_socket.ip(), ipv_4_socket.port())
              }
              SocketAddr::V6(ipv_6_socket) => {
                println!("New connection using IPv6 from ip {} on poer {}", ipv_6_socket.ip(), ipv_6_socket.port())
              }
            }
          }
          Err(_) => panic!("Oh dear")
        };
        thread::spawn(move|| {
            // connection succeeded
            handle_client(stream)
        });
      }
      Err(e) => {
        println!("Shit be fooked!: {}", e) 
      }
	  }
	}

	// close the socket server
	drop(listener);
}
