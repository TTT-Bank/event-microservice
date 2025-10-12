use domain::models::{token::{Claims, RefreshToken, TokenPair}, user::{UserId, UserRole}};
use infrastructure::db::refresh::repository::RefreshRepository;
use time::OffsetDateTime;

use crate::{Result, ServiceError};

pub struct RefreshService<T: RefreshRepository> {
        repository: T,
}

impl<T: RefreshRepository> RefreshService<T> {
        pub fn new(repository: T) -> Self {
                Self { repository }
        }

        pub async fn create(&self, user_id: UserId, role: UserRole) -> Result<TokenPair> {
                let current_timestamp = OffsetDateTime::now_utc();

                let refresh_token =
                        Claims::new_refresh(user_id, current_timestamp, role.clone()).encode()?;

                let res = self.repository
                        .create(user_id as i64, &refresh_token)
                        .await;

                match res {
                        Ok(_) => {
                                let access_token =
                                        Claims::new_access(user_id, current_timestamp, role).encode()?;

                                Ok(TokenPair {
                                        access_token,
                                        refresh_token
                                })
                        },
                        Err(err) => Err(err.into())
                }
        }

        pub async fn update(&self, old: RefreshToken) -> Result<TokenPair> {
                let mut old_claims = Claims::decode_from(&old)?;
                let user_id = old_claims.sub;
                let current_timestamp = OffsetDateTime::now_utc();

                old_claims.iat = current_timestamp;
                old_claims.nbf = current_timestamp;

                let new_refresh_token = old_claims.clone().encode()?;

                let res = self.repository
                        .update(user_id as i64, &old, &new_refresh_token)
                        .await;

                match res {
                        Ok(res) => {
                                let access_token =
                                        Claims::new_access(user_id, current_timestamp, old_claims.role).encode()?;

                                res.map(|_| {
                                        TokenPair {
                                                refresh_token: new_refresh_token,
                                                access_token
                                        }
                                }).ok_or(ServiceError::NotFound("refresh".to_string(), user_id.to_string()))
                        },
                        Err(err) => Err(err.into())
                }
        }

        pub async fn delete(&self, user_id: UserId) -> Result<()> {
                let res = self.repository
                        .delete(user_id as i64)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(|_| ())
                                        .ok_or(ServiceError::NotFound("refresh".to_string(), user_id.to_string())),
                        Err(err) => Err(err.into())
                }
        }
}