use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{get, post, web};
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
    path = "/users/{id}",
    params(
        ("id" = i32, description = "Unique identifier of the User")
    ),
    responses((status = OK, body = User)),
    tag = "Demo"
)]
#[get("/user/{id}")]
async fn get_user(path: web::Path<(i32,)>) -> Json<User> {
    Json(User {
        id: path.into_inner().0,
        name: "Tom".to_string(),
    })
}
/// 新增一个用户
///
/// Post案例
#[utoipa::path(
    post,
    path = "/user",
    request_body = User,
    responses(
        (status = 200, description = "User created successfully", body = User)
    ),
    tag = "Demo"
)]
#[post("/user")]
async fn add_user(user: Json<User>) -> Json<User> {
    Json(User {
        id: user.id + 114,
        name: user.name.clone(),
    })
}
pub(crate) fn demo_apis(c: &mut ServiceConfig) {
    c.service(get_user);
    c.service(add_user);
}
