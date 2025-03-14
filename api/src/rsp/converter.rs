use crate::rsp::ApiResponse;
use crate::rsp::code::{DB_ERR, POOL_ERR};
use crate::rsp::types::{ErrorDetail, FieldError};
use redis::RedisError;
use sea_orm::DbErr;
use serde_json::{Value, json};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::ValidationErrors;

use super::code::{PARAMS_VALIDATE_ERR, REDIS_ERR};

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
