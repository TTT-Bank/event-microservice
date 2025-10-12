use std::{fmt::Display, str::FromStr};

use time::PrimitiveDateTime;

use crate::error::DomainError;
use super::{event::{EventId, EventModel}, user::UserId, utils::{FilterOp, OrderOp}};

#[derive(Debug, Clone)]
pub struct FavoriteId {
        pub user_id: UserId,
        pub event_id: EventId
}

impl Display for FavoriteId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "user {} and event {}", self.user_id, self.event_id)
        }
}

#[derive(Debug, Clone)]
pub struct FavoriteModel {
        pub user_id: UserId,
        pub event_id: EventId,
        pub created_at: PrimitiveDateTime,
        pub updated_at: PrimitiveDateTime
}

#[derive(Debug, Clone)]
pub struct FavoriteEventModel {
        pub event: EventModel,
        pub created_at: PrimitiveDateTime,
        pub updated_at: PrimitiveDateTime
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
                                _ => Err(DomainError::Filter(s.to_string()))
                        }
                } else {
                        Err(DomainError::Filter(s.to_string()))
                }
        }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
                                _ => Err(DomainError::Order(s.to_string()))
                        }
                } else {
                        match s.trim() {
                                "event_id" => Ok(Self::EventId(OrderOp::default())),
                                "created_at" => Ok(Self::CreatedAt(OrderOp::default())),
                                "updated_at" => Ok(Self::UpdatedAt(OrderOp::default())),
                                _ => Err(DomainError::Order(s.to_string()))
                        }
                }
        }
}

impl Default for FavoriteOrder {
        fn default() -> Self {
                Self::EventId(OrderOp::default())
        }
}
