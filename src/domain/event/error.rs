#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum EventError {
        #[error("Status {0} does not exist")]
        Status(String),
        #[error("Order {0} does not exists")]
        Order(String),
        #[error("Filter {0} does not exists")]
        Filter(String)
}
