use super::user::error::UserError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DomainError {
        #[error("User error: {0}")]
        User(#[from] UserError),
        #[error("OrderOp {0} does not exist")]
        OrderOp(String),
        #[error("FilterOp {0} does not exist")]
        FilterOp(String),
        #[error("Wrong filter style {0}")]
        Filter(String),
        #[error("{0}")]
        Infallible(#[from] std::convert::Infallible)
}
