pub mod server;

pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn init_config() -> Config {
    Config {
        host: String::from("localhost"),
        port: 8080,
    }
}
