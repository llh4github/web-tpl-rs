use crate::global::AppResources;
use crate::util;
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, error, web, Error, HttpMessage};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::task::{Context, Poll};

pub struct Jwt;
pub struct JwtService<S> {
    service: S,
}
impl<S: 'static, B> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
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
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
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

        // 提取 Authorization 头
        let auth_header = req.headers().get("Authorization");
        let token = auth_header
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .and_then(|h| Some(h.to_string()));

        let token = match token {
            Some(t) => t,
            None => {
                return Box::pin(async {
                    Err(error::ErrorUnauthorized("缺少 Authorization 头"))
                });
            }
        };
        let jwt_cfg = resources.cfg.jwt;
        let decode = util::parse_token(&jwt_cfg, token);
        match decode {
            Ok(claims) => {
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            }
            Err(error) => Box::pin(async move {
                match error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Err(error::ErrorUnauthorized("Token 已过期"))
                    }
                    _ => Err(error::ErrorUnauthorized("无效 Token")),
                }
            }),
        }
    }
}
