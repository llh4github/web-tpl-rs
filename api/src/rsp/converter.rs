//! 转换器
//! 将各种错误转换为ApiResponse
use crate::rsp::ApiResponse;
use crate::rsp::code::{DB_ERR, POOL_ERR};
use crate::rsp::types::{ErrorDetail, FieldError};
use redis::RedisError;
use sea_orm::DbErr;
use serde_json::{Value, json};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::ValidationErrors;

use super::AppErrors;
use super::code::{JWT_TOKEN_ERR, PARAMS_VALIDATE_ERR, REDIS_ERR, SERDE_JSON_ERR, UNKNOWN_ERR};

impl From<AppErrors> for ApiResponse<Value> {
    fn from(errors: AppErrors) -> Self {
        let (code, msg, data) = match errors {
            AppErrors::CommonErr(msg) => (UNKNOWN_ERR, msg, Value::Null),
            AppErrors::JwtValidateErr {
                token: _,
                source: _,
            } => (JWT_TOKEN_ERR, "凭证无效".to_string(), Value::Null),
            AppErrors::JwtCreateErr(_error) => {
                (JWT_TOKEN_ERR, "Token创建失败".to_string(), Value::Null)
            }
            AppErrors::PoolErr(_error) => (POOL_ERR, "资源池化出错".to_string(), Value::Null),
            AppErrors::RedisErr(_redis_error) => (REDIS_ERR, "redis出错".to_string(), Value::Null),
            AppErrors::SerdeJsonErr(_error) => (
                SERDE_JSON_ERR,
                "序列化或反序列化出错".to_string(),
                Value::Null,
            ),
            // _ => (UNKNOWN_ERR, "未知错误".to_string(), Value::Null),
        };
        ApiResponse::error_with_data(code, msg, data)
    }
}

fn convert_params(params: HashMap<Cow<'static, str>, Value>) -> Option<Value> {
    let json_value = serde_json::to_value(params).unwrap_or(Value::Null);
    Some(json_value)
}
pub(crate) fn convert_validation_errors(err: &ValidationErrors) -> Value {
    let error_list: Vec<FieldError> = err
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
    json!(error_list)
}

impl From<ValidationErrors> for ApiResponse<Value> {
    fn from(errors: ValidationErrors) -> Self {
        let errors_json = convert_validation_errors(&errors);
        log::error!("ValidationErrors : {}", errors);
        ApiResponse::error_with_data(PARAMS_VALIDATE_ERR, "参数验证失败", errors_json)
    }
}

impl From<&ValidationErrors> for ApiResponse<Value> {
    fn from(errors: &ValidationErrors) -> Self {
        let errors_json = convert_validation_errors(errors);
        log::error!("ValidationErrors : {}", errors);
        ApiResponse::error_with_data(PARAMS_VALIDATE_ERR, "参数验证失败", errors_json)
    }
}

impl From<DbErr> for ApiResponse<Value> {
    fn from(errors: DbErr) -> Self {
        errors.to_string();
        log::error!("DB run err: {}", errors);
        ApiResponse::error_with_data(DB_ERR, "DB Err", Value::String(errors.to_string()))
    }
}

impl From<r2d2::Error> for ApiResponse<Value> {
    fn from(errors: r2d2::Error) -> Self {
        log::error!("r2d2 Pool run err: {}", errors);
        ApiResponse::error(POOL_ERR, "池化对象出错")
    }
}

impl From<RedisError> for ApiResponse<Value> {
    fn from(errors: RedisError) -> Self {
        log::error!("Redis run err: {}", errors);
        ApiResponse::error(REDIS_ERR, "Redis Error")
    }
}
