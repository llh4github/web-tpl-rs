use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator_derive::Validate;

use super::PageParam;

/// User Login Request Dto
#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct LoginReq {
    /// 用户名
    #[validate(length(min = 1, max = 20))]
    pub username: String,

    /// 密码
    #[validate(length(min = 1, max = 20))]
    pub password: String,
}
/// User Login Response Dto
#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct LoginToken {
    /// 登录成功后的token
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

    #[serde(flatten)]
    pub param: PageParam,
}
/// User Update Request Dto
#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct UpdatePwd {
    #[validate(range(min = 1))]
    pub id: i32,

    #[validate(length(min = 1, max = 20))]
    pub password: String,
}
