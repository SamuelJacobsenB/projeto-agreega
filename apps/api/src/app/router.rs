use axum::{Router, middleware};

use crate::{
    app::AppState,
    domains::{
        audit::routes::admin_routes as audit_admin_routes,
        auth::routes::{
            authenticated_routes as auth_authenticated_routes, public_routes as auth_public_routes,
        },
        clients::routes::{
            authenticated_routes as clients_authenticated_routes,
            staff_routes as clients_staff_routes,
        },
        document_approvals::routes::{
            authenticated_routes as document_approvals_authenticated_routes,
            staff_routes as document_approvals_staff_routes,
        },
        document_comments::routes::authenticated_routes as document_comments_authenticated_routes,
        document_versions::routes::{
            authenticated_routes as document_versions_authenticated_routes,
            staff_routes as document_versions_staff_routes,
        },
        documents::routes::{
            authenticated_routes as documents_authenticated_routes,
            staff_routes as documents_staff_routes,
        },
        invitations::routes::{
            public_routes as invitations_public_routes, staff_routes as invitations_staff_routes,
        },
        message_attachments::routes::authenticated_routes as message_attachments_authenticated_routes,
        messages::routes::authenticated_routes as messages_authenticated_routes,
        notifications::routes::authenticated_routes as notifications_authenticated_routes,
        payments::routes::{
            authenticated_routes as payments_authenticated_routes,
            staff_routes as payments_staff_routes,
        },
        photos::routes::{
            authenticated_routes as photos_authenticated_routes,
            staff_routes as photos_staff_routes,
        },
        project_images::routes::{
            public_routes as project_images_public_routes,
            staff_routes as project_images_staff_routes,
        },
        projects::routes::{
            authenticated_routes as projects_authenticated_routes,
            staff_routes as projects_staff_routes,
        },
        stages::routes::{
            authenticated_routes as stages_authenticated_routes,
            staff_routes as stages_staff_routes,
        },
        tasks::routes::{
            authenticated_routes as tasks_authenticated_routes, staff_routes as tasks_staff_routes,
        },
        users::routes::{
            authenticated_routes as users_authenticated_routes, staff_routes as users_staff_routes,
        },
    },
    middlewares::{
        auth::{require_admin, require_auth, require_staff},
        cors::cors_layer,
        logging::log_security_events,
        rate_limit,
    },
};

pub fn create_router(state: AppState) -> Router {
    let rate_limits = rate_limit::RateLimitPolicies::new(&state.config.trusted_proxy_cidrs);

    let public = Router::new()
        .nest("/auth", auth_public_routes(&rate_limits))
        .nest("/invitations", invitations_public_routes(&rate_limits))
        .nest("/project-images", project_images_public_routes());

    let authenticated_routes = Router::new()
        .nest("/auth", auth_authenticated_routes())
        .nest("/clients", clients_authenticated_routes())
        .nest("/projects", projects_authenticated_routes())
        .nest("/users", users_authenticated_routes())
        .nest("/stages", stages_authenticated_routes())
        .nest("/tasks", tasks_authenticated_routes())
        .nest("/messages", messages_authenticated_routes())
        .nest("/notifications", notifications_authenticated_routes())
        .nest("/payments", payments_authenticated_routes())
        .nest(
            "/message-attachments",
            message_attachments_authenticated_routes(&rate_limits),
        )
        .nest("/documents", documents_authenticated_routes())
        .nest(
            "/document-approvals",
            document_approvals_authenticated_routes(),
        )
        .nest(
            "/document-versions",
            document_versions_authenticated_routes()
                .merge(document_comments_authenticated_routes()),
        )
        .nest("/photos", photos_authenticated_routes())
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let staff = Router::new()
        .nest("/clients", clients_staff_routes())
        .nest("/project-images", project_images_staff_routes(&rate_limits))
        .nest("/projects", projects_staff_routes())
        .nest("/users", users_staff_routes())
        .nest("/stages", stages_staff_routes())
        .nest("/tasks", tasks_staff_routes())
        .nest("/documents", documents_staff_routes())
        .nest("/payments", payments_staff_routes())
        .nest("/document-approvals", document_approvals_staff_routes())
        .nest(
            "/document-versions",
            document_versions_staff_routes(&rate_limits),
        )
        .nest("/photos", photos_staff_routes(&rate_limits))
        .nest("/invitations", invitations_staff_routes(&rate_limits))
        .layer(middleware::from_fn(require_staff))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let admin = Router::new()
        .nest("/audit-logs", audit_admin_routes())
        .layer(middleware::from_fn(require_admin))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let cors = cors_layer([
        state.config.website_url.as_str(),
        state.config.app_url.as_str(),
    ])
    .expect("Failed to configure CORS.");

    Router::new()
        .nest(
            "/api/v1",
            public
                .merge(authenticated_routes)
                .merge(staff)
                .merge(admin)
                .layer(rate_limits.general.clone()),
        )
        .layer(middleware::from_fn(log_security_events))
        .layer(cors)
        .with_state(state)
}
