use sqlx::{Pool, Postgres, QueryBuilder};
use crate::domain::favorite::model::*;
use crate::domain::favorite::repository::FavoriteRepository;
use crate::domain::utils::Offset;

use crate::domain::event::model::{EventId, EventModel};
use crate::domain::user::model::UserId;

use super::error::Result;

pub struct PgFavoriteRepository {
        pool: Pool<Postgres>
}

impl FavoriteRepository<Pool<Postgres>> for PgFavoriteRepository {
    type Error = sqlx::Error;

        fn init(pool: Pool<Postgres>) -> Self {
                Self { pool }
        }

        async fn get(&self, user_id: UserId, event_id: EventId) -> Result<Option<EventModel>> {
                sqlx::query_as(
                r#"
                        SELECT
                        e.id,
                        e.organizer_id,
                        e.title,
                        e.description,
                        e.date,
                        e.cost,
                        e.address,
                        e.status,
                        e.created_at,
                        e.updated_at
                        FROM "favorite" f
                        RIGHT JOIN "event" e ON f.event_id = e.id
                        WHERE f.user_id = $1
                        AND f.event_id = $2
                "#
                )
                .bind(user_id)
                .bind(event_id)
                .fetch_optional(&self.pool)
                .await
        }

        async fn list(&self, user_id: UserId, offset: Offset, filters: &[FavoriteFilter], order_by: &[FavoriteOrder]) -> Result<Vec<EventModel>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"
                        SELECT
                        e.id,
                        e.organizer_id,
                        e.title,
                        e.description,
                        e.date,
                        e.cost,
                        e.address,
                        e.status,
                        e.created_at,
                        e.updated_at
                        FROM "favorite" f
                        RIGHT JOIN "event" e ON f.event_id = e.id
                        WHERE f.user_id = "#
                );
                query_builder.push_bind(user_id);

                if !filters.is_empty() {
                        let mut separated = query_builder.separated(" AND ");
                        separated.push_unseparated(" AND ");

                        for filter in filters {
                                match filter {
                                        FavoriteFilter::EventId(op) => separated.push("event_id ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value())
                                };
                        }
                }

                if !order_by.is_empty() {
                        let mut separated = query_builder.separated(", ");
                        separated.push_unseparated(" ORDER BY ");

                        for order in order_by {
                                match order {
                                        FavoriteOrder::EventId(op) => separated.push("event_id ").push_unseparated(op.to_string()),
                                        FavoriteOrder::CreatedAt(op) => separated.push("created_at ").push_unseparated(op.to_string()),
                                        FavoriteOrder::UpdatedAt(op) => separated.push("updated_at ").push_unseparated(op.to_string()),
                                };
                        }
                }

                query_builder.push(" LIMIT ").push_bind(offset.limit);
                query_builder.push(" OFFSET (").push_bind(offset.limit).push(" * (").push_bind(offset.page).push(" - 1))");

                query_builder.build_query_as().fetch_all(&self.pool).await
        }

        async fn create(&self, user_id: UserId, event_id: EventId) -> Result<FavoriteModel> {
                sqlx::query_as(
                        r#"
                        INSERT INTO "favorite" (user_id, event_id)
                        VALUES ($1, $2)
                        RETURNING user_id, event_id, created_at, updated_at
                        "#
                )
                .bind(user_id)
                .bind(event_id)
                .fetch_one(&self.pool)
                .await
        }

        async fn delete(&self, user_id: UserId, event_id: EventId) -> Result<Option<FavoriteModel>>  {
                sqlx::query_as(
                        r#"
                        DELETE FROM "favorite"
                        WHERE user_id = $1
                        AND event_id = $2
                        RETURNING user_id, event_id, created_at, updated_at
                        "#
                )
                .bind(user_id)
                .bind(event_id)
                .fetch_optional(&self.pool)
                .await
        }
}
