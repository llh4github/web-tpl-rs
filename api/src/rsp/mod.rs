//! 统一响应体处理
//!
//! 正确与错误响应均用  `ApiResponse` 实现
//!
//! converter.rs 中存放各种error向 `ApiResponse` 转化逻辑。
//! *由于并非所有error都支持序列化，故不采用枚举+thiserror的形式*
pub mod code;
mod converter;
pub mod types;
mod wrapper;

use common::OK_STR;
use serde::Serialize;
pub use types::PageResult;
pub use wrapper::ApiResponse;

/// ApiResult 接口统一响应结果
pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<serde_json::Value>>;

pub fn ok_rsp<T: Serialize>(data: T) -> ApiResult<T> {
    Ok(ApiResponse::success(data))
}
pub fn ok_with_msg<T: Serialize>(data: T, msg: Into<String>) -> ApiResult<T> {
    Ok(ApiResponse::success_with_msg(data, msg))
}

pub fn error_rsp(code: code::ErrorCode, msg: Into<String>) -> ApiResult<T> {
    Err(ApiResponse::error(code, msg))
}

pub fn error_rsp_data(code: code::ErrorCode, msg: Into<String>, data: Value) -> ApiResult<T> {
    Err(ApiResponse::error_with_data(code, msg, data))
}
