use std::env;

use ipnet::IpNet;

use crate::constants::rate_limit::RateLimitSettings;

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

    pub rate_limits: RateLimitSettings,
    pub max_upload_size_bytes: i64,
    pub max_upload_body_size_bytes: usize,
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

        let defaults = RateLimitSettings::default();
        let rate_limits = RateLimitSettings {
            general_requests_per_second: positive_env_or(
                "RATE_LIMIT_GENERAL_PER_SECOND",
                defaults.general_requests_per_second,
            )?,
            general_burst: positive_env_or("RATE_LIMIT_GENERAL_BURST", defaults.general_burst)?,
            login_requests_per_minute: positive_env_or(
                "RATE_LIMIT_LOGIN_PER_MINUTE",
                defaults.login_requests_per_minute,
            )?,
            password_reset_requests_per_minute: positive_env_or(
                "RATE_LIMIT_PASSWORD_RESET_PER_MINUTE",
                defaults.password_reset_requests_per_minute,
            )?,
            session_refresh_requests_per_minute: positive_env_or(
                "RATE_LIMIT_SESSION_REFRESH_PER_MINUTE",
                defaults.session_refresh_requests_per_minute,
            )?,
            upload_requests_per_minute: positive_env_or(
                "RATE_LIMIT_UPLOAD_PER_MINUTE",
                defaults.upload_requests_per_minute,
            )?,
            invitation_accept_requests_per_minute: positive_env_or(
                "RATE_LIMIT_INVITATION_ACCEPT_PER_MINUTE",
                defaults.invitation_accept_requests_per_minute,
            )?,
            invitation_create_requests_per_hour: positive_env_or(
                "RATE_LIMIT_INVITATION_CREATE_PER_HOUR",
                defaults.invitation_create_requests_per_hour,
            )?,
            invitation_create_burst: positive_env_or(
                "RATE_LIMIT_INVITATION_CREATE_BURST",
                defaults.invitation_create_burst,
            )?,
            max_tracked_clients_per_policy: positive_env_or(
                "RATE_LIMIT_MAX_TRACKED_CLIENTS",
                defaults.max_tracked_clients_per_policy,
            )?,
            gc_interval_seconds: positive_env_or(
                "RATE_LIMIT_GC_INTERVAL_SECONDS",
                defaults.gc_interval_seconds,
            )?,
        };
        let max_upload_size_bytes = positive_env_or("MAX_UPLOAD_SIZE_BYTES", 20 * 1024 * 1024_i64)?;
        let upload_body_overhead_bytes =
            positive_env_or("UPLOAD_BODY_OVERHEAD_BYTES", 64 * 1024_usize)?;
        let max_upload_body_size_bytes = usize::try_from(max_upload_size_bytes)?
            .checked_add(upload_body_overhead_bytes)
            .ok_or("MAX_UPLOAD_SIZE_BYTES plus UPLOAD_BODY_OVERHEAD_BYTES overflows")?;
        let cookie_secure = bool_env_or("COOKIE_SECURE", false)?;

        Ok(Self {
            port: env::var("PORT")?.parse()?,
            website_url: env::var("WEBSITE_URL")?,
            app_url: env::var("APP_URL")?,
            database_url: env::var("DATABASE_URL")?,
            jwt_secret,
            cookie_secure,
            trusted_proxy_cidrs,
            email_api_key: env::var("EMAIL_API_KEY")?,
            email_from: env::var("EMAIL_FROM")?,
            rate_limits,
            max_upload_size_bytes,
            max_upload_body_size_bytes,
        })
    }
}

fn bool_env_or(name: &str, default: bool) -> Result<bool, Box<dyn std::error::Error>> {
    match env::var(name) {
        Ok(value) => parse_boolean(name, &value),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(error) => Err(error.into()),
    }
}

fn parse_boolean(name: &str, value: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if value.eq_ignore_ascii_case("true") {
        Ok(true)
    } else if value.eq_ignore_ascii_case("false") {
        Ok(false)
    } else {
        Err(format!("{name} deve ser 'true' ou 'false'.").into())
    }
}

fn positive_env_or<T>(name: &str, default: T) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr + PartialOrd + From<u8> + Copy + std::fmt::Display,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let value = match env::var(name) {
        Ok(value) => parse_positive_value(name, &value)?,
        Err(env::VarError::NotPresent) => default,
        Err(error) => return Err(error.into()),
    };

    Ok(value)
}

fn parse_positive_value<T>(name: &str, value: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr + PartialOrd + From<u8> + std::fmt::Display,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let parsed = value.parse::<T>()?;
    if parsed < T::from(1) {
        return Err(format!("{name} deve ser maior que zero.").into());
    }
    Ok(parsed)
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

#[cfg(test)]
mod tests {
    use super::{parse_boolean, parse_positive_value, parse_trusted_proxy_cidrs};

    #[test]
    fn parses_positive_environment_settings() {
        assert_eq!(parse_positive_value::<u32>("LIMIT", "5").unwrap(), 5);
        assert!(parse_positive_value::<u32>("LIMIT", "0").is_err());
        assert!(parse_positive_value::<u32>("LIMIT", "invalid").is_err());
    }

    #[test]
    fn parses_cookie_security_as_an_explicit_boolean() {
        assert!(parse_boolean("COOKIE_SECURE", "true").unwrap());
        assert!(!parse_boolean("COOKIE_SECURE", "FALSE").unwrap());
        assert!(parse_boolean("COOKIE_SECURE", "yes").is_err());
    }

    #[test]
    fn parses_trusted_proxy_cidrs_and_rejects_invalid_values() {
        assert_eq!(
            parse_trusted_proxy_cidrs("10.0.0.0/8,2001:db8::/32")
                .unwrap()
                .len(),
            2
        );
        assert!(parse_trusted_proxy_cidrs("not-a-cidr").is_err());
    }
}
