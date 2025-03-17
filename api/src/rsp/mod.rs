//! 统一响应体处理
//!
//! 正确与错误响应均用  `ApiResponse` 实现
//!
//! converter.rs 中存放各种error向 `ApiResponse` 转化逻辑。
//! *由于并非所有error都支持序列化，故不采用枚举+thiserror的形式*
#[allow(dead_code)]
pub mod code;
mod converter;
mod errors;
pub mod types;
mod wrapper;

pub use errors::AppErrors;
use serde::Serialize;
pub use types::PageResult;
pub use wrapper::ApiResponse;


/// ApiResult 接口统一响应结果
pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<serde_json::Value>>;

#[allow(dead_code)]
pub fn ok_rsp<T: Serialize>(data: T) -> ApiResult<T> {
    Ok(ApiResponse::success(data))
}
#[allow(dead_code)]
pub fn ok_with_msg<T: Serialize>(data: T, msg: impl Into<String>) -> ApiResult<T> {
    Ok(ApiResponse::success_with_msg(data, msg))
}

#[allow(dead_code)]
pub fn error_rsp<T: Serialize>(code: impl Into<String>, msg: impl Into<String>) -> ApiResult<T> {
    Err(ApiResponse::error(code, msg))
}

#[allow(dead_code)]
pub fn error_rsp_data<T: Serialize>(
    code: impl Into<String>,
    msg: impl Into<String>,
    data: serde_json::Value,
) -> ApiResult<T> {
    Err(ApiResponse::error_with_data(code, msg, data))
}
