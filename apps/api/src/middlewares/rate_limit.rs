use std::{net::IpAddr, num::NonZeroU32, time::Duration};

use axum::{
    Json,
    http::{HeaderValue, StatusCode, header::CACHE_CONTROL},
    response::{IntoResponse, Response},
    routing::MethodRouter,
};
use axum_governor::{GovernorConfigBuilder, GovernorLayer, Quota, RejectionReason, SmartIp};
use ipnet::IpNet;

use crate::{constants::rate_limit as limits, response::ApiResponse};

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
    pub fn new(trusted_proxies: &[IpNet]) -> Self {
        Self {
            general: general(trusted_proxies),
            login: login(trusted_proxies),
            password_reset_request: password_reset(trusted_proxies),
            password_reset_complete: password_reset(trusted_proxies),
            session_refresh: session_refresh(trusted_proxies),
            upload: upload(trusted_proxies),
            invitation_accept: invitation_accept(trusted_proxies),
            invitation_create: invitation_create(trusted_proxies),
        }
    }
}

pub fn protect_route<S>(route: MethodRouter<S>, layer: RateLimitLayer) -> MethodRouter<S>
where
    S: Clone + 'static,
{
    route.layer::<_, std::convert::Infallible>(layer)
}

fn with_rate_limit(quota: Quota, trusted_proxies: &[IpNet]) -> RateLimitLayer {
    let config = GovernorConfigBuilder::default()
        .with_extractor(SmartIp::new().with_trusted_proxies(trusted_proxies.iter().copied()))
        .expect_connect_info()
        .quota_default(quota)
        .gc_interval(Duration::from_secs(30))
        .max_keys(limits::MAX_TRACKED_CLIENTS_PER_POLICY)
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

pub fn general(trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_second(
            NonZeroU32::new(limits::GENERAL_REQUESTS_PER_SECOND)
                .expect("General rate must be greater than zero"),
        )
        .burst(
            NonZeroU32::new(limits::GENERAL_BURST)
                .expect("General burst must be greater than zero"),
        ),
        trusted_proxies,
    )
}

pub fn login(trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(limits::LOGIN_REQUESTS_PER_MINUTE)
                .expect("Login rate must be greater than zero"),
        ),
        trusted_proxies,
    )
}

pub fn password_reset(trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(limits::PASSWORD_RESET_REQUESTS_PER_MINUTE)
                .expect("Password reset rate must be greater than zero"),
        ),
        trusted_proxies,
    )
}

pub fn session_refresh(trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(limits::SESSION_REFRESH_REQUESTS_PER_MINUTE)
                .expect("Session refresh rate must be greater than zero"),
        ),
        trusted_proxies,
    )
}

pub fn upload(trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(limits::UPLOAD_REQUESTS_PER_MINUTE)
                .expect("Upload rate must be greater than zero"),
        ),
        trusted_proxies,
    )
}

pub fn invitation_accept(trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_minute(
            NonZeroU32::new(limits::INVITATION_ACCEPT_REQUESTS_PER_MINUTE)
                .expect("Invitation acceptance rate must be greater than zero"),
        ),
        trusted_proxies,
    )
}

pub fn invitation_create(trusted_proxies: &[IpNet]) -> RateLimitLayer {
    with_rate_limit(
        Quota::requests_per_hour(
            NonZeroU32::new(limits::INVITATION_CREATE_REQUESTS_PER_HOUR)
                .expect("Invitation creation rate must be greater than zero"),
        )
        .burst(
            NonZeroU32::new(limits::INVITATION_CREATE_BURST)
                .expect("Invitation creation burst must be greater than zero"),
        ),
        trusted_proxies,
    )
}
