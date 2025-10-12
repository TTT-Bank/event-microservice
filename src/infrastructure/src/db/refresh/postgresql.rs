use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::{entity::RefreshTokenEntity, repository::RefreshRepository};
use crate::Result;

pub struct PgRefreshRepository {
        pool: Pool<Postgres>
}

impl PgRefreshRepository {
        pub fn new(pool: Pool<Postgres>) -> Self {
                Self { pool }
        }
}

#[async_trait]
impl RefreshRepository for PgRefreshRepository {
        async fn create(&self, user_id: i64, token: &str) -> Result<RefreshTokenEntity> {
                sqlx::query_as(
                r#"
                        INSERT INTO "refresh" (user_id, token)
                        VALUES ($1, $2)
                        ON CONFLICT (user_id) DO UPDATE SET token = EXCLUDED.token
                        RETURNING token
                "#
                )
                .bind(user_id)
                .bind(token)
                .fetch_one(&self.pool)
                .await
                .map_err(Into::into)
        }

        async fn update(&self, user_id: i64, old: &str, token: &str) -> Result<Option<RefreshTokenEntity>> {
                sqlx::query_as(
                r#"
                        UPDATE "refresh"
                        SET token = $1
                        WHERE user_id = $2
                        AND token = $3
                        RETURNING token
                "#
                )
                .bind(token)
                .bind(user_id)
                .bind(old)
                .fetch_optional(&self.pool)
                .await
                .map_err(Into::into)
        }

        async fn delete(&self, user_id: i64) -> Result<Option<RefreshTokenEntity>> {
                sqlx::query_as(
                r#"
                        DELETE FROM "refresh"
                        WHERE id = $1
                        RETURNING token
                "#
                )
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(Into::into)
        }
}