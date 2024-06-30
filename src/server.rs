use std::net::{TcpListener, TcpStream};
use std::str;

pub struct Server {
    addr: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: String,
}

fn handle_client(stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
}

impl Server {
    #[inline]
    pub fn http(addr: &str) -> std::io::Result<Server> {
        Server::new(addr)
    }

    pub fn new(addr: &str) -> std::io::Result<Server> {
        Self::listen(addr)
    }

    pub fn listen(addr: &str) -> std::io::Result<Server> {
        let listener = TcpListener::bind(addr)?;
        println!("Listening on http://{}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        Ok(Server {
            addr: addr.to_string(),
        })
    }
}
