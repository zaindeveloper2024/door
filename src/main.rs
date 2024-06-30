use door::init_config;
use door::server::Server;

fn main() {
    let config = init_config();
    Server::http("addr".to_string());
}
