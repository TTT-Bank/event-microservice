use async_trait::async_trait;

use crate::Result;
use super::entity::RefreshTokenEntity;

#[async_trait]
pub trait RefreshRepository {
        async fn create(&self, id: i64, token: &str) -> Result<RefreshTokenEntity>;
        async fn update(&self, id: i64, old: &str, token: &str) -> Result<Option<RefreshTokenEntity>>;
        async fn delete(&self, id: i64) -> Result<Option<RefreshTokenEntity>>;
}