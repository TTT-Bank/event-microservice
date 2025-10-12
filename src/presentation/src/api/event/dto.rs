use domain::models::event::{NewEvent, EventModel, EventStatus};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::{OffsetDateTime, PrimitiveDateTime};
use utoipa::ToSchema;

use super::super::{Result, HandlerError};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "EventStatus")]
pub struct EventStatusDto {
        pub status: String,
}

impl TryFrom<EventStatusDto> for EventStatus {
        type Error = HandlerError;

        fn try_from(value: EventStatusDto) -> Result<Self> {
                value.status.parse().map_err(|err| HandlerError::Parse(err))
        }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "NewEvent")]
pub struct NewEventDto {
        pub organizer_id: i64,
        pub title: String,
        pub description: String,
        #[serde_as(as = "TimestampSeconds")]
        pub date: OffsetDateTime,
        pub cost: i32,
        pub address: String
}

impl TryFrom<NewEventDto> for NewEvent {
        type Error = HandlerError;

        fn try_from(value: NewEventDto) -> Result<Self> {
                Ok(NewEvent {
                        organizer_id: value.organizer_id.try_into()?,
                        title: value.title,
                        description: value.description,
                        date: value.date,
                        cost: value.cost.try_into()?,
                        address: value.address
                })
        }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "Event")]
pub struct EventDto {
        pub id: i64,
        pub organizer_id: i64,
        pub title: String,
        pub description: String,
        #[serde_as(as = "TimestampSeconds")]
        pub date: OffsetDateTime,
        pub cost: i32,
        pub address: String,
        pub status: String,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: PrimitiveDateTime
}

impl From<EventModel> for EventDto {
        fn from(value: EventModel) -> Self {
                Self {
                        id: value.id as i64,
                        organizer_id: value.organizer_id as i64,
                        title: value.title,
                        description: value.description,
                        date: value.date,
                        cost: value.cost as i32,
                        address: value.address,
                        status: value.status.to_string(),
                        created_at: value.created_at,
                        updated_at: value.updated_at
                }
        }
}
