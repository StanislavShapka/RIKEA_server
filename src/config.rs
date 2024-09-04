#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String
}

impl Config {
    pub(crate) fn new() -> Config {
        Config {
            database_url: std::env::var("DATABASE_URL").unwrap()
        }
    }
}