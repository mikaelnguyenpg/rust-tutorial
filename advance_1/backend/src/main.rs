mod user_store;

use crate::user_store::User;
use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, Debug, Serialize, ToSchema)]
struct Response<T> {
    message: String,
    value: Option<T>,
}

#[utoipa::path(
    post,
    path = "/api/user",
    request_body = User,
    responses(
        (status = 200, description = "Created", body = Response<u32>),
    ),
    tag = "Users"
)]
async fn create_user(Json(user): Json<User>) -> Json<Response<u32>> {
    let ret = user_store::create_user(&user);
    let respone = match ret {
        Ok(()) => Response {
            message: String::from("Created"),
            value: Some(user.id),
        },
        Err(msg) => Response {
            message: msg,
            value: None,
        },
    };
    Json(respone)
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    request_body = User,
    responses(
        (status = 200, description = "Updated"),
    ),
    tag = "Users"
)]
async fn edit_user(Path(id): Path<u32>, Json(user): Json<User>) -> Json<Response<()>> {
    let ret = user_store::update_user(id, Some(user.name), Some(user.email));
    let respone = match ret {
        Ok(()) => Response {
            message: String::from("Updated"),
            value: None,
        },
        Err(msg) => Response {
            message: msg,
            value: None,
        },
    };
    Json(respone)
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User detail", body = Response<User>),
    ),
    tag = "Users"
)]
async fn get_user_detail(Path(id): Path<u32>) -> Json<Response<User>> {
    let user = user_store::get_user(id);
    Json(Response {
        message: format!("Result for user_id: {}", id),
        value: user.cloned(),
    })
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Deleted"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users"
)]
async fn delete_user(Path(id): Path<u32>) -> Json<Response<()>> {
    let ret = user_store::delete_user(id);
    let respone = match ret {
        Ok(()) => Response {
            message: String::from("Deleted"),
            value: None,
        },
        Err(msg) => Response {
            message: msg,
            value: None,
        },
    };
    Json(respone)
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "Result", body = Response<Vec<User>>),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users"
)]
async fn get_all_users() -> Json<Response<Vec<User>>> {
    let ret = user_store::get_all_users();
    Json(Response {
        message: String::from("Result"),
        value: Some(ret),
    })
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        edit_user,
        delete_user,
        get_user_detail,
        get_all_users
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
        .route("/api/user", post(create_user))
        .route("/api/users/{id}", put(edit_user))
        .route("/api/users/{id}", get(get_user_detail))
        .route("/api/users/{id}", delete(delete_user))
        .route("/api/users", get(get_all_users))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("/api/user started at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
