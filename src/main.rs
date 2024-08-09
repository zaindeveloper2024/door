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

    let server = Server::new("127.0.0.1:8080");
    if let Err(e) = server.run() {
        eprintln!("Error: {}", e);
    }
}
