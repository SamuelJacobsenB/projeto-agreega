use axum::{Router, middleware};

use crate::{
    app::AppState,
    domains::auth::routes::{
        authenticated_routes as auth_authenticated_routes, public_routes as auth_public_routes,
    },
    domains::clients::routes::{
        authenticated_routes as clients_authenticated_routes, staff_routes as clients_staff_routes,
    },
    domains::invitations::routes::{
        public_routes as invitations_public_routes, staff_routes as invitations_staff_routes,
    },
    middlewares::auth::{require_admin, require_auth, require_staff},
};

pub fn create_router(state: AppState) -> Router {
    let public = Router::new()
        .nest("/auth", auth_public_routes())
        .nest("/invitations", invitations_public_routes());

    let authenticated_routes = Router::new()
        .nest("/auth", auth_authenticated_routes())
        .nest("/clients", clients_authenticated_routes())
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let staff = Router::new()
        .nest("/clients", clients_staff_routes())
        .nest("/invitations", invitations_staff_routes())
        .layer(middleware::from_fn(require_staff))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let admin = Router::new()
        .layer(middleware::from_fn(require_admin))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .nest(
            "/api/v1",
            public.merge(authenticated_routes).merge(staff).merge(admin),
        )
        .with_state(state)
}
