use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use utoipa::ToSchema;

use crate::domain::{error::DomainError, event::model::{EventId, EventModel}, favorite::error::FavoriteError, user::model::UserId, utils::{FilterOp, OrderOp}};

#[serde_as]
#[derive(Debug, PartialEq, Eq, Clone, Hash, sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct FavoriteModel {
        pub user_id: EventId,
        pub event_id: UserId,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: time::PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: time::PrimitiveDateTime
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Clone, Hash, sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct FavoriteEventModel {
        pub event: EventModel,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: time::PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: time::PrimitiveDateTime
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, ToSchema)]
pub enum FavoriteFilter {
        EventId(FilterOp<EventId>)
}

impl FromStr for FavoriteFilter {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.trim();
                if let Some((field, op)) = s.split_once(' ') {
                        match field.trim() {
                                "event_id" => Ok(Self::EventId(op.parse()?)),
                                _ => Err(FavoriteError::Filter(s.to_string()).into())
                        }
                } else {
                        Err(DomainError::Filter(s.to_string()))
                }
        }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, ToSchema)]
pub enum FavoriteOrder {
        EventId(OrderOp),
        CreatedAt(OrderOp),
        UpdatedAt(OrderOp)
}

impl FromStr for FavoriteOrder {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some((field, op)) = s.split_once(' ') {
                        match field.trim() {
                                "event_id" => Ok(Self::EventId(op.parse()?)),
                                "created_at" => Ok(Self::CreatedAt(op.parse()?)),
                                "updated_at" => Ok(Self::UpdatedAt(op.parse()?)),
                                _ => Err(FavoriteError::Order(s.to_string()).into())
                        }
                } else {
                        match s.trim() {
                                "event_id" => Ok(Self::EventId(OrderOp::default())),
                                "created_at" => Ok(Self::CreatedAt(OrderOp::default())),
                                "updated_at" => Ok(Self::UpdatedAt(OrderOp::default())),
                                _ => Err(FavoriteError::Order(s.to_string()).into())
                        }
                }
        }
}

impl Default for FavoriteOrder {
        fn default() -> Self {
                Self::EventId(OrderOp::default())
        }
}
