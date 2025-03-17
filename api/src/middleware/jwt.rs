use crate::rsp::ApiResponse;
use crate::rsp::code::JWT_TOKEN_ERR;
use crate::util;
use crate::{global::AppResources, rsp::AppErrors};
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error, web,
};
use futures_util::future::{LocalBoxFuture, Ready, err, ready};
use serde_json::json;
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

        let resources = req
            .app_data::<web::Data<AppResources>>()
            .expect("cfg::Settings is not found")
            .get_ref()
            .clone();
        let jwt_cfg = resources.cfg.jwt;

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
        let valiadte_result =
            util::validat_token(&mut pool, &resources.cfg.cache, &jwt_cfg, token.clone());
        match valiadte_result {
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
