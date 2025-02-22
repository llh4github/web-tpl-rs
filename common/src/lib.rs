use std::string::ToString;

const OK: &str = "OK";
/// Json响应统一包装体
#[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
pub struct RspWrapper<T> {
    /// 响应消息
    msg: String,
    /// 响应码
    code: String,
    /// 是否成功
    success: bool,
    /// 响应数据体
    data: Option<T>,
}
impl<T> RspWrapper<T> {
    /// 带数据的正常响应
    pub fn ok(data: T) -> RspWrapper<T> {
        RspWrapper {
            msg: OK.to_string(),
            code: "".to_string(),
            success: true,
            data: Some(data),
        }
    }
    /// 不带数据的正常响应
    pub fn ok_empty() -> RspWrapper<T> {
        RspWrapper {
            msg: OK.to_string(),
            code: "".to_string(),
            success: true,
            data: None,
        }
    }

    /// 不带数据的错误响应
    pub fn error(msg: String, code: String) -> RspWrapper<T> {
        RspWrapper {
            msg,
            code,
            success: false,
            data: None,
        }
    }
}
