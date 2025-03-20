use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator_derive::Validate;

/// Role add Data DTO
#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
pub struct AddReq {
    /// Role name
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    /// Role code
    #[validate(length(min = 1, max = 20))]
    pub code: String,
}
