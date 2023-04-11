#[derive(Debug)]
pub struct Database {
    pub url: String,
}

#[derive(Debug)]
pub struct Config {
    pub database: Database,
}

pub fn load_config() -> Config {
    println!("Loading config...");
    dotenv::dotenv().expect("Failed to read .env file");

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Config {
        database
    }
}