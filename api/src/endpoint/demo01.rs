use crate::rsp::{ApiError, ApiResponse};
use actix_web::web::Json;
use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use validator_derive::Validate;

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct Student {
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
async fn get_student(path: web::Path<(i32,)>) -> Result<ApiResponse<Student>, ApiError> {
    Ok(ApiResponse::success(Student {
        id: path.0 + 114514,
        age: path.0,
    }))
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
async fn add_student(student: Json<Student>) -> Result<ApiResponse<Student>, ApiError> {
    student.validate()?;
    Ok(ApiResponse::success(Student {
        id: student.id + 114514,
        age: student.age,
    }))
}
