use axum::Json;
use axum::extract::Path;
use validator::Validate;

use crate::models::common::Response;
use crate::models::user::{RequestUser, User};
use crate::services::user_service;

#[utoipa::path(
    post,
    path = "/api/user",
    request_body = RequestUser,
    responses(
        (status = 200, description = "Created", body = Response<u32>),
    ),
    tag = "Users"
)]
pub async fn create_user(Json(user): Json<RequestUser>) -> Json<Response<usize>> {
    if let Err(e) = user.validate() {
        return Json(Response {
            message: e.to_string(),
            value: None,
        });
    }

    let ret = user_service::create_user(user);
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
pub async fn edit_user(Path(id): Path<usize>, Json(user): Json<RequestUser>) -> Json<Response<()>> {
    let ret = user_service::update_user(id, Some(user.name), Some(user.email));
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
        ("id" = usize, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User detail", body = Response<User>),
    ),
    tag = "Users"
)]
pub async fn get_user_detail(Path(id): Path<usize>) -> Json<Response<User>> {
    let user = user_service::get_user(id);
    Json(Response {
        message: format!("Result for user_id: {}", id),
        value: user.cloned(),
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
pub async fn delete_user(Path(id): Path<usize>) -> Json<Response<()>> {
    let ret = user_service::delete_user(id);
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
pub async fn get_all_users() -> Json<Response<Vec<User>>> {
    let ret = user_service::get_all_users();
    Json(Response {
        message: String::from("Result"),
        value: Some(ret),
    })
}
