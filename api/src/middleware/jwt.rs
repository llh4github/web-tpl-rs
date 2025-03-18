use crate::rsp::ApiResponse;
use crate::rsp::code::{JWT_TOKEN_ERR, UNKNOWN_ERR};
use crate::util;
use crate::rsp::AppErrors;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error, web,
};
use cache::RedisConnectionManager;
use common::cfg::AppCfg;
use futures_util::future::{LocalBoxFuture, Ready, ready};
use r2d2::Pool;
use serde_json::json;
use std::sync::OnceLock;
use std::task::{Context, Poll};

pub struct Jwt;
pub struct JwtService<S> {
    service: S,
}
impl<S: 'static, B> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtService { service }))
    }
}
static MATCHER: OnceLock<matchit::Router<String>> = OnceLock::new();
impl<S, B> Service<ServiceRequest> for JwtService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 跳过 OPTIONS 方法（用于 CORS 预检）
        if req.method() == "OPTIONS" {
            return Box::pin(self.service.call(req));
        }

        let cfg = req.app_data::<web::Data<AppCfg>>();
        let cfg = match cfg {
            Some(data) => data,
            None => {
                log::error!("无法读取配置文件内的数据");
                return Box::pin(async { error_json("无法读取配置文件内的数据") });
            }
        };
        let redis_pool = req.app_data::<web::Data<Pool<RedisConnectionManager>>>();
        let redis_pool = match redis_pool {
            Some(data) => data,
            None => {
                log::error!("无法获取 redis 连接池");
                return Box::pin(async { error_json("无法获取 redis 连接池") });
            }
        };

        let jwt_cfg = &cfg.jwt;
        let matcher = MATCHER.get_or_init(|| {
            // let jwt_cfg = &cfg.jwt;
            let mut router = matchit::Router::new();
            // log::debug!("uri( {:?} ) is anno", &jwt_cfg.anno_url);
            for x in &jwt_cfg.anno_url {
                router.insert(x.clone(), format!("value-{}", x)).unwrap();
            }
            router
        });
        let uri = req.uri().to_string();
        let rs = matcher.at(&*uri);
        // log::debug!("match rs {:?}", rs);
        if rs.is_ok() {
            return Box::pin(self.service.call(req));
        }

        // 提取 Authorization 头
        let auth_header = req.headers().get(&jwt_cfg.header_name);
        let token = auth_header
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix(&jwt_cfg.header_prefix))
            .and_then(|h| Some(h.to_string()));

        let token = match token {
            Some(t) => t,
            None => {
                return Box::pin(async { error_json("无Token信息") });
            }
        };

        let mut pool = match redis_pool.get() {
            Ok(p) => p,
            Err(e) => {
                log::error!("redis pool error : {}", e);
                return Box::pin(async {
                    let msg = ApiResponse::error(JWT_TOKEN_ERR, "Redis连接池错误");
                    let msg = json!(msg).to_string();
                    Err(error::ErrorUnauthorized(msg))
                });
            }
        };
        let validate_result = util::validat_token(&mut pool, &cfg.cache, &jwt_cfg, token.clone());
        match validate_result {
            Ok(_) => {
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            }
            Err(error) => Box::pin(async move {
                log::debug!("token 验证未通过：{} {}", error, token.clone());
                match error {
                    AppErrors::JwtValidateErr { token: _, source } => match source.kind() {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                            error_json("Token已过期")
                        }
                        _ => error_json("无效 Token"),
                    },
                    _ => error_json("无效 Token"),
                }
            }),
        }
    }
}

fn error_json<T>(msg: &str) -> Result<T, Error> {
    let msg = ApiResponse::error(UNKNOWN_ERR, msg);
    let msg = json!(msg).to_string();
    Err(error::ErrorInternalServerError(msg))
}
