use door::init_config;
use door::option::ListenAddr;
use door::server::Server;

fn main() {
    let config = init_config();
    println!("{:?}", config);

    let ip_addrs = vec![
        "127.0.0.1:8080".parse().unwrap(),
        "192.168.1.1:8080".parse().unwrap(),
    ];
    let config_ip = ListenAddr::IP(ip_addrs);
    println!("{:?}", config_ip);

    if let Err(e) = Server::http("127.0.0.1:8000") {
        eprintln!("Error: {}", e);
    }
}
