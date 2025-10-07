use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use utoipa::ToSchema;

use super::{error::EventError, super::{error::DomainError, utils::{FilterOp, OrderOp}, user::model::UserId}};

pub type EventId = i64;

#[serde_as]
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, ToSchema)]
pub struct NewEvent {
        pub organizer_id: UserId,
        pub title: String,
        pub description: String,
        #[serde_as(as = "TimestampSeconds")]
        pub date: time::PrimitiveDateTime,
        pub cost: u32,
        pub address: String
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "status", rename_all = "PascalCase")]
pub enum EventStatus {
        Approved,
        Rejected,
        #[default]
        OnReview
}

impl FromStr for EventStatus {
        type Err = EventError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.trim() {
                        "Approved" => Ok(Self::Approved),
                        "Rejected" => Ok(Self::Rejected),
                        "OnReview" => Ok(Self::OnReview),
                        _ => Err(EventError::Status(s.to_string()))
                }
        }
}

impl Display for EventStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = match self {
                        Self::Approved => "Approved",
                        Self::Rejected => "Rejected",
                        Self::OnReview => "OnReview"
                };

                f.write_str(string)
        }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Clone, Hash, sqlx::FromRow, Deserialize, Serialize, ToSchema, sqlx::Type)]
pub struct EventModel {
        pub id: EventId,
        pub organizer_id: UserId,
        pub title: String,
        pub description: String,
        #[serde_as(as = "TimestampSeconds")]
        pub date: time::PrimitiveDateTime,
        pub cost: i32,
        pub address: String,
        pub status: EventStatus,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: time::PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: time::PrimitiveDateTime
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, ToSchema)]
pub enum EventUpdate {
        Status(EventStatus),
        Title(String),
        Description(String),
        Cost(u32),
        Address(String)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, ToSchema)]
pub enum EventFilter {
        OrganizerId(FilterOp<UserId>),
        Status(FilterOp<EventStatus>),
        Title(FilterOp<String>),
        Cost(FilterOp<u32>)
}

impl FromStr for EventFilter {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.trim();
                if let Some((field, op)) = s.split_once(' ') {
                        match field.trim() {
                                "organizer_id" => Ok(Self::OrganizerId(op.parse()?)),
                                "status" => Ok(Self::Status(op.parse()?)),
                                "title" => Ok(Self::Title(op.parse()?)),
                                "cost" => Ok(Self::Cost(op.parse()?)),
                                _ => Err(EventError::Filter(s.to_string()).into())
                        }
                } else {
                        Err(DomainError::Filter(s.to_string()))
                }
        }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, ToSchema)]
pub enum EventOrder {
        Id(OrderOp),
        OrganizerId(OrderOp),
        Cost(OrderOp),
        Status(OrderOp),
        CreatedAt(OrderOp),
        UpdatedAt(OrderOp)
}

impl FromStr for EventOrder {
        type Err = DomainError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some((field, op)) = s.split_once(' ') {
                        match field.trim() {
                                "id" => Ok(Self::Id(op.parse()?)),
                                "organizer_id" => Ok(Self::OrganizerId(op.parse()?)),
                                "cost" => Ok(Self::Cost(op.parse()?)),
                                "status" => Ok(Self::Status(op.parse()?)),
                                "created_at" => Ok(Self::CreatedAt(op.parse()?)),
                                "updated_at" => Ok(Self::UpdatedAt(op.parse()?)),
                                _ => Err(EventError::Order(s.to_string()).into())
                        }
                } else {
                        match s.trim() {
                                "id" => Ok(Self::Id(OrderOp::default())),
                                "organizer_id" => Ok(Self::OrganizerId(OrderOp::default())),
                                "cost" => Ok(Self::Cost(OrderOp::default())),
                                "status" => Ok(Self::Status(OrderOp::default())),
                                "created_at" => Ok(Self::CreatedAt(OrderOp::default())),
                                "updated_at" => Ok(Self::UpdatedAt(OrderOp::default())),
                                _ => Err(EventError::Order(s.to_string()).into())
                        }
                }
        }
}

impl Default for EventOrder {
        fn default() -> Self {
                Self::Id(OrderOp::default())
        }
}