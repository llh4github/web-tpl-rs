use crate::global::AppResources;
use crate::rsp::ApiErrors::CommonError;
use crate::rsp::{ok_rsp, ApiResponse};
use crate::{dto, rsp};
use actix_web::{get, post, web};
use db::entities::auth_user;
use db::entities::prelude::AuthUser;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, TransactionTrait};
use validator::Validate;

/// 根据 ID 查找数据
#[utoipa::path(
    get,
    path = "/user/{id}",
    params(
        ("id" = i32, description = "Data ID")
    ),
    responses((status = OK, body = ApiResponse<Option<auth_user::Model>>)),
    tag = "用户管理模块"
)]
#[get("/user/{id}")]
pub async fn find_user(
    id: web::Path<i32>,
    data: web::Data<AppResources>,
) -> rsp::ApiResult<Option<auth_user::Model>> {
    let db = &data.db;
    let option: Option<auth_user::Model> = AuthUser::find_by_id(id.into_inner()).one(db).await?;
    ok_rsp(option)
}
/// 新增用户数据
#[utoipa::path(
    post,
    path = "/user",
    request_body = dto::user::AddReq,
    responses((status = OK, body = ApiResponse<Option<auth_user::Model>>)),
    tag = "用户管理模块"
)]
#[post("/user")]
pub async fn add_user(
    req: web::Json<dto::user::AddReq>,
    data: web::Data<AppResources>,
) -> rsp::ApiResult<Option<auth_user::Model>> {
    req.validate()?;
    let txn = data.db.begin().await?;
    let option: Option<auth_user::Model> = AuthUser::find()
        .filter(Condition::all().add(auth_user::Column::Username.eq(req.username.clone())))
        .one(&txn)
        .await?;
    if option.is_some() {
        txn.commit().await?;
        return Err(CommonError {
            code: "1001",
            msg: "用户名已存在",
        }
        .into());
    }

    let option: Option<auth_user::Model> = AuthUser::find()
        .filter(Condition::all().add(auth_user::Column::Email.eq(req.email.clone())))
        .one(&txn)
        .await?;
    if option.is_some() {
        txn.commit().await?;
        return Err(CommonError {
            code: "1001",
            msg: "邮箱已存在",
        }
            .into());
    }

    let x = auth_user::ActiveModel {
        username: Set(req.username.clone()),
        password: Set(bcrypt::hash(req.password.clone(), bcrypt::DEFAULT_COST).unwrap()),
        email: Set(req.email.clone()),
        ..Default::default()
    };
    let s = AuthUser::insert(x).exec_with_returning(&txn).await?;
    txn.commit().await?;
    ok_rsp(Some(s))
}
