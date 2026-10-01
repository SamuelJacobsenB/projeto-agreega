use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub cookie_secure: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let jwt_secret = env::var("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            return Err("JWT_SECRET deve ter pelo menos 32 bytes.".into());
        }

        Ok(Self {
            port: env::var("PORT")?.parse()?,
            database_url: env::var("DATABASE_URL")?,
            jwt_secret,
            cookie_secure: env::var("COOKIE_SECURE")
                .map(|value| value.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        })
    }
}
