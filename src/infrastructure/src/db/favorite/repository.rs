use async_trait::async_trait;
use domain::models::{favorite::{FavoriteFilter, FavoriteOrder}, utils::Offset};

use super::entity::{FavoriteEntity, FavoriteEventProjection};
use crate::Result;

#[async_trait]
pub trait FavoriteRepository {
        async fn get(&self, user_id: i64, event_id: i64) -> Result<Option<FavoriteEventProjection>>;
        async fn list(&self, user_id: i64, offset: Offset, filters: &[FavoriteFilter], order_by: &[FavoriteOrder]) -> Result<Vec<FavoriteEventProjection>>;
        async fn create(&self, user_id: i64, event_id: i64) -> Result<FavoriteEntity>;
        async fn delete(&self, user_id: i64, event_id: i64) -> Result<Option<FavoriteEntity>>;
}