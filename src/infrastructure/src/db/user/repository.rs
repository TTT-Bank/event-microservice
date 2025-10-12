use async_trait::async_trait;
use domain::models::{user::{UserUpdate, UserFilter, UserOrder}, utils::Offset};

use super::entity::UserEntity;
use crate::Result;

#[async_trait]
pub trait UserRepository {
        async fn get(&self, id: i64) -> Result<Option<UserEntity>>;
        async fn list(&self, offset: Offset, filters: &[UserFilter], order_by: &[UserOrder]) -> Result<Vec<UserEntity>>;
        async fn create(&self, login: &str, password_hash: &str) -> Result<UserEntity>;
        async fn update(&self, id: i64, changes: UserUpdate) -> Result<Option<UserEntity>>;
        async fn delete(&self, id: i64) -> Result<Option<UserEntity>>;

        async fn get_by_login(&self, login: &str) -> Result<Option<UserEntity>>;
}