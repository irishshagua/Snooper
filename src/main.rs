extern crate rustc_serialize;
extern crate httparse;
extern crate docopt;
extern crate mio;

mod http;
mod server;
mod client;

use mio::*;
use mio::tcp::*;

use docopt::Docopt;

use std::net::SocketAddr;

use server::ProxyServer;
use server::SERVER_TOKEN;

const USAGE: &'static str = "
Snooper

Usage:
  snooper [--ip <ip> --port <port>]
  
Options:
  -h --help          Show this screen.
  --version          Show version.
  --ip=<addr>        Network address to bind to [default: 127.0.0.1]
  --port=<port>      Port to bind to [default: 3128]
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

  let address = format!("{}:{}", args.flag_ip, args.flag_port)
    .parse::<SocketAddr>().unwrap();
  let listener = TcpListener::bind(&address).unwrap();

  let mut event_loop = EventLoop::new().unwrap();
  let mut server = ProxyServer::new(listener);

  event_loop.register(&server.socket,
    SERVER_TOKEN,
    EventSet::readable(),
    PollOpt::edge()).unwrap();

  event_loop.run(&mut server).unwrap();
}
