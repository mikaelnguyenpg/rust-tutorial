use axum::Json;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use validator::Validate;

use crate::db::DbTransaction;
use crate::models::common::Response;
use crate::models::user::{RequestUser, User};
use crate::services::user_service::UserService;

#[utoipa::path(
    post,
    path = "/api/user",
    request_body = RequestUser,
    responses(
        (status = 200, description = "Created", body = Response<u32>),
    ),
    tag = "Users"
)]
pub async fn create_user(
    Extension(tx): Extension<DbTransaction>,
    Json(user): Json<RequestUser>,
) -> Json<Response<i32>> {
    if let Err(e) = user.validate() {
        return Json(Response {
            message: e.to_string(),
            value: None,
        });
    }
    let user_service = UserService::new(tx);
    let ret = user_service.create_user(user).await;
    let response = match ret {
        Ok(uid) => Response {
            message: String::from("Created"),
            value: Some(uid),
        },
        Err(msg) => Response {
            message: msg,
            value: None,
        },
    };
    Json(response)
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = usize, Path, description = "User ID")
    ),
    request_body = RequestUser,
    responses(
        (status = 200, description = "Updated"),
    ),
    tag = "Users"
)]
pub async fn edit_user(
    Path(id): Path<i32>,
    Extension(tx): Extension<DbTransaction>,
    Json(user): Json<RequestUser>,
) -> impl IntoResponse {
    let user_service = UserService::new(tx);
    let ret: Result<(), String> = user_service.update_user(id, user).await;
    // let respone = match ret {
    //     Ok(()) => Response {
    //         message: String::from("Updated"),
    //         value: None,
    //     },
    //     Err(msg) => Response {
    //         message: msg,
    //         value: None,
    //     },
    // };
    // Json(respone);
    // (StatusCode::OK, Json(json!({ "message": "Updated" })))
    (StatusCode::OK, Json(json!({ "error": "Testing" })))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = usize, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User detail", body = Response<User>),
    ),
    tag = "Users"
)]
pub async fn get_user_detail(
    Path(id): Path<i32>,
    Extension(tx): Extension<DbTransaction>,
) -> Json<Response<User>> {
    let user_service = UserService::new(tx);
    let user = user_service.get_user(id).await;
    Json(Response {
        message: format!("Result for user_id: {}", id),
        value: user,
    })
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = usize, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Deleted"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users"
)]
pub async fn delete_user(
    Path(id): Path<i32>,
    Extension(tx): Extension<DbTransaction>,
) -> Json<Response<()>> {
    let user_service = UserService::new(tx);
    let ret = user_service.delete_user(id).await;
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
pub async fn get_all_users(Extension(tx): Extension<DbTransaction>) -> Json<Response<Vec<User>>> {
    let user_service = UserService::new(tx);
    let ret = user_service.get_all_users().await;
    Json(Response {
        message: String::from("Result"),
        value: ret,
    })
}
