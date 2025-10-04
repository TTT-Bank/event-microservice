use super::model::*;
use super::super::utils::Offset;
use super::super::user::model::UserId;
use super::super::event::model::{EventId, EventModel};

pub trait FavoriteRepository<Db> {
        type Error: std::error::Error;

        fn init(pool: Db) -> Self;

        async fn get(&self, user_id: UserId, event_id: EventId) -> Result<Option<EventModel>, Self::Error>;
        async fn list(&self, user_id: UserId, offset: Offset, filters: &[FavoriteFilter], order_by: &[FavoriteOrder]) -> Result<Vec<EventModel>, Self::Error>;
        async fn create(&self, user_id: UserId, event_id: EventId) -> Result<FavoriteModel, Self::Error>;
        async fn delete(&self, user_id: UserId, event_id: EventId) -> Result<Option<FavoriteModel>, Self::Error>;
}