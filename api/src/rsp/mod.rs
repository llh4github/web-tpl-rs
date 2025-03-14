mod converter;
pub mod errors;
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
pub fn ok_with_msg<T: Serialize>(data: T, msg: &str) -> ApiResult<T> {
    Ok(ApiResponse::success_with_msg(data, msg))
}
