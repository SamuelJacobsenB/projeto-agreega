use std::env;

use ipnet::IpNet;

#[derive(Clone)]
pub struct Config {
    pub port: u16,

    pub website_url: String,
    pub app_url: String,

    pub database_url: String,

    pub jwt_secret: String,

    pub cookie_secure: bool,

    pub trusted_proxy_cidrs: Vec<IpNet>,

    pub email_api_key: String,
    pub email_from: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let jwt_secret = env::var("JWT_SECRET")?;
        if jwt_secret.len() < 32 {
            return Err("JWT_SECRET deve ter pelo menos 32 bytes.".into());
        }

        let trusted_proxy_cidrs = match env::var("TRUSTED_PROXY_CIDRS") {
            Ok(value) => parse_trusted_proxy_cidrs(&value)?,
            Err(env::VarError::NotPresent) => Vec::new(),
            Err(error) => return Err(error.into()),
        };

        Ok(Self {
            port: env::var("PORT")?.parse()?,
            website_url: env::var("WEBSITE_URL")?,
            app_url: env::var("APP_URL")?,
            database_url: env::var("DATABASE_URL")?,
            jwt_secret,
            cookie_secure: env::var("COOKIE_SECURE")
                .map(|value| value.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            trusted_proxy_cidrs,
            email_api_key: env::var("EMAIL_API_KEY")?,
            email_from: env::var("EMAIL_FROM")?,
        })
    }
}

fn parse_trusted_proxy_cidrs(value: &str) -> Result<Vec<IpNet>, std::io::Error> {
    value
        .split(',')
        .map(str::trim)
        .filter(|cidr| !cidr.is_empty())
        .map(|cidr| {
            cidr.parse::<IpNet>().map_err(|error| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("TRUSTED_PROXY_CIDRS contém um CIDR inválido '{cidr}': {error}"),
                )
            })
        })
        .collect()
}
