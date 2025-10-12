use domain::models::{favorite::{FavoriteEventModel, FavoriteModel}};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::PrimitiveDateTime;
use utoipa::ToSchema;

use crate::api::event::dto::EventDto;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "Favorite")]
pub struct FavoriteDto {
        pub user_id: i64,
        pub event_id: i64,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: PrimitiveDateTime
}

impl From<FavoriteModel> for FavoriteDto {
        fn from(value: FavoriteModel) -> Self {
                Self {
                        user_id: value.user_id as i64,
                        event_id: value.event_id as i64,
                        created_at: value.created_at,
                        updated_at: value.updated_at
                }
        }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "FavoriteEvent")]
pub struct FavoriteEventDto {
        pub event: EventDto,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: PrimitiveDateTime
}

impl From<FavoriteEventModel> for FavoriteEventDto {
        fn from(value: FavoriteEventModel) -> Self {
                Self {
                        event: value.event.into(),
                        created_at: value.created_at,
                        updated_at: value.updated_at
                }
        }
}
