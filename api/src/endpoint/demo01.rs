use crate::rsp::{ApiResponse, ApiResult, ok_rsp};
use actix_web::web::Json;
use actix_web::{get, post, web};
use redis::ToRedisArgs;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;
use utoipa_actix_web::service_config::ServiceConfig;

pub(super) fn register_api(c: &mut ServiceConfig) {
    c.service(get_student);
    c.service(add_student);
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct Student {
    #[validate(range(min = 0))]
    id: i32,
    /// 0-100 超过就会报错
    #[validate(range(min = 0, max = 100))]
    age: i32,
}
/// 根据ID查找用户
///
/// GET请求案例
#[utoipa::path(
    get,
    path = "/demo01/student/{id}",
    params(
        ("id" = i32, description = "Unique identifier of the Student ")
    ),
    responses((status = OK, body = ApiResponse<Student>)),
    tag = "Demo01"
)]
#[get("/demo01/student/{id}")]
async fn get_student(path: web::Path<(i32,)>) -> ApiResult<Student> {
    ok_rsp(Student {
        id: path.0 + 114514,
        age: path.0,
    })
}
/// 新增数据
///
/// 字段验证案例
#[utoipa::path(
    post,
    path = "/demo01/student",
    request_body = Student,
    responses(
        (status = 200, description = "Student created successfully", body = ApiResponse<Student>)
    ),
    tag = "Demo01"
)]
#[post("/demo01/student")]
async fn add_student(student: Json<Student>) -> ApiResult<Student> {
    student.validate()?;
    ok_rsp(student.into_inner())
}
