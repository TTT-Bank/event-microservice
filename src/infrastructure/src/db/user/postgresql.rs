use async_trait::async_trait;
use sqlx::{Pool, Postgres, query_builder::QueryBuilder};

use domain::models::{user::{UserUpdate, UserFilter, UserOrder}, utils::Offset};

use super::repository::UserRepository;
use super::entity::UserEntity;
use crate::Result;

pub struct PgUserRepository {
        pool: Pool<Postgres>
}

impl PgUserRepository {
        pub fn new(pool: Pool<Postgres>) -> Self {
                Self { pool }
        }
}

#[async_trait]
impl UserRepository for PgUserRepository {
        async fn get(&self, id: i64) -> Result<Option<UserEntity>> {
                sqlx::query_as(
                        r#"
                        SELECT id, login, password_hash, role, created_at, updated_at
                        FROM "user"
                        WHERE id = $1
                        "#
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(Into::into)
        }

        async fn list(&self, offset: Offset, filters: &[UserFilter], order_by: &[UserOrder]) -> Result<Vec<UserEntity>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"
                        SELECT id, login, password_hash, role, created_at, updated_at FROM "user"
                        "#);

                if !filters.is_empty() {
                        let mut separated = query_builder.separated(" AND ");
                        separated.push_unseparated(" WHERE ");

                        for filter in filters {
                                match filter {
                                        UserFilter::Login(op) => separated.push("login ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value()),
                                        UserFilter::Role(op) => separated.push("role ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value().to_string()),
                                };
                        }
                }

                if !order_by.is_empty() {
                        let mut separated = query_builder.separated(", ");
                        separated.push_unseparated(" ORDER BY ");

                        for order in order_by {
                                match order {
                                        UserOrder::Id(op) => separated.push("id ").push_unseparated(op.to_string()),
                                        UserOrder::Role(op) => separated.push("role ").push_unseparated(op.to_string()),
                                        UserOrder::CreatedAt(op) => separated.push("created_at ").push_unseparated(op.to_string()),
                                        UserOrder::UpdatedAt(op) => separated.push("updated_at ").push_unseparated(op.to_string()),
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

        async fn create(&self, login: &str, password_hash: &str) -> Result<UserEntity> {
                sqlx::query_as(
                        r#"
                        INSERT INTO "user" (login, password_hash)
                        VALUES ($1, $2)
                        RETURNING id, login, password_hash, role, created_at, updated_at
                        "#
                )
                .bind(login)
                .bind(password_hash)
                .fetch_one(&self.pool)
                .await
                .map_err(Into::into)
        }

        async fn update(&self, id: i64, changes: UserUpdate) -> Result<Option<UserEntity>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"UPDATE "user" SET "#);

                match changes {
                        UserUpdate::Login(login) => query_builder.push("login = ").push_bind(login).push(' '),
                        UserUpdate::Password(password) => query_builder.push("password_hash = ").push_bind(password).push(' '),
                        UserUpdate::Role(role) => query_builder.push("role = ").push_bind(role.to_string()).push(' ')
                };

                query_builder.push("WHERE id = ").push_bind(id).push(' ');
                query_builder.push(
                        r#"RETURNING id, login, password_hash, role, created_at, updated_at"#
                );
                query_builder
                        .build_query_as()
                        .fetch_optional(&self.pool)
                        .await
                        .map_err(Into::into)
        }

        async fn delete(&self, id: i64) -> Result<Option<UserEntity>> {
                sqlx::query_as(
                        r#"
                        DELETE FROM "user"
                        WHERE id = $1
                        RETURNING id, login, password_hash, role, created_at, updated_at
                        "#
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(Into::into)
        }

        async fn get_by_login(&self, login: &str) -> Result<Option<UserEntity>> {
                sqlx::query_as(
                        r#"
                        SELECT id, login, password_hash, role, created_at, updated_at
                        FROM "user"
                        WHERE login = $1
                        "#
                )
                .bind(login)
                .fetch_optional(&self.pool)
                .await
                .map_err(Into::into)
        }
}
