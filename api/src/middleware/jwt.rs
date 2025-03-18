use crate::rsp::code::{JWT_TOKEN_ERR, UNKNOWN_ERR};
use crate::rsp::ApiResponse;
use crate::util;
use crate::{global::AppResources, rsp::AppErrors};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error,
    web, Error,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
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

        let resources = req.app_data::<web::Data<AppResources>>();
        let resources = match resources {
            Some(data) => data,
            None => {
                log::error!("无法读取配置文件内的数据");
                return Box::pin(async {
                    let msg = ApiResponse::error(UNKNOWN_ERR, "无法读取应用配置数据");
                    let msg = json!(msg).to_string();
                    Err(error::ErrorInternalServerError(msg))
                });
            }
        };

        let jwt_cfg = &resources.cfg.jwt;
        let matcher = MATCHER.get_or_init(|| {
            let jwt_cfg = &resources.cfg.jwt;
            let mut router = matchit::Router::new();
            // log::debug!("uri( {:?} ) is anno", &jwt_cfg.anno_url);
            for x in &jwt_cfg.anno_url {
                router.insert(x.clone(), format!("value-{}", x)).unwrap();
            }
            router
        });
        let uri = req.uri().to_string();
        let rs = matcher.at(&*uri);
        log::debug!("match rs {:?}", rs);
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
                return Box::pin(async {
                    let msg = ApiResponse::error(JWT_TOKEN_ERR, "无Token信息");
                    let msg = json!(msg).to_string();
                    Err(error::ErrorUnauthorized(msg))
                });
            }
        };

        let mut pool = match resources.redis_pool.get() {
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
        let validate_result =
            util::validat_token(&mut pool, &resources.cfg.cache, &jwt_cfg, token.clone());
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
                            let msg = ApiResponse::error(JWT_TOKEN_ERR, "Token已过期");
                            let msg = json!(msg).to_string();
                            Err(error::ErrorUnauthorized(msg))
                        }
                        _ => {
                            let msg = ApiResponse::error(JWT_TOKEN_ERR, "无效 Token");
                            let msg = json!(msg).to_string();
                            Err(error::ErrorUnauthorized(msg))
                        }
                    },
                    _ => {
                        let msg = ApiResponse::error(JWT_TOKEN_ERR, "无效 Token");
                        let msg = json!(msg).to_string();
                        Err(error::ErrorUnauthorized(msg))
                    }
                }
            }),
        }
    }
}
