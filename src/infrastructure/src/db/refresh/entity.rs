use domain::models::token::RefreshToken;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct RefreshTokenEntity(String);

impl From<RefreshTokenEntity> for RefreshToken {
        fn from(value: RefreshTokenEntity) -> Self {
                value.0
        }
}