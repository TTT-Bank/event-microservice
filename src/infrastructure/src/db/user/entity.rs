use domain::models::user::UserModel;
use sqlx::FromRow;

use time::PrimitiveDateTime;

#[derive(Debug, Clone, FromRow)]
pub struct UserEntity {
        pub id: i64,
        pub login: String,
        pub password_hash: String,
        pub role: String,
        pub created_at: PrimitiveDateTime,
        pub updated_at: PrimitiveDateTime
}

impl From<UserEntity> for UserModel {
        fn from(value: UserEntity) -> Self {
                UserModel {
                        id: value.id as u64,
                        login: value.login,
                        role: value.role.parse().unwrap(),
                        created_at: value.created_at,
                        updated_at: value.updated_at
                }
        }
}