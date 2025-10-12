#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DomainError {
        #[error("Couldn't parse {0}")]
        Parse(String),
        #[error("OrderOp {0} does not exist")]
        OrderOp(String),
        #[error("Wrong order style {0}")]
        Order(String),
        #[error("FilterOp {0} does not exist")]
        FilterOp(String),
        #[error("Wrong filter style {0}")]
        Filter(String),
        #[error("{0}")]
        Infallible(#[from] std::convert::Infallible),
        #[error("{0}")]
        ParseInt(#[from] std::num::ParseIntError)
}
