use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use utoipa::gen::serde_json;
use utoipa::gen::serde_json::Value;
use validator::ValidationErrors;

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    /// 字段错误列表
    field_errors: Vec<FieldError>,
}

#[derive(Serialize)]
pub struct FieldError {
    /// 字段名称
    field: String,
    /// 该字段的错误详情
    errors: Vec<ErrorDetail>,
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

impl From<&ValidationErrors> for ValidationErrorResponse {
    fn from(errors: &ValidationErrors) -> Self {
        ValidationErrorResponse {
            field_errors: errors
                .field_errors()
                .iter()
                .map(|(field_name, errors)| FieldError {
                    field: field_name.to_string(),
                    errors: errors
                        .iter()
                        .map(|validation_error| ErrorDetail {
                            code: validation_error.code.to_string(),
                            message: validation_error.message.clone().map(|m| m.to_string()),
                            params: convert_params(validation_error.params.clone()),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
fn convert_params(params: HashMap<Cow<'static, str>, Value>) -> Option<Value> {
    let a = serde_json::to_value(params).unwrap_or(Value::Null);
    Some(a)
}
