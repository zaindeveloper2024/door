use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Read, Write};
use std::collections::HashMap;
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
        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut  request_line)?;
        let headers = self.parse_headers(&mut reader)?;
        println!("Headers: {:?}", headers);

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
        stream.write(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }


    fn parse_headers(&self, reader: &mut BufReader<&TcpStream>) -> std::io::Result<HashMap<String, String>> {
        let mut headers = HashMap::new();
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }
        Ok(headers)
    }
}
