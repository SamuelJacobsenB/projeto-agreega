use std::{net::IpAddr, num::NonZeroU32, time::Duration};

use axum::{
    Json,
    http::{HeaderValue, StatusCode, header::CACHE_CONTROL},
    response::{IntoResponse, Response},
    routing::MethodRouter,
};
use axum_governor::{GovernorConfigBuilder, GovernorLayer, Quota, RejectionReason, SmartIp};
use ipnet::IpNet;

use crate::{constants::rate_limit::RateLimitSettings, response::ApiResponse};

pub type RateLimitLayer = GovernorLayer<IpAddr>;

#[derive(Clone)]
pub struct RateLimitPolicies {
    pub general: RateLimitLayer,
    pub login: RateLimitLayer,
    pub password_reset_request: RateLimitLayer,
    pub password_reset_complete: RateLimitLayer,
    pub session_refresh: RateLimitLayer,
    pub upload: RateLimitLayer,
    pub invitation_accept: RateLimitLayer,
    pub invitation_create: RateLimitLayer,
}

impl RateLimitPolicies {
    pub fn new(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> Self {
        Self {
            general: general(settings, trusted_proxies),
            login: login(settings, trusted_proxies),
            password_reset_request: password_reset(settings, trusted_proxies),
            password_reset_complete: password_reset(settings, trusted_proxies),
            session_refresh: session_refresh(settings, trusted_proxies),
            upload: upload(settings, trusted_proxies),
            invitation_accept: invitation_accept(settings, trusted_proxies),
            invitation_create: invitation_create(settings, trusted_proxies),
        }
    }
}

pub fn protect_route<S>(route: MethodRouter<S>, layer: RateLimitLayer) -> MethodRouter<S>
where
    S: Clone + 'static,
{
    route.layer::<_, std::convert::Infallible>(layer)
}

fn with_rate_limit(
    quota: Quota,
    settings: &RateLimitSettings,
    trusted_proxies: &[IpNet],
) -> RateLimitLayer {
    let config = GovernorConfigBuilder::default()
        .with_extractor(SmartIp::new().with_trusted_proxies(trusted_proxies.iter().copied()))
        .expect_connect_info()
        .quota_default(quota)
        .gc_interval(Duration::from_secs(settings.gc_interval_seconds))
        .max_keys(settings.max_tracked_clients_per_policy)
        .redact_keys(true)
        .error_handler(rate_limit_error_response)
        .finish()
        .expect("Invalid rate limit configuration");

    GovernorLayer::new(config)
}

fn rate_limit_error_response(reason: RejectionReason) -> Response {
    let (status, message) = match reason {
        RejectionReason::QuotaExceeded { .. } => (
            StatusCode::TOO_MANY_REQUESTS,
            "Muitas solicitações. Tente novamente mais tarde.",
        ),
        RejectionReason::KeyExtractionFailed(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Não foi possível validar a origem da solicitação.",
        ),
    };

    let mut response = (status, Json(ApiResponse::<()>::error(message))).into_response();
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));

    response
}

fn general(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_second(
            NonZeroU32::new(settings.general_requests_per_second)
                .expect("General rate must be greater than zero"),
        )
        .burst(
            NonZeroU32::new(settings.general_burst)
                .expect("General burst must be greater than zero"),
        ),
        settings,
        trusted_proxies,
    )
}

fn login(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(settings.login_requests_per_minute)
                .expect("Login rate must be greater than zero"),
        ),
        settings,
        trusted_proxies,
    )
}

fn password_reset(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(settings.password_reset_requests_per_minute)
                .expect("Password reset rate must be greater than zero"),
        ),
        settings,
        trusted_proxies,
    )
}

fn session_refresh(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(settings.session_refresh_requests_per_minute)
                .expect("Session refresh rate must be greater than zero"),
        ),
        settings,
        trusted_proxies,
    )
}

fn upload(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(settings.upload_requests_per_minute)
                .expect("Upload rate must be greater than zero"),
        ),
        settings,
        trusted_proxies,
    )
}

fn invitation_accept(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(settings.invitation_accept_requests_per_minute)
                .expect("Invitation acceptance rate must be greater than zero"),
        ),
        settings,
        trusted_proxies,
    )
}

fn invitation_create(settings: &RateLimitSettings, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_hour(
            NonZeroU32::new(settings.invitation_create_requests_per_hour)
                .expect("Invitation creation rate must be greater than zero"),
        )
        .burst(
            NonZeroU32::new(settings.invitation_create_burst)
                .expect("Invitation creation burst must be greater than zero"),
        ),
        settings,
        trusted_proxies,
    )
}

#[cfg(test)]
mod tests {
    use super::RateLimitPolicies;
    use crate::constants::rate_limit::RateLimitSettings;

    #[test]
    fn constructs_all_limiters_from_runtime_settings() {
        let settings = RateLimitSettings {
            login_requests_per_minute: 8,
            max_tracked_clients_per_policy: 2_000,
            ..RateLimitSettings::default()
        };
        let trusted_proxies = ["10.0.0.0/8".parse().unwrap()];

        let _policies = RateLimitPolicies::new(&settings, &trusted_proxies);
    }
}
