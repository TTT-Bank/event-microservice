use domain::models::event::EventModel;
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, Clone, FromRow)]
pub struct EventEntity {
        pub id: i64,
        pub organizer_id: i64,
        pub title: String,
        pub description: String,
        pub date: OffsetDateTime,
        pub cost: i32,
        pub address: String,
        pub status: String,
        pub created_at: PrimitiveDateTime,
        pub updated_at: PrimitiveDateTime
}

impl From<EventEntity> for EventModel {
        fn from(value: EventEntity) -> Self {
                EventModel {
                        id: value.id as u64,
                        organizer_id: value.organizer_id as u64,
                        title: value.title,
                        description: value.description,
                        date: value.date,
                        cost: value.cost as u32,
                        address: value.address,
                        status: value.status.parse().unwrap(),
                        created_at: value.created_at,
                        updated_at: value.updated_at
                }
        }
}
