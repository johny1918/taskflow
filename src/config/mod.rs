use dotenv;

pub struct Config {
    port: String,
    ip: String,
}

impl Config {
    pub fn new(ip: String, port: String) -> Self {
        Self {
            port,
            ip
        }
    }
   pub fn get_port(&self) -> String {
        self.port.clone()
    }

    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }

}
pub fn read_config() -> Config {
    dotenv::dotenv().ok();
    let port = dotenv::var("PORT").unwrap();
    let ip = dotenv::var("IP").unwrap();
    Config::new(ip, port)
}
