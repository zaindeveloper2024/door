pub mod server;

pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn init_config() -> Config {
    Config {
        host: "localhost".to_string(),
        port: 8080,
    }
}
