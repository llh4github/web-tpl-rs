//! 响应包装器
//! 用于包装响应数据，统一响应格式
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use chrono::Utc;
use common::{OK_STR, SUCCESS_STR};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Error, Debug)]
#[error("ApiResponse: {code} {msg} {success} {data}")]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub success: bool,
    pub data: T,
    #[serde(serialize_with = "serialize_timestamp")]
    timestamp: (),
}

fn serialize_timestamp<S>(_: &(), serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let dt = Utc::now();
    serializer.serialize_i64(dt.timestamp_millis())
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应构造函数
    pub fn success(data: T) -> Self {
        Self {
            data,
            code: SUCCESS_STR.to_string(),
            msg: OK_STR.to_string(),
            success: true,
            timestamp: (),
        }
    }
    pub fn success_with_msg(data: T, msg: impl Into<String>) -> Self {
        Self {
            data,
            code: SUCCESS_STR.to_string(),
            msg: msg.into(),
            success: true,
            timestamp: (),
        }
    }
}
impl ApiResponse<Value> {
    /// 错误响应构造函数
    pub fn error(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            data: Value::Null,
            code: code.into(),
            msg: msg.into(),
            success: false,
            timestamp: (),
        }
    }

    pub fn error_with_data(code: impl Into<String>, msg: impl Into<String>, data: Value) -> Self {
        Self {
            data,
            code: code.into(),
            msg: msg.into(),
            success: false,
            timestamp: (),
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
        HttpResponse::build(StatusCode::OK).json(self)
    }
}
