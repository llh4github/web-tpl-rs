use std::error::Error;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
    
    #[error("Resource not found")]
    NotFound,

}
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    code: u16,          // HTTP 状态码（如 400、500）
    error_type: String, // 错误类型标识（如 "invalid_input"）
    message: String,    // 用户可读的错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>, // 可选的错误详情（如错误链）
}

impl ResponseError for MyError {
    // 根据错误类型返回对应的 HTTP 状态码
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            MyError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    // 生成 JSON 格式的错误响应
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            error_type: self.error_type().to_string(),
            message: self.to_string(),
            details: self.details(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

// 辅助方法：获取错误类型标识
impl MyError {
    fn error_type(&self) -> &str {
        match self {
            MyError::InvalidInput(_) => "invalid_input",
            MyError::Internal(_) => "internal_error",
            MyError::NotFound => "not_found",
        }
    }

    // 提取错误链详情
    fn details(&self) -> Option<String> {
        if let Some(source) = self.source() {
            let mut details = String::new();
            let mut current: &dyn std::error::Error = source;
            details.push_str(&format!("{}", current));
            while let Some(next) = current.source() {
                details.push_str(&format!("\nCaused by: {}", next));
                current = next;
            }
            Some(details)
        } else {
            None
        }
    }
}