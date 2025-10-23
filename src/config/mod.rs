use dotenv;

pub struct Config {
    port: String,
    ip: String,
    database_url: String,
}

impl Config {
    pub fn new(ip: String, port: String, database_url: String) -> Self {
        Self {
            port,
            ip,
            database_url,
        }
    }
    pub fn get_port(&self) -> String {
        self.port.clone()
    }

    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }

    pub fn get_database_url(&self) -> String {
        self.database_url.clone()
    }
}
pub fn read_config() -> Config {
    dotenv::dotenv().ok();
    let port = dotenv::var("PORT").unwrap();
    let ip = dotenv::var("IP").unwrap();
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    Config::new(ip, port, database_url)
}
