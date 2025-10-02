use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use sqlx::types::Json;
use utoipa::ToSchema;
use serde_with::{serde_as, TimestampSeconds};
use crate::domain::user::model::UserId;

use super::error::Result;
use super::utils::Offset;

pub type EventId = i64;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Address {
        pub city: Box<str>,
        pub street: Box<str>,
        pub house: i16,
        #[serde(default)]
        pub housing: Option<i16>,
        #[serde(default)]
        pub building: Option<i16>,
        #[serde(default)]
        pub metro: Option<Box<str>>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "status", rename_all = "PascalCase")]
pub enum Status {
        Approved,
        Rejected,
        OnReview
}

#[serde_as]
#[derive(Debug, Deserialize, ToSchema)]
pub struct NewEvent {
        pub organizer_id: UserId,
        pub title: String,
        pub description: String,
        #[serde_as(as = "TimestampSeconds")]
        pub date: time::PrimitiveDateTime,
        pub cost: i32,
        #[schema(value_type = Object)]
        pub address: Json<Address>
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
pub struct EventModel {
        pub id: EventId,
        pub organizer_id: UserId,
        pub title: Box<str>,
        pub description: Box<str>,
        #[serde_as(as = "TimestampSeconds")]
        pub date: time::PrimitiveDateTime,
        pub cost: i32,
        #[schema(value_type = Object)]
        pub address: Json<Address>,
        pub status: Status,
        #[serde_as(as = "TimestampSeconds")]
        pub created_at: time::PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        pub updated_at: time::PrimitiveDateTime
}

pub async fn insert(conn: &mut PgConnection, event: NewEvent) -> Result<EventModel> {
        sqlx::query_as!(
                EventModel,
                r#"
                        INSERT INTO "event" (organizer_id, title, description, date, cost, address)
                        VALUES ($1, $2, $3, $4, $5, $6)
                        RETURNING id, organizer_id, title, description, date, cost, address AS "address: Json<Address>", status AS "status: Status", created_at, updated_at
                "#,
                event.organizer_id,
                event.title,
                event.description,
                event.date,
                event.cost,
                event.address as _
        )
        .fetch_one(conn)
        .await
}

pub async fn update_status(conn: &mut PgConnection, id: EventId, status: Status) -> Result<Option<EventModel>> {
        sqlx::query_as!(
                EventModel,
                r#"
                        UPDATE "event"
                        SET status = $1
                        WHERE id = $2
                        RETURNING id, organizer_id, title, description, date, cost, address AS "address: Json<Address>", status AS "status: Status", created_at, updated_at
                "#,
                status as _,
                id
        )
        .fetch_optional(conn)
        .await
}

pub async fn get_by_id(conn: &mut PgConnection, id: EventId) -> Result<Option<EventModel>> {
        sqlx::query_as!(
                EventModel,
                r#"
                        SELECT id, organizer_id, title, description, date, cost, address AS "address: Json<Address>", status AS "status: Status", created_at, updated_at
                        FROM "event"
                        WHERE id = $1
                "#,
                id
        )
        .fetch_optional(conn)
        .await
}

pub async fn get_all_by_status(conn: &mut PgConnection, status: Status, offset: Offset) -> Result<Vec<EventModel>> {
        sqlx::query_as!(
                EventModel,
                r#"
                        SELECT id, organizer_id, title, description, date, cost, address AS "address: Json<Address>", status AS "status: Status", created_at, updated_at
                        FROM "event"
                        WHERE status = $1
                        LIMIT $2
                        OFFSET ($2::INT * ($3::INT - 1))
                "#,
                status as _,
                offset.limit,
                offset.page
        )
        .fetch_all(conn)
        .await
}

pub async fn delete_by_id(conn: &mut PgConnection, id: EventId) -> Result<Option<EventModel>> {
        sqlx::query_as!(
                EventModel,
                r#"
                        DELETE FROM "event"
                        WHERE id = $1
                        RETURNING id, organizer_id, title, description, date, cost, address AS "address: Json<Address>", status AS "status: Status", created_at, updated_at
                "#,
                id
        )
        .fetch_optional(conn)
        .await
}