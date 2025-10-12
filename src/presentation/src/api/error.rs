#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
        #[error("{0}")]
        Service(#[from] use_case::error::ServiceError),
        #[error("field should be gte than 0")]
        LsThanZero(#[from] std::num::TryFromIntError),
        #[error("max login len had been exceeded")]
        MaxLoginLen,
        #[error("min login len had not been reached")]
        MinLoginLen,
        #[error("max password len had been exceeded")]
        MaxPasswordLen,
        #[error("min password len had not been reached")]
        MinPasswordLen,
        #[error("id accesses does not match")]
        IdMismatch,
        #[error("{0}")]
        Parse(#[from] domain::error::DomainError)
}

impl actix_web::ResponseError for HandlerError {}

pub type Result<T> = core::result::Result<T, HandlerError>;