use actix_web::{post, web};
use common::util::pwd_util;
use db::entities::{auth_user, prelude::AuthUser};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    dto,
    global::AppResources,
    rsp::{ApiResponse, ApiResult, error_rsp, ok_rsp},
    util::create_and_cache_token,
};

pub(super) fn register_api(c: &mut ServiceConfig) {
    c.service(login);
}

const LOGIN_FAIL: &str = "LOGIN_FAIL";
const LOGIN_FAIL_MSG: &str = "用户名或密码不正确";

/// 用户登录接口
#[utoipa::path(
    post,
    path = "/login",
    request_body = dto::user::LoginReq,
    responses((status = OK, body = ApiResponse<Option<dto::user::LoginToken>>)),
    tag = "用户管理模块"
)]
#[post("/login")]
async fn login(
    req: web::Json<dto::user::LoginReq>,
    data: web::Data<AppResources>,
) -> ApiResult<dto::user::LoginToken> {
    let db = &data.db;
    let option: Option<auth_user::Model> = AuthUser::find()
        .filter(auth_user::Column::Username.eq(req.username.clone()))
        .one(db)
        .await?;
    let user = match option {
        Some(u) => u,
        None => {
            log::debug!("username( {} ) not found", req.username);
            return error_rsp(LOGIN_FAIL, LOGIN_FAIL_MSG);
        }
    };
    if !pwd_util::is_match_pwd(&req.password, user.password) {
        log::debug!("username( {} ) password is not matched.", req.username);
        return error_rsp(LOGIN_FAIL, LOGIN_FAIL_MSG);
    }
    let mut pool = data.redis_pool.get()?;
    let token = create_and_cache_token(&mut pool, user.username, &data.cfg.jwt, &data.cfg.cache)?;
    ok_rsp(dto::user::LoginToken { token })
}
