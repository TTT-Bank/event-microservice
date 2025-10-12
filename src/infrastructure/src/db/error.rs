#[derive(Debug, thiserror::Error)]
pub enum DbError {
        #[error("error occurred while inserting or updating {table} foreign field {column}")]
        ForeignKeyViolation{
                table: String,
                column: String,
                source: Box<dyn std::error::Error>
        },
        #[error("error occurred while inserting or updating {table} unique field {column}")]
        UniqueViolation{
                table: String,
                column: String,
                source: Box<dyn std::error::Error>
        },
        #[error("unknown error occurred while working with db")]
        Other(#[source] Box<dyn std::error::Error>),
}

impl From<sqlx::error::Error> for DbError {
        fn from(value: sqlx::error::Error) -> Self {
                use sqlx::error::Error;
                match value {
                        Error::Database(err) => {
                                use sqlx::postgres::PgDatabaseError;
                                let pg_err = err.downcast::<PgDatabaseError>();
                                match pg_err.code() {
                                        "23503" => Self::ForeignKeyViolation {
                                                table: pg_err.table().unwrap().to_string(),
                                                column: pg_err.column().unwrap().to_string(),
                                                source: pg_err
                                        },
                                        "23505" => Self::UniqueViolation {
                                                table: pg_err.table().unwrap().to_string(),
                                                column: pg_err.column().unwrap().to_string(),
                                                source: pg_err
                                        },
                                        _ => Self::Other(pg_err)
                                }
                        }
                        _ => Self::Other(Box::new(value))
                }
        }
}

impl From<sqlx::migrate::MigrateError> for DbError {
        fn from(value: sqlx::migrate::MigrateError) -> Self {
                Self::Other(Box::new(value))
        }
}

pub type Result<T> = std::result::Result<T, DbError>;