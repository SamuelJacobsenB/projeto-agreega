use axum::{Router, middleware};

use crate::{
    app::AppState,
    domains::{
        auth::routes::{
            authenticated_routes as auth_authenticated_routes, public_routes as auth_public_routes,
        },
        clients::routes::{
            authenticated_routes as clients_authenticated_routes,
            staff_routes as clients_staff_routes,
        },
        invitations::routes::{
            public_routes as invitations_public_routes, staff_routes as invitations_staff_routes,
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
    middlewares::auth::{require_admin, require_auth, require_staff},
};

pub fn create_router(state: AppState) -> Router {
    let public = Router::new()
        .nest("/auth", auth_public_routes())
        .nest("/invitations", invitations_public_routes())
        .nest("/project-images", project_images_public_routes());

    let authenticated_routes = Router::new()
        .nest("/auth", auth_authenticated_routes())
        .nest("/clients", clients_authenticated_routes())
        .nest("/projects", projects_authenticated_routes())
        .nest("/users", users_authenticated_routes())
        .nest("/stages", stages_authenticated_routes())
        .nest("/tasks", tasks_authenticated_routes())
        .nest("/photos", photos_authenticated_routes())
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let staff = Router::new()
        .nest("/clients", clients_staff_routes())
        .nest("/invitations", invitations_staff_routes())
        .nest("/project-images", project_images_staff_routes())
        .nest("/projects", projects_staff_routes())
        .nest("/users", users_staff_routes())
        .nest("/stages", stages_staff_routes())
        .nest("/tasks", tasks_staff_routes())
        .nest("/photos", photos_staff_routes())
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
