#[derive(Clone)]
pub struct RateLimitSettings {
    pub general_requests_per_second: u32,
    pub general_burst: u32,
    pub login_requests_per_minute: u32,
    pub password_reset_requests_per_minute: u32,
    pub session_refresh_requests_per_minute: u32,
    pub upload_requests_per_minute: u32,
    pub invitation_accept_requests_per_minute: u32,
    pub invitation_create_requests_per_hour: u32,
    pub invitation_create_burst: u32,
    pub max_tracked_clients_per_policy: usize,
    pub gc_interval_seconds: u64,
}

impl Default for RateLimitSettings {
    fn default() -> Self {
        Self {
            general_requests_per_second: 2,
            general_burst: 30,
            login_requests_per_minute: 5,
            password_reset_requests_per_minute: 3,
            session_refresh_requests_per_minute: 20,
            upload_requests_per_minute: 10,
            invitation_accept_requests_per_minute: 5,
            invitation_create_requests_per_hour: 10,
            invitation_create_burst: 3,
            max_tracked_clients_per_policy: 10_000,
            gc_interval_seconds: 30,
        }
    }
}
