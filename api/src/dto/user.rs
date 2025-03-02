use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator_derive::Validate;

/// User Add Request Dto
#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct AddReq {
    #[validate(length(min = 1, max = 20))]
    pub username: String,
    #[validate(length(min = 1, max = 20))]
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}
/// User Page Request Dto
#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct PageReq {
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 页码
    pub page: i32,
    /// 每页大小
    pub size: i32,
}
