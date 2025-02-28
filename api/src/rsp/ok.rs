use actix_web::http::header::TryIntoHeaderValue;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, ResponseError};
use serde::Serialize;
use std::fmt::{Debug, Display};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct EmptyData;
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    data: T,
    code: String,
    msg: String,
    success: bool,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应构造函数
    pub fn success(data: T) -> Self {
        Self {
            data,
            code: "success".to_string(),
            msg: String::new(),
            success: true,
        }
    }


}
impl ApiResponse<EmptyData> {
    // 错误响应构造函数
    pub fn error(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            data: EmptyData,
            code: code.into(),
            msg: msg.into(),
            success: false,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Serialize, ToSchema, Debug, Error)]
#[error("[{code}] {msg}")]
pub struct ApiError {
    code: String,
    msg: String,
    success: bool,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .json(self)
    }
}
// 实现 From<ValidationErrors> 转换
impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages = errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                format!(
                    "{}: {}",
                    field,
                    errs.iter()
                        .map(|e| e.message.clone().unwrap_or_default())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("; ");

        ApiError {
            code: "Validator".to_string(),
            msg: format!("Validation error: {}", messages),
            success: false,
        }
    }
}
