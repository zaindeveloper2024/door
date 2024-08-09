use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;

pub struct Server {
    addr: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Server {
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        println!("Listening on {}", self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
        stream.write(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
