use super::{event::error::EventError, user::error::UserError, favorite::error::FavoriteError};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DomainError {
        #[error("User error: {0}")]
        User(#[from] UserError),
        #[error("Event error: {0}")]
        Event(#[from] EventError),
        #[error("Favorite error: {0}")]
        Favorite(#[from] FavoriteError),
        #[error("OrderOp {0} does not exist")]
        OrderOp(String),
        #[error("FilterOp {0} does not exist")]
        FilterOp(String),
        #[error("Wrong filter style {0}")]
        Filter(String),
        #[error("{0}")]
        Infallible(#[from] std::convert::Infallible),
        #[error("{0}")]
        ParseInt(#[from] std::num::ParseIntError)
}
