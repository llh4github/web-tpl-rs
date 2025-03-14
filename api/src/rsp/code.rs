/// 接口响应错误码类型
pub type ErrorCode = &str;

pub const DB_ERR :ErrorCode = "DB_ERR";
pub const DB_QUERY_ERR :ErrorCode = "DB_QUERY_ERR";
pub const UNKONWN_ERR :ErrorCode = "UNKONWN_ERR";


pub const DATA_NOT_FIND_ERR :ErrorCode = "DATA_NOT_FIND";