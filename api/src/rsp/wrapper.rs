use crate::rsp::converter::convert_validation_errors;
use actix_web::http::StatusCode;
use actix_web::http::header::TryIntoHeaderValue;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use common::{OK_STR, SUCCESS_STR};
use serde::Serialize;
use serde_json::Value;
use std::fmt::{Debug, Display};
use thiserror::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

use super::errors::MyError;

/// ApiResult 接口统一响应结果
pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<Value>>;
pub type ApiResult2<T> = Result<ApiResponse<T>, MyError>;

#[derive(Serialize, ToSchema)]
pub struct EmptyData;
#[derive(Serialize, ToSchema, Error, Debug)]
#[error("ApiResponse: {code} {msg} {success} {data}")]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应构造函数
    pub fn success(data: T) -> Self {
        Self {
            data,
            code: SUCCESS_STR.to_string(),
            msg: OK_STR.to_string(),
            success: true,
        }
    }
}
impl ApiResponse<Value> {
    // 错误响应构造函数
    pub fn error(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            data: Value::Null,
            code: code.into(),
            msg: msg.into(),
            success: false,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl ResponseError for ApiResponse<Value> {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .json(self)
    }
}

#[deprecated(since = "0.1.0", note = "Use ApiResponse instead")]
#[derive(Serialize, Debug, Error)]
pub enum ApiErrors<'a> {
    #[error("CommonError: {code} {msg}")]
    CommonError { code: &'a str, msg: &'a str },
    #[error("ValidationError: {0}")]
    ValidationError(#[from] ValidationErrors),
    // DbError(#[from] DbErr),
}
impl<'a> From<ApiErrors<'a>> for ApiResponse<Value> {
    fn from(value: ApiErrors<'a>) -> Self {
        match value {
            ApiErrors::CommonError { code, msg } => ApiResponse {
                code: code.to_string(),
                msg: msg.to_string(),
                success: false,
                data: Value::Null,
            },
            ApiErrors::ValidationError(errors) => {
                let errors_json = convert_validation_errors(&errors);
                ApiResponse {
                    code: "Validator".to_string(),
                    msg: "ValidationErrors".to_string(),
                    success: false,
                    data: errors_json,
                }
            } // ApiErrors::DbError(_) => ApiResponse {
              //     code: "DbError".to_string(),
              //     msg: "DbError".to_string(),
              //     success: false,
              //     data: Value::Null,
              // },
        }
    }
}
