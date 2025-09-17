use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, types::Json};

use super::error::Result;
use super::utils::Offset;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
        pub city: Box<str>,
        pub street: Box<str>,
        pub house: i16,
        pub housing: Option<i16>,
        pub building: Option<i16>,
        pub metro: Option<Box<str>>
}

#[derive(Debug, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
pub enum Status {
        Approved,
        Rejected,
        OnReview
}

#[derive(Debug, Deserialize)]
pub struct NewEvent<'a> {
        pub organizer_id: i64,
        pub title: &'a str,
        pub description: &'a str,
        pub date: time::PrimitiveDateTime,
        pub cost: i32,
        pub address: Json<Address>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventModel {
        pub id: i64,
        pub organizer_id: i64,
        pub title: Box<str>,
        pub description: Box<str>,
        pub date: time::PrimitiveDateTime,
        pub cost: i32,
        pub address: Json<Address>,
        pub status: Status,
        pub created_at: time::PrimitiveDateTime,
        pub updated_at: time::PrimitiveDateTime
}

pub async fn insert(pool: Pool<Postgres>, event: NewEvent<'_>) -> Result<()> {
        sqlx::query!(
                r#"
                        INSERT INTO "event" (organizer_id, title, description, date, cost, address)
                        VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                event.organizer_id,
                event.title,
                event.description,
                event.date,
                event.cost,
                event.address as _
        )
        .execute(&pool)
        .await?;

        Ok(())
}

pub async fn update_status(pool: Pool<Postgres>, id: i64, status: Status) -> Result<()> {
        sqlx::query!(
                r#"
                        UPDATE "event"
                        SET status = $1
                        WHERE id = $2
                "#,
                status as _,
                id
        )
        .execute(&pool)
        .await?;

        Ok(())
}

pub async fn get_by_id(pool: Pool<Postgres>, id: i64) -> Result<Option<EventModel>> {
        sqlx::query_as!(
                EventModel,
                r#"
                        SELECT id, organizer_id, title, description, date, cost, address AS "address: Json<Address>", status AS "status: Status", created_at, updated_at
                        FROM "event"
                        WHERE id = $1
                "#,
                id
        )
        .fetch_optional(&pool)
        .await
        .map_err(Into::into)
}

pub async fn get_all_by_status(pool: Pool<Postgres>, status: Status, offset: Offset) -> Result<Vec<EventModel>> {
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
        .fetch_all(&pool)
        .await
        .map_err(Into::into)
}

pub async fn delete_by_id(pool: Pool<Postgres>, id: i64) -> Result<()> {
        sqlx::query!(
                r#"
                        DELETE FROM "event"
                        WHERE id = $1
                "#,
                id
        )
        .execute(&pool)
        .await?;

        Ok(())
}