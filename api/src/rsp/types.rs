use serde::Serialize;
use utoipa::r#gen::serde_json::Value;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct FieldError {
    /// 字段名称
    pub field: String,
    /// 该字段的错误详情
    pub error_detail: Vec<ErrorDetail>,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorDetail {
    /// 错误类型代码 (如 "email", "length" 等)
    pub code: String,
    /// 人类可读的错误信息
    pub message: Option<String>,
    /// 额外参数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct PageResult<T: Serialize> {
    /// 总页数
    pub total_page: u64,
    /// 总记录数
    pub total_ele: u64,
    /// 当前页数据
    pub data: Vec<T>,
}
