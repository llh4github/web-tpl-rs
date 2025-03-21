use crate::{
    dto::{self},
    rsp::{ApiResponse, ApiResult, PageResult, code::DATA_EXIST_ERR, error_rsp, ok_rsp},
    util::ReidsUtil,
};
use actix_web::{get, post, web};
use chrono::Utc;
use db::entities::{auth_role, prelude::AuthRole};
use sea_orm::{
    ActiveValue::Set, ColumnTrait as _, Condition, DatabaseConnection, EntityTrait,
    PaginatorTrait as _, QueryFilter as _, QueryOrder, TransactionTrait as _,
    sea_query::SimpleExpr,
};
use utoipa_actix_web::service_config::ServiceConfig;
use validator::Validate;

pub(super) fn register_api(c: &mut ServiceConfig) {
    c.service(get_role);
    c.service(add_role);
    c.service(page_query);
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

/// 新增数据
#[utoipa::path(
    post,
    path = "/role",
    request_body = dto::role::AddReq,
    responses((status = OK, body = ApiResponse<Option<auth_role::Model>>)),
    tag = "角色管理模块"
)]
#[post("/role")]
async fn add_role(
    req: web::Json<dto::role::AddReq>,
    db_inject: web::Data<DatabaseConnection>,
) -> ApiResult<Option<auth_role::Model>> {
    req.validate()?;
    let txn = db_inject.get_ref().begin().await?;

    let option: Option<auth_role::Model> = AuthRole::find()
        .filter(Condition::all().add(auth_role::Column::Code.eq(req.code.clone())))
        .one(&txn)
        .await?;
    if option.is_some() {
        txn.rollback().await?;
        return error_rsp(DATA_EXIST_ERR, "角色代码已存在");
    }
    let model = auth_role::ActiveModel {
        name: Set(req.name.clone()),
        code: Set(req.code.clone()),
        created_at: Set(Some(Utc::now().naive_local())),
        ..Default::default()
    };
    let model = AuthRole::insert(model).exec_with_returning(&txn).await?;
    txn.commit().await?;

    ok_rsp(Some(model))
}

/// 分页查询
#[utoipa::path(
    post,
    path = "/role/page",
    request_body = dto::role::PageReq,
    responses((status = OK, body = ApiResponse<PageResult<auth_role::Model>>)),
    tag = "角色管理模块"
)]
#[post("/role/page")]
async fn page_query(
    req: web::Json<dto::role::PageReq>,
    db_inject: web::Data<DatabaseConnection>,
) -> ApiResult<PageResult<auth_role::Model>> {
    let cond = Condition::all()
        .add_option(req.code.as_ref().map_or(None::<SimpleExpr>, |v| {
            Some(auth_role::Column::Code.contains(v))
        }))
        .add_option(req.name.as_ref().map_or(None::<SimpleExpr>, |v| {
            Some(auth_role::Column::Name.contains(v))
        }));

    let query = AuthRole::find()
        .filter(cond)
        .order_by_desc(auth_role::Column::UpdatedAt);

    let db = db_inject.get_ref();
    let paginator = query.paginate(db, req.param.size);
    let total_page = paginator.num_pages().await?;
    let total_ele = paginator.num_items().await?;
    let list: Vec<auth_role::Model> = paginator.fetch_page(req.param.page - 1).await?;
    let rs = PageResult {
        total_page,
        total_ele,
        data: list,
    };
    ok_rsp(rs)
}
