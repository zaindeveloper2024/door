use door::init_config;
use door::server::Server;

fn main() {
    let config = init_config();
    if let Err(e) = Server::http("127.0.0.1:8000") {
        eprintln!("Error: {}", e);
    }
}
