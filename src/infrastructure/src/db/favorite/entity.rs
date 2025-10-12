use domain::models::{event::EventModel, favorite::{FavoriteEventModel, FavoriteModel}};
use sqlx::FromRow;

use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, Clone, FromRow)]
pub struct FavoriteEntity {
        pub user_id: i64,
        pub event_id: i64,
        pub created_at: PrimitiveDateTime,
        pub updated_at: PrimitiveDateTime
}

impl From<FavoriteEntity> for FavoriteModel {
        fn from(value: FavoriteEntity) -> Self {
                FavoriteModel {
                        user_id: value.user_id as u64,
                        event_id: value.event_id as u64,
                        created_at: value.created_at,
                        updated_at: value.updated_at
                }
        }
}

#[derive(Debug, Clone, FromRow)]
pub struct FavoriteEventProjection {
        pub event_id: i64,
        pub event_organizer_id: i64,
        pub event_title: String,
        pub event_description: String,
        pub event_date: OffsetDateTime,
        pub event_cost: i32,
        pub event_address: String,
        pub event_status: String,
        pub event_created_at: PrimitiveDateTime,
        pub event_updated_at: PrimitiveDateTime,
        pub favorite_created_at: PrimitiveDateTime,
        pub favorite_updated_at: PrimitiveDateTime
}

impl From<FavoriteEventProjection> for FavoriteEventModel {
        fn from(value: FavoriteEventProjection) -> Self {
                let event = EventModel {
                        id: value.event_id as u64,
                        organizer_id: value.event_organizer_id as u64,
                        title: value.event_title,
                        description: value.event_description,
                        date: value.event_date,
                        cost: value.event_cost as u32,
                        address: value.event_address,
                        status: value.event_status.parse().unwrap(),
                        created_at: value.event_created_at,
                        updated_at: value.event_updated_at
                };

                FavoriteEventModel {
                        event: event.into(),
                        created_at: value.favorite_created_at,
                        updated_at: value.favorite_updated_at
                }
        }
}