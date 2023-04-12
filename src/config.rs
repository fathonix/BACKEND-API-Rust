#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String
}

pub fn load_config() -> Config {
    println!("Loading config...");
    dotenv::dotenv().expect("Failed to read .env file");

    let database = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let host = std::env::var("HOST")
        .unwrap_or("127.0.0.1:8000".to_string());

    Config {
        database_url: database,
        host
    }
}