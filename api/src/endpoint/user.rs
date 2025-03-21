use crate::rsp::code::DATA_NOT_FIND_ERR;
use crate::rsp::{ApiResponse, ApiResult, PageResult, error_rsp, ok_rsp};
use crate::util::ReidsUtil;
use crate::{dto, rsp};
use actix_web::{get, post, web};
use common::util::pwd_util;
use db::entities::prelude::{AuthRole, AuthUser, LinkUserRole};
use db::entities::{auth_role, auth_user, link_user_role};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::sqlx::types::chrono;
use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, EntityTrait as _, IntoActiveModel,
    LoaderTrait, QuerySelect, TransactionTrait,
};
use sea_orm::{ColumnTrait, QueryOrder};
use sea_orm::{PaginatorTrait, QueryFilter};
use utoipa_actix_web::service_config::ServiceConfig;
use validator::Validate;

const REDIS_KEY: &str = "user-module";

pub(super) fn register_api(c: &mut ServiceConfig) {
    c.service(find_user);
    c.service(add_user);
    c.service(page_query);
    c.service(update_pwd);
}

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
    redis_util: web::Data<ReidsUtil>,
    db_inject: web::Data<DatabaseConnection>,
) -> ApiResult<Option<auth_user::Model>> {
    let key = format!("{}:{}", REDIS_KEY, id);

    let cached: Option<auth_user::Model> = redis_util.fetch_and_dejson(&key)?;
    if let Some(cached) = cached {
        return ok_rsp(Some(cached));
    }

    let db = db_inject.get_ref();
    let option: Option<auth_user::Model> = AuthUser::find_by_id(*id).one(db).await?;
    redis_util.cache_json_str(&key, &option)?;

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
    db_inject: web::Data<DatabaseConnection>,
) -> ApiResult<Option<auth_user::Model>> {
    req.validate()?;
    let txn = db_inject.get_ref().begin().await?;
    let option: Option<auth_user::Model> = AuthUser::find()
        .filter(Condition::all().add(auth_user::Column::Username.eq(req.username.clone())))
        .one(&txn)
        .await?;
    if option.is_none() {
        txn.commit().await?;
        return error_rsp(DATA_NOT_FIND_ERR, format!("Username: {}", req.username));
    }

    let option: Option<auth_user::Model> = AuthUser::find()
        .filter(Condition::all().add(auth_user::Column::Email.eq(req.email.clone())))
        .one(&txn)
        .await?;
    if option.is_none() {
        txn.commit().await?;
        return error_rsp(DATA_NOT_FIND_ERR, format!("Email: {}", req.email));
    }

    let pwd_hash = pwd_util::hash_pwd(&req.username);
    let x = auth_user::ActiveModel {
        username: Set(req.username.clone()),
        password: Set(pwd_hash),
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
    db_inject: web::Data<DatabaseConnection>,
) -> rsp::ApiResult<PageResult<dto::user::PageEle>> {
    let db = db_inject.get_ref();
    let db = db.begin().await?;
    let cond = Condition::all()
        .add_option(req.username.as_ref().map_or(None::<SimpleExpr>, |v| {
            Some(auth_user::Column::Username.contains(v.clone()))
        }))
        .add_option(req.email.as_ref().map_or(None::<SimpleExpr>, |v| {
            Some(auth_user::Column::Email.contains(v.clone()))
        }));

    let query = AuthUser::find()
        .filter(cond)
        .order_by_desc(auth_user::Column::UpdatedAt)
        .into_partial_model::<dto::user::PageEle>();
    let paginator = query.paginate(&db, req.param.size);
    let total_page = paginator.num_pages().await?;
    let total_ele = paginator.num_items().await?;
    let mut list = paginator.fetch_page(req.param.page - 1).await?;
    let user_ids = list.iter().map(|v| v.id).collect::<Vec<i32>>();
    let link_data = LinkUserRole::find()
        .filter(link_user_role::Column::UserId.is_in(user_ids))
        .all(&db)
        .await?;
    let roles = AuthRole::find()
        .filter(
            auth_role::Column::Id.is_in(link_data.iter().map(|v| v.role_id).collect::<Vec<i32>>()),
        )
        .into_partial_model::<dto::user::RoleInfo>()
        .all(&db)
        .await?;

    for ele in list.iter_mut() {
        let role_info = roles
            .iter()
            .filter(|v| {
                link_data
                    .iter()
                    .any(|x| x.role_id == v.id && x.user_id == ele.id)
            })
            .map(|v| dto::user::RoleInfo {
                id: v.id,
                name: v.name.clone(),
                code: v.code.clone(),
            })
            .collect::<Vec<dto::user::RoleInfo>>();
        ele.roles = role_info;
    }
    let rs = PageResult {
        total_page,
        total_ele,
        data: list,
    };
    db.commit().await?;
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
    db_inject: web::Data<DatabaseConnection>,
) -> ApiResult<bool> {
    let db = db_inject.get_ref();
    let txn = db.begin().await?;
    let option: Option<auth_user::Model> = AuthUser::find_by_id(req.id).one(&txn).await?;
    match option {
        None => {
            txn.commit().await?;
            error_rsp(DATA_NOT_FIND_ERR, format!("id: {}", req.id))
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
