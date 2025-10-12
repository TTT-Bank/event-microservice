use argon2::{
        Argon2, PasswordVerifier, password_hash::{
                PasswordHash, PasswordHasher, SaltString, rand_core::OsRng
        }
};
use domain::models::{user::{UserCredentials, UserFilter, UserId, UserModel, UserOrder, UserUpdate}, utils::Offset};
use infrastructure::db::user::repository::UserRepository;

use crate::{Result, ServiceError};

pub struct UserService<T: UserRepository> {
        repository: T,
}

impl<T: UserRepository> UserService<T> {
        pub fn new(repository: T) -> Self {
                Self { repository }
        }

        pub async fn get(&self, id: UserId) -> Result<UserModel> {
                let res = self.repository
                        .get(id as i64)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("user".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn get_by_login(&self, credentials: UserCredentials) -> Result<UserModel> {
                let res = self.repository
                        .get_by_login(&credentials.login)
                        .await;

                match res {
                        Ok(res) => {
                                match res {
                                        Some(user) => {
                                                let password_hash = PasswordHash::new(&user.password_hash)?;
                                                Argon2::default().verify_password(credentials.password.as_bytes(), &password_hash)?;

                                                Ok(user.into())
                                        },
                                        None => Err(ServiceError::NotFound("user".to_string(), credentials.login))
                                }
                        },
                        Err(err) => Err(err.into())
                }
        }

        pub async fn create(&self, credentials: UserCredentials) -> Result<UserModel> {
                let salt = SaltString::generate(&mut OsRng);

                let password_hash =
                        Argon2::default()
                                .hash_password(credentials.password.as_bytes(), &salt)?
                                .to_string();

                let res = self.repository
                        .create(&credentials.login, &password_hash)
                        .await;

                match res {
                        Ok(res) => Ok(res.into()),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn list(&self, offset: Offset, filters: &[UserFilter], order_by: &[UserOrder]) -> Result<Vec<UserModel>> {
                let res = self.repository
                        .list(offset, filters, order_by)
                        .await;

                match res {
                        Ok(res) => Ok(res.into_iter().map(Into::into).collect()),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn update(&self, id: UserId, changes: UserUpdate) -> Result<UserModel> {
                let res = self.repository
                        .update(id as i64, changes)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("user".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }

        pub async fn delete(&self, id: UserId) -> Result<UserModel> {
                let res = self.repository
                        .delete(id as i64)
                        .await;

                match res {
                        Ok(res) =>
                                res.map(Into::into)
                                        .ok_or(ServiceError::NotFound("user".to_string(), id.to_string())),
                        Err(err) => Err(err.into())
                }
        }
}