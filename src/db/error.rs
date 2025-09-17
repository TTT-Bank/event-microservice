#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DbError {
        #[error("{0}")]
        Sqlx(#[from] sqlx::Error),
        #[error("{0}")]
        Migrate(#[from] sqlx::migrate::MigrateError)
}

pub type Result<T> = core::result::Result<T, DbError>;