use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, ResponseError};
use common::Rsp;
use serde::Serialize;
use thiserror::Error;
use utoipa_actix_web::service_config::ServiceConfig;

#[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
struct User {
    /// ID
    id: i32,
    /// username
    name: String,
}

/// 根据ID查找用户
///
/// GET请求案例
#[utoipa::path(
    get,
    path = "/user/{id}",
    params(
        ("id" = i32, description = "Unique identifier of the User")
    ),
    responses((status = OK, body = Rsp<User>)),
    tag = "Demo"
)]
#[get("/user/{id}")]
async fn get_user(path: web::Path<(i32,)>) -> Json<Rsp<User>> {
    let user = User {
        id: path.into_inner().0,
        name: "Tom".to_string(),
    };
    Json(Rsp::ok(user))
}
/// 新增一个用户
///
/// Post案例
#[utoipa::path(
    post,
    path = "/user",
    request_body = User,
    responses(
        (status = 200, description = "User created successfully", body = Rsp<User>)
    ),
    tag = "Demo"
)]
#[post("/user")]
async fn add_user(user: Json<User>) -> Json<Rsp<User>> {
    let data = Rsp::ok(User {
        id: user.id + 114,
        name: user.name.clone(),
    });
    Json(data)
}
pub(crate) fn demo_apis(c: &mut ServiceConfig) {
    c.service(get_user);
    c.service(add_user);
}
#[derive(Debug, Serialize)]
struct SuccessResponse<T> {
    data: T,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Error)]
enum MyError {
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };
        match *self {
            MyError::InternalError => HttpResponse::Ok().json(error_response),
            MyError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
        }
    }
}
