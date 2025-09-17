#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BaseError {
        #[error("Io error: {0}")]
        Io(#[from] std::io::Error),
        #[error("ActixWeb error: {0}")]
        ActixWeb(#[from] actix_web::error::Error),
        #[error("Dotenvy error: {0}")]
        Dotenvy(#[from] dotenvy::Error),
        #[error("Db error: {0}")]
        Db(#[from] crate::db::error::DbError)
}

pub type Result<T> = core::result::Result<T, BaseError>;