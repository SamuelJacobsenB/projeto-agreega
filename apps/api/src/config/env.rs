use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            port: env::var("PORT")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
