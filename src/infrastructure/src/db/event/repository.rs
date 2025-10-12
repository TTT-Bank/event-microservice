use async_trait::async_trait;
use domain::models::{event::{EventFilter, EventOrder, EventUpdate}, utils::Offset};
use time::OffsetDateTime;

use super::entity::EventEntity;
use crate::Result;

#[async_trait]
pub trait EventRepository {
        async fn get(&self, id: i64) -> Result<Option<EventEntity>>;
        async fn list(&self, offset: Offset, filters: &[EventFilter], order_by: &[EventOrder]) -> Result<Vec<EventEntity>>;
        async fn create(&self,
                organizer_id: i64, title: &str, description: &str,
                date: OffsetDateTime, cost: i32, address: &str
        ) -> Result<EventEntity>;
        async fn update(&self, id: i64, changes: EventUpdate) -> Result<Option<EventEntity>>;
        async fn delete(&self, id: i64) -> Result<Option<EventEntity>>;
}