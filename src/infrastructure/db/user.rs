use sqlx::{Pool, Postgres, query_builder::QueryBuilder};

use crate::domain::{user::{model::*, repository::UserRepository}, utils::Offset};

use super::error::Result;

pub struct PgUserRepository {
        pool: Pool<Postgres>
}

impl UserRepository<Pool<Postgres>> for PgUserRepository {
        type Error = sqlx::Error;

        fn init(pool: Pool<Postgres>) -> Self {
                Self { pool }
        }

        async fn get(&self, id: UserId) -> Result<Option<UserModel>> {
                sqlx::query_as(
                        r#"
                        SELECT id, login, role, created_at, updated_at
                        FROM "user"
                        WHERE id = $1
                        "#
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
        }

        async fn list(&self, offset: Offset, filters: &[UserFilter], order_by: &[UserOrder]) -> Result<Vec<UserModel>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"
                        SELECT id, login, role, created_at, updated_at FROM "user"
                        "#);

                if !filters.is_empty() {
                        let mut separated = query_builder.separated(" AND ");
                        separated.push_unseparated(" WHERE ");

                        for filter in filters {
                                match filter {
                                        UserFilter::Login(op) => separated.push("login ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value()),
                                        UserFilter::Role(op) => separated.push("role ").push_unseparated(op.operation() + " ").push_bind_unseparated(op.value()),
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

                query_builder.push(" LIMIT ").push_bind(offset.limit);
                query_builder.push(" OFFSET (").push_bind(offset.limit).push(" * (").push_bind(offset.page).push(" - 1))");

                query_builder.build_query_as().fetch_all(&self.pool).await
        }

        async fn create(&self, credentials: UserCredentials) -> Result<UserModel> {
                sqlx::query_as(
                        r#"
                        INSERT INTO "user" (login, password_hash)
                        VALUES ($1, $2)
                        RETURNING id, login, role, created_at, updated_at
                        "#
                )
                .bind(credentials.login)
                .bind(credentials.password)
                .fetch_one(&self.pool)
                .await
        }

        async fn update(&self, id: UserId, changes: UserUpdate) -> Result<Option<UserModel>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"UPDATE "user" SET "#);

                match changes {
                        UserUpdate::Login(login) => query_builder.push("login = ").push_bind(login).push(' '),
                        UserUpdate::Password(password) => query_builder.push("password_hash = ").push_bind(password).push(' '),
                        UserUpdate::Role(role) => query_builder.push("role = ").push_bind(role).push(' ')
                };

                query_builder.push("WHERE id = ").push_bind(id).push(' ');
                query_builder.push(
                        r#"RETURNING id, login, role, created_at, updated_at"#
                );
                query_builder.build_query_as().fetch_optional(&self.pool).await
        }

        async fn delete(&self, id: UserId) -> Result<Option<UserModel>> {
                sqlx::query_as(
                        r#"
                        DELETE FROM "user"
                        WHERE id = $1
                        RETURNING id, login, role, created_at, updated_at
                        "#
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
        }

        async fn login(&self, login: String) -> Result<Option<UserModelWithPassword>> {
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
        }
}
