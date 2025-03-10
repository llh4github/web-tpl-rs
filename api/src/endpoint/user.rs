use crate::global::AppResources;
use crate::rsp::ApiErrors::CommonError;
use crate::rsp::{ok_rsp, ApiResponse, ApiResult, PageResult};
use crate::{dto, rsp};
use actix_web::{get, post, web};
use db::entities::auth_user;
use db::entities::prelude::AuthUser;
use log::{debug, info};
use redis::Commands;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Condition, EntityTrait, IntoActiveModel, TransactionTrait};
use sea_orm::{ColumnTrait, QueryOrder};
use sea_orm::{PaginatorTrait, QueryFilter};
use serde_json::json;
use validator::Validate;

const REDIS_KEY: &str = "web-tpl:cache:user";

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
) -> ApiResult<Option<auth_user::Model>> {
    let mut pool = data.redis_pool.get()?;
    let cached: Option<String> = redis::cmd("GET")
        .arg(format!("{}:{}", REDIS_KEY, *id))
        .query(&mut pool)?;
    if let Some(cached) = cached {
        debug!("Cache found. user-id {}", *id);
        let cached: Option<auth_user::Model> = serde_json::from_str(&cached).unwrap();
        return ok_rsp(cached);
    } else {
        debug!("Cache not found, run db query. user-id {}", *id);
    }

    let db = &data.db;
    let option: Option<auth_user::Model> = AuthUser::find_by_id(*id).one(db).await?;
    redis::cmd("SET")
        .arg(format!("{}:{}", REDIS_KEY, *id))
        .arg(json!(option).to_string())
        .arg("EX")
        .arg(60 * 60)
        .exec(&mut pool)?;

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
) -> ApiResult<Option<auth_user::Model>> {
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

/// 分页查询
#[utoipa::path(
    post,
    path = "/user/page",
    request_body = dto::user::PageReq,
    responses((status = OK, body = ApiResponse<PageResult<auth_user::Model>>)),
    tag = "用户管理模块"
)]
#[post("/user/page")]
pub async fn page_query(
    req: web::Json<dto::user::PageReq>,
    data: web::Data<AppResources>,
) -> rsp::ApiResult<PageResult<auth_user::Model>> {
    let db = &data.db;
    let cond = Condition::all()
        .add_option(req.username.as_ref().map_or(None::<SimpleExpr>, |v| {
            Some(auth_user::Column::Username.contains(v.clone()))
        }))
        .add_option(req.email.as_ref().map_or(None::<SimpleExpr>, |v| {
            Some(auth_user::Column::Email.contains(v.clone()))
        }));
    let query = AuthUser::find()
        .filter(cond)
        .order_by_desc(auth_user::Column::UpdatedAt);
    let paginator = query.paginate(db, req.size);
    let total_page = paginator.num_pages().await?;
    let total_ele = paginator.num_items().await?;
    let list: Vec<auth_user::Model> = paginator.fetch_page(req.page - 1).await?;
    let rs = PageResult {
        total_page,
        total_ele,
        data: list,
    };
    ok_rsp(rs)
}
/// 更新用户密码
#[utoipa::path(
    post,
    path = "/user/update/pwd",
    request_body = dto::user::UpdatePwd,
    responses((status = OK, body = ApiResponse<bool>)),
    tag = "用户管理模块"
)]
#[post("/user/update/pwd")]
pub async fn update_pwd(
    req: web::Json<dto::user::UpdatePwd>,
    data: web::Data<AppResources>,
) -> ApiResult<bool> {
    let db = &data.db;
    let txn = db.begin().await?;
    let option: Option<auth_user::Model> = AuthUser::find_by_id(req.id).one(&txn).await?;
    match option {
        None => {
            txn.commit().await?;
            Err(ApiResponse::error("DataNotFound", "Data not found"))
        }
        Some(m) => {
            let mut data_db = m.into_active_model();
            data_db.password =
                Set(bcrypt::hash(req.password.clone(), bcrypt::DEFAULT_COST).unwrap());
            data_db.updated_at = Set(Some(chrono::Local::now().naive_local()));
            data_db.update(&txn).await?;
            txn.commit().await?;
            ok_rsp(true)
        }
    }
}
