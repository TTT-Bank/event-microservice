use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HandlerError {
        #[error("{0}")]
        Sqlx(#[source] sqlx::Error),
        #[error("{0:?}")]
        Db(sqlx::error::ErrorKind),
        #[error("{0}")]
        ActixWeb(#[from] actix_web::error::Error),
        #[error("{0}")]
        SerdeJson(#[from] serde_json::Error),
        #[error("{0}")]
        Argon2(argon2::password_hash::Error),
        #[error("{0}")]
        Domain(#[from] crate::domain::error::DomainError),
        #[error("{0}")]
        Auth(#[from] actix_web_httpauth::extractors::AuthenticationError<Bearer>),
        #[error("{0}")]
        Jwt(#[from] jsonwebtoken::errors::Error),
}

impl From<sqlx::Error> for HandlerError {
        fn from(value: sqlx::Error) -> Self {
                match value.as_database_error() {
                        Some(err) => Self::Db(err.kind()),
                        None => Self::Sqlx(value)
                }
        }
}

impl From<argon2::password_hash::Error> for HandlerError {
        fn from(value: argon2::password_hash::Error) -> Self {
                Self::Argon2(value)
        }
}

impl actix_web::ResponseError for HandlerError {}

pub type Result<T> = core::result::Result<T, HandlerError>;