use crate::rsp::types::{ErrorDetail, FieldError};
use crate::rsp::ApiResponse;
use log::debug;
use sea_orm::DbErr;
use serde_json::{json, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::ValidationErrors;

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
        ApiResponse {
            code: "Validator".to_string(),
            msg: "ValidationErrors".to_string(),
            success: false,
            data: errors_json,
        }
    }
}

impl From<DbErr> for ApiResponse<Value> {
    fn from(errors: DbErr) -> Self {
        errors.to_string();

        debug!("DbErr: {:?}", errors);
        ApiResponse {
            code: "DbErr".to_string(),
            msg: "DbErr".to_string(),
            success: false,
            data: Value::String(errors.to_string()),
        }
    }
}

impl From<&ValidationErrors> for ApiResponse<Value> {
    fn from(errors: &ValidationErrors) -> Self {
        let errors_json = convert_validation_errors(errors);
        ApiResponse {
            code: "Validator".to_string(),
            msg: "ValidationErrors".to_string(),
            success: false,
            data: errors_json,
        }
    }
}
