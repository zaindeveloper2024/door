pub struct Server {}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: String,
}

impl Server {
    #[inline]
    pub fn http(addr: String) {}

    pub fn new() {
        Self::listen()
    }

    pub fn listen() {}
}
