use super::model::{EventFilter, EventId, EventModel, EventOrder, EventUpdate, NewEvent};
use super::super::utils::Offset;

pub trait EventRepository<Db> {
        type Error: std::error::Error;

        fn init(pool: Db) -> Self;

        async fn get(&self, id: EventId) -> Result<Option<EventModel>, Self::Error>;
        async fn list(&self, offset: Offset, filters: &[EventFilter], order_by: &[EventOrder]) -> Result<Vec<EventModel>, Self::Error>;
        async fn create(&self, event: NewEvent) -> Result<EventModel, Self::Error>;
        async fn update(&self, id: EventId, changes: EventUpdate) -> Result<Option<EventModel>, Self::Error>;
        async fn delete(&self, id: EventId) -> Result<Option<EventModel>, Self::Error>;
}