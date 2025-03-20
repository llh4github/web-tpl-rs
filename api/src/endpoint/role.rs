use actix_web::{get, web};
use db::entities::{auth_role, prelude::AuthRole};
use sea_orm::{DatabaseConnection, EntityTrait};
use utoipa_actix_web::service_config::ServiceConfig;
use crate::{
    rsp::{ApiResponse, ApiResult, ok_rsp},
    util::ReidsUtil,
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
    redis_util: web::Data<ReidsUtil>,
    db_inject: web::Data<DatabaseConnection>,
) -> ApiResult<Option<auth_role::Model>> {
    let key = format!("{}:{}", REDIS_KEY, id);
    let cached: Option<auth_role::Model> = redis_util.fetch_and_dejson(&key)?;
    if let Some(cached) = cached {
        return ok_rsp(Some(cached));
    }

    let db = db_inject.get_ref();

    let option: Option<auth_role::Model> = AuthRole::find_by_id(*id).one(db).await?;
    redis_util.cache_json_str(&key, &option)?;
    ok_rsp(option)
}
