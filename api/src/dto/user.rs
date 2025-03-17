use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator_derive::Validate;

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct LoginReq {
    /// 用户名
    #[validate(length(min = 1, max = 20))]
    pub username: String,

    /// 密码
    #[validate(length(min = 1, max = 20))]
    pub password: String,
}
#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct LoginToken {
    pub token: String,
}

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
#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct PageReq {
    /// 用户名
    pub username: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 页码
    #[validate(range(min = 1))]
    #[schema(example = 1)]
    pub page: u64,
    /// 每页大小
    #[validate(range(min = 1))]
    #[schema(example = 10)]
    pub size: u64,
}
/// User Update Request Dto
#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct UpdatePwd {
    #[validate(range(min = 1))]
    pub id: i32,

    #[validate(length(min = 1, max = 20))]
    pub password: String,
}
