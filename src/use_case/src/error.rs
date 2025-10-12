#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
        #[error("{0}")]
        Db(#[from] infrastructure::db::error::DbError),
        #[error("couldn't find {0} with field {0}")]
        NotFound(String, String),
        #[error("JWT have expired")]
        Expired(#[source] jsonwebtoken::errors::Error),
        #[error("{0}")]
        Other(#[source] Box<dyn std::error::Error>),
}

impl From<argon2::password_hash::Error> for ServiceError {
        fn from(value: argon2::password_hash::Error) -> Self {
                Self::Other(Box::new(value))
        }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
        fn from(value: jsonwebtoken::errors::Error) -> Self {
                use jsonwebtoken::errors::ErrorKind;
                match value.kind() {
                        ErrorKind::ExpiredSignature => Self::Expired(value),
                        _ => Self::Other(Box::new(value))
                }
        }
}

pub type Result<T> = std::result::Result<T, ServiceError>;