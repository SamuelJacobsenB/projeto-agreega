use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
};

static LAST_SECURITY_LOG_MS: AtomicU64 = AtomicU64::new(0);

pub async fn log_security_events(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path();
    let route = sensitive_route(path);
    let critical_mutation = route.is_some_and(|_| is_critical_mutation(&method, path));

    let response = next.run(request).await;
    let status = response.status();

    if critical_mutation && status.is_success() {
        tracing::info!(
            route = route.unwrap_or("unknown"),
            method = %method,
            status = status.as_u16(),
            event = "critical_mutation",
            "Critical API operation completed"
        );
    } else if is_suspicious_status(status) && route.is_some() && allow_security_log() {
        if status.is_server_error() {
            tracing::error!(
                route = route.unwrap_or("unknown"),
                method = %method,
                status = status.as_u16(),
                event = "sensitive_route_failure",
                "Sensitive API operation failed"
            );
        } else {
            tracing::warn!(
                route = route.unwrap_or("unknown"),
                method = %method,
                status = status.as_u16(),
                event = "suspicious_response",
                "Suspicious response on sensitive API route"
            );
        }
    }

    response
}

fn sensitive_route(path: &str) -> Option<&'static str> {
    let path = path.strip_prefix("/api/v1/")?;
    let (route, _) = path.split_once('/').unwrap_or((path, ""));

    match route {
        "auth" => Some("auth"),
        "clients" => Some("clients"),
        "document-approvals" => Some("document_approvals"),
        "documents" => Some("documents"),
        "document-versions" => Some("document_versions"),
        "invitations" => Some("invitations"),
        "message-attachments" => Some("message_attachments"),
        "payments" => Some("payments"),
        "photos" => Some("photos"),
        "projects" => Some("projects"),
        "users" => Some("users"),
        _ => None,
    }
}

fn is_critical_mutation(method: &Method, path: &str) -> bool {
    let is_mutation = matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    );

    if !is_mutation {
        return false;
    }

    let Some(route) = sensitive_route(path) else {
        return false;
    };

    match route {
        "auth" => path.ends_with("/logout") || path.contains("/password-reset"),
        "clients"
        | "document_approvals"
        | "documents"
        | "invitations"
        | "message_attachments"
        | "payments"
        | "photos"
        | "projects"
        | "users" => true,
        _ => false,
    }
}

fn is_suspicious_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::NOT_FOUND
            | StatusCode::UNAUTHORIZED
            | StatusCode::FORBIDDEN
            | StatusCode::TOO_MANY_REQUESTS
    ) || status.is_server_error()
}

fn allow_security_log() -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let mut previous = LAST_SECURITY_LOG_MS.load(Ordering::Relaxed);
    loop {
        if now.saturating_sub(previous) < 1_000 {
            return false;
        }

        match LAST_SECURITY_LOG_MS.compare_exchange_weak(
            previous,
            now,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return true,
            Err(current) => previous = current,
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{http::Method, http::StatusCode};

    use super::{is_critical_mutation, is_suspicious_status, sensitive_route};

    #[test]
    fn only_selected_sensitive_routes_are_classified() {
        assert_eq!(sensitive_route("/api/v1/payments/123"), Some("payments"));
        assert_eq!(sensitive_route("/api/v1/messages/123"), None);
    }

    #[test]
    fn critical_mutations_are_selected_without_logging_reads() {
        assert!(is_critical_mutation(&Method::PATCH, "/api/v1/payments/123"));
        assert!(!is_critical_mutation(
            &Method::GET,
            "/api/v1/payments/project/123"
        ));
        assert!(!is_critical_mutation(&Method::POST, "/api/v1/auth/login"));
        assert!(is_critical_mutation(
            &Method::POST,
            "/api/v1/auth/password-reset"
        ));
    }

    #[test]
    fn only_security_relevant_statuses_are_suspicious() {
        assert!(is_suspicious_status(StatusCode::FORBIDDEN));
        assert!(is_suspicious_status(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(!is_suspicious_status(StatusCode::BAD_REQUEST));
        assert!(!is_suspicious_status(StatusCode::OK));
    }
}
