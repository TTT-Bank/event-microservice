use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use serde_with::{serde_as, TimestampSeconds};
use crate::db::event::EventId;
use crate::domain::user::model::UserId;

use super::event::EventModel;
use super::utils::Offset;
use super::error::Result;

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
pub struct FavoriteModel {
        user_id: EventId,
        event_id: UserId,
        #[serde_as(as = "TimestampSeconds")]
        created_at: time::PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        updated_at: time::PrimitiveDateTime
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct FavoriteEvent {
        event: EventModel,
        #[serde_as(as = "TimestampSeconds")]
        created_at: time::PrimitiveDateTime,
        #[serde_as(as = "TimestampSeconds")]
        updated_at: time::PrimitiveDateTime
}

pub async fn insert(conn: &mut PgConnection, event_id: EventId, user_id: UserId) -> Result<FavoriteModel> {
        sqlx::query_as!(
                FavoriteModel,
                r#"
                INSERT INTO "favorite" (user_id, event_id)
                VALUES ($1, $2)
                RETURNING user_id, event_id, created_at, updated_at
                "#,
                user_id,
                event_id
        )
        .fetch_one(conn)
        .await
}

pub async fn get_all_by_user(conn: &mut PgConnection, user_id: UserId, offset: Offset) -> Result<Vec<FavoriteEvent>> {
        sqlx::query_as!(
                FavoriteEvent,
                r#"
                SELECT
                        ("event".id,
                        "event".organizer_id,
                        "event".title,
                        "event".description,
                        "event".date,
                        "event".cost,
                        "event".address,
                        "event".status,
                        "event".created_at,
                        "event".updated_at) AS "event!: EventModel",
                        "favorite".created_at,
                        "favorite".updated_at
                FROM "favorite"
                JOIN "event" ON "favorite".event_id = "event".id
                WHERE "favorite".user_id = $1
                LIMIT $2
                OFFSET ($2::INT * ($3::INT - 1))
                "#,
                user_id, offset.limit, offset.page
        )
        .fetch_all(conn)
        .await
}

pub async fn delete(conn: &mut PgConnection, user_id: UserId, event_id: EventId) -> Result<Option<FavoriteModel>> {
        sqlx::query_as!(
                FavoriteModel,
                r#"
                DELETE FROM "favorite"
                WHERE user_id = $1
                AND event_id = $2
                RETURNING user_id, event_id, created_at, updated_at
                "#,
                user_id,
                event_id
        )
        .fetch_optional(conn)
        .await
}