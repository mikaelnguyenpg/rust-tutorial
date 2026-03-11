mod db;
mod handlers;
mod models;
mod services;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::user::{create_user, delete_user, edit_user, get_all_users, get_user_detail};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user::create_user,
        crate::handlers::user::edit_user,
        crate::handlers::user::delete_user,
        crate::handlers::user::get_user_detail,
        crate::handlers::user::get_all_users
    ),
    tags(
        (name = "Users", description = "User management endpoints")
    ),
    info(
        title = "Demo API",
        version = "0.1.0",
        description = "REST API for ser management",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    ),
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        // user routers
        .route("/api/user", post(create_user))
        .route("/api/users/{id}", put(edit_user))
        .route("/api/users/{id}", get(get_user_detail))
        .route("/api/users/{id}", delete(delete_user))
        .route("/api/users", get(get_all_users))
        // product routers
        //
        // swagger - openapi
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("/api/user started at http://localhost:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Received Ctrl+C signal, starting graceful shutdown...");
        },
        _ = terminate => {
            println!("Received SIGTERM signal, starting graceful shutdown...");
        },
    }
}
