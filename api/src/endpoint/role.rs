use actix_web::{get, web};
use cache::RedisConnectionManager;
use common::cfg::AppCfg;
use db::entities::{auth_role, prelude::AuthRole};
use r2d2::Pool;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    rsp::{ApiResponse, ApiResult, ok_rsp},
    util::CacheKeyUtil,
};

pub(super) fn register_api(c: &mut ServiceConfig) {
    c.service(get_role);
}

const REDIS_KEY: &str = "role-module";

/// 根据 ID 查找数据
#[utoipa::path(
    get,
    path = "/role/{id}",
    params(
        ("id" = i32, description = "Data ID")
    ),
    responses((status = OK, body = ApiResponse<Option<auth_role::Model>>)),
    tag = "角色管理模块"
)]
#[get("/role/{id}")]
async fn get_role(
    id: web::Path<i32>,
    cfg: web::Data<AppCfg>,
    redis_inject: web::Data<Pool<RedisConnectionManager>>,
    cache_key_util: web::Data<CacheKeyUtil>,
    db_inject: web::Data<DatabaseConnection>,
) -> ApiResult<Option<auth_role::Model>> {
    let key = cache_key_util.cache_key_i32(REDIS_KEY, *id);
    let mut pool = redis_inject.get()?;
    let cached: Option<String> = redis::cmd("GET").arg(&key).query(&mut pool)?;

    if let Some(cached) = cached {
        log::debug!("Cache found. role-id {}", *id);
        let cached: Option<auth_role::Model> = serde_json::from_str(&cached).unwrap();
        return ok_rsp(cached);
    } else {
        log::debug!("Cache not found, run db query. role-id {}", *id);
    }

    let db = db_inject.get_ref();

    let option: Option<auth_role::Model> = AuthRole::find_by_id(*id).one(db).await?;
    redis::cmd("SET")
        .arg(&key)
        .arg(json!(option).to_string())
        .arg("EX")
        .arg(cfg.cache.ttl)
        .exec(&mut pool)?;

    ok_rsp(option)
}

