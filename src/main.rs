use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::io::{BufRead, BufReader};

fn main() {
   
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	fn handle_client(stream: TcpStream) {
		let lines = BufReader::new(stream.try_clone().unwrap()).lines();
        for line in lines {
            match line {
            	Ok(text) => { println!("Received: {}", text) }
            	Err(_) => { println!("Ohhhhhhh SHIT!!!!") }
            }
        }
	}

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
    	match stream {
        	Ok(stream) => {
        		match stream.peer_addr() {
        			Ok(socket_addr) => {
        				match socket_addr {
        					SocketAddr::V4(ipv_4_socket) => {println!("New connection using IP V4 from: ip: {}, port: {}", ipv_4_socket.ip(), ipv_4_socket.port());}
        					SocketAddr::V6(ipv_6_socket) => {println!("V6 - Unimplemented!");}
        				} 
        			}
        			Err(_) => panic!("Oh dear")
        		};
            	thread::spawn(move|| {
                	// connection succeeded
                	handle_client(stream)
            	});
 	        }
        	Err(e) => {println!("Shit be fooked!: {}", e) }
    	}
	}

	// close the socket server
	drop(listener);
}
