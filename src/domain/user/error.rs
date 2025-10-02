#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum UserError {
        #[error("Role {0} does not exist")]
        Role(String),
        #[error("Wrong order style {0}")]
        Order(String),
        #[error("Filter {0} does not exists")]
        Filter(String)
}
