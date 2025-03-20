use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator_derive::Validate;

pub mod role;
pub mod user;

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct PageParam {
    /// 页码
    #[validate(range(min = 1))]
    #[schema(example = 1)]
    pub page: u64,
    /// 每页大小
    #[validate(range(min = 1))]
    #[schema(example = 10)]
    pub size: u64,
}
