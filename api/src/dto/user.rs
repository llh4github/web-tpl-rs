use sea_orm::prelude::DateTime;
use sea_orm::{DerivePartialModel, FromQueryResult};
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

/// 分页查询返回数据
#[derive(Deserialize, Serialize, Debug, ToSchema, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "db::entities::auth_user::Entity")]
pub struct PageEle {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    #[sea_orm(skip)]
    pub roles: Vec<RoleInfo>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "db::entities::auth_role::Entity")]
pub struct RoleInfo {
    pub id: i32,
    pub name: String,
    pub code: String,
}
