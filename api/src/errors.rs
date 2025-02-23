use actix_web::body::MessageBody;
use actix_web::{HttpResponse, Responder, ResponseError};
use common::Rsp;
use std::fmt::Display;
use thiserror::Error;
use validator::ValidationErrors;

/// 业务逻中出现的错误
#[derive(Debug, Error)]
pub enum BizError {
    #[error("")]
    ValidatorError(ValidationErrors),

    #[error("")]
    QueryError,

    // Unknown(Rsp<None>)
}

pub type ApiRsp = Result<impl Responder, BizError>;
impl ResponseError for BizError {
    fn error_response(&self) -> HttpResponse {
        let rsp = match self {
            BizError::ValidatorError(e) => Rsp::error_data("".to_string(), "".to_string(), ValidationErrorResponse::from(e)),
            BizError::QueryError => Rsp::error("".to_string(), "".to_string()),
            // BizError::Unknown(e) => e
        };
        HttpResponse::Ok().body(rsp)
    }
}
