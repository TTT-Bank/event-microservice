use domain::models::user::{UserCredentials, UserModel, UserRole};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::PrimitiveDateTime;
use utoipa::ToSchema;

use super::super::{Result, HandlerError};

const MIN_LOGIN_LEN: usize = 6;
const MAX_LOGIN_LEN: usize = 30;
const MIN_PASSWORD_LEN: usize = 8;
const MAX_PASSWORD_LEN: usize = 100;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "UserRole")]
pub struct UserRoleDto {
        pub role: String,
}

impl TryFrom<UserRoleDto> for UserRole {
        type Error = HandlerError;

        fn try_from(value: UserRoleDto) -> Result<Self> {
                value.role.parse().map_err(|err| HandlerError::Parse(err))
        }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "UserCredentials")]
pub struct UserCredentialsDto {
        pub login: String,
        pub password: String
}

impl TryFrom<UserCredentialsDto> for UserCredentials {
        type Error = HandlerError;

        fn try_from(value: UserCredentialsDto) -> Result<Self> {
                if value.login.len() < MIN_LOGIN_LEN {
                        return Err(HandlerError::MinLoginLen);
                }

                if value.login.len() > MAX_LOGIN_LEN {
                        return Err(HandlerError::MaxLoginLen);
                }

                if value.password.len() < MIN_PASSWORD_LEN {
                        return Err(HandlerError::MinPasswordLen);
                }

                if value.password.len() > MAX_PASSWORD_LEN {
                        return Err(HandlerError::MaxPasswordLen);
                }

                Ok(UserCredentials {
                        login: value.login,
                        password: value.password
                })
        }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "User")]
pub struct UserDto {
        pub id: i64,
        pub login: String,
        pub role: String,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: PrimitiveDateTime
}

impl From<UserModel> for UserDto {
        fn from(value: UserModel) -> Self {
                Self {
                        id: value.id as i64,
                        login: value.login,
                        role: value.role.to_string(),
                        created_at: value.created_at,
                        updated_at: value.updated_at
                }
        }
}
