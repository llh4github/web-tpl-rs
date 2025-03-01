use actix_web::http::header::TryIntoHeaderValue;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, ResponseError};
use serde::Serialize;
use serde_json::{json, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use thiserror::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

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
    data: Value,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .json(self)
    }
}
fn convert_validation_errors(err: &ValidationErrors) -> Value {
    let error_map: Vec<FieldError> = err
        .field_errors()
        .iter()
        .map(|(field, errors)| FieldError {
            field: field.to_string(),
            error_detail: errors
                .iter()
                .map(|validation_error| ErrorDetail {
                    code: validation_error.code.to_string(),
                    message: validation_error.message.clone().map(|m| m.to_string()),
                    params: convert_params(validation_error.params.clone()),
                })
                .collect(),
        })
        .collect();
    json!({ "field_errors": error_map })
}


#[derive(Serialize)]
pub struct FieldError {
    /// 字段名称
    field: String,
    /// 该字段的错误详情
    error_detail: Vec<ErrorDetail>,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    /// 错误类型代码 (如 "email", "length" 等)
    code: String,
    /// 人类可读的错误信息
    message: Option<String>,
    /// 额外参数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

fn convert_params(params: HashMap<Cow<'static, str>, Value>) -> Option<Value> {
    let a = serde_json::to_value(params).unwrap_or(Value::Null);
    Some(a)
}
// 实现 From<ValidationErrors> 转换
impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let errors_json = convert_validation_errors(&errors);
        ApiError {
            code: "Validator".to_string(),
            msg: "ValidationErrors".to_string(),
            success: false,
            data: errors_json,
        }
    }
}
