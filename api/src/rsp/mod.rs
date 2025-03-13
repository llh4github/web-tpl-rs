mod converter;
pub mod types;
mod wrapper;
pub mod errors;

use common::OK_STR;
use serde::Serialize;
pub use types::PageResult;
pub use wrapper::ApiErrors;
pub use wrapper::ApiResponse;
pub use wrapper::ApiResult;
pub use wrapper::ApiResult2;

pub fn ok_rsp<T: Serialize>(data: T) -> ApiResult<T> {
    Ok(ApiResponse::success(data))
}
pub fn ok_with_msg<T: Serialize>(data: T, msg: &str) -> ApiResult<T> {
    Ok(ApiResponse {
        data,
        msg: msg.to_string(),
        code: OK_STR.to_string(),
        success: true,
    })
}
