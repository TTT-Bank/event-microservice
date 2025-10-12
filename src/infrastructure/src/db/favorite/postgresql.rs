use async_trait::async_trait;
use sqlx::{Pool, Postgres, QueryBuilder};
use domain::models::{favorite::{FavoriteFilter, FavoriteOrder}, utils::Offset};

use super::entity::{FavoriteEntity, FavoriteEventProjection};
use super::repository::FavoriteRepository;
use crate::Result;

pub struct PgFavoriteRepository {
        pool: Pool<Postgres>
}

impl PgFavoriteRepository {
        pub fn new(pool: Pool<Postgres>) -> Self {
                Self { pool }
        }
}

#[async_trait]
impl FavoriteRepository for PgFavoriteRepository {
        async fn get(&self, user_id: i64, event_id: i64) -> Result<Option<FavoriteEventProjection>> {
                sqlx::query_as(
                r#"
                        SELECT
                        e.id AS event_id,
                        e.organizer_id AS event_organizer_id,
                        e.title AS event_title,
                        e.description AS event_description,
                        e.date AS event_date,
                        e.cost AS event_cost,
                        e.address AS event_address,
                        e.status AS event_status,
                        e.created_at AS event_created_at,
                        e.updated_at AS event_updated_at,
                        f.created_at AS favorite_created_at,
                        f.updated_at AS favorite_updated_at
                        FROM "favorite" f
                        JOIN "event" e ON f.event_id = e.id
                        WHERE f.user_id = $1
                        AND f.event_id = $2
                "#
                )
                .bind(user_id)
                .bind(event_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(Into::into)
        }

        async fn list(&self, user_id: i64, offset: Offset, filters: &[FavoriteFilter], order_by: &[FavoriteOrder]) -> Result<Vec<FavoriteEventProjection>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"
                        SELECT
                        e.id AS event_id,
                        e.organizer_id AS event_organizer_id,
                        e.title AS event_title,
                        e.description AS event_description,
                        e.date AS event_date,
                        e.cost AS event_cost,
                        e.address AS event_address,
                        e.status AS event_status,
                        e.created_at AS event_created_at,
                        e.updated_at AS event_updated_at,
                        f.created_at AS favorite_created_at,
                        f.updated_at AS favorite_updated_at
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
                                        FavoriteFilter::EventId(op) => separated.push("event_id ").push_unseparated(op.operation() + " ").push_bind_unseparated(*op.value() as i64)
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

                query_builder.push(" LIMIT ").push_bind(offset.limit as i32);
                query_builder.push(" OFFSET (").push_bind(offset.limit as i32).push(" * (").push_bind(offset.page as i32).push(" - 1))");

                query_builder
                        .build_query_as()
                        .fetch_all(&self.pool)
                        .await
                        .map_err(Into::into)
        }

        async fn create(&self, user_id: i64, event_id: i64) -> Result<FavoriteEntity> {
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
                .map_err(Into::into)
        }

        async fn delete(&self, user_id: i64, event_id: i64) -> Result<Option<FavoriteEntity>>  {
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
                .map_err(Into::into)
        }
}
