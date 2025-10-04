#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FavoriteError {
        #[error("Order {0} does not exists")]
        Order(String),
        #[error("Filter {0} does not exists")]
        Filter(String)
}
