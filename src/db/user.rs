use sqlx::{PgTransaction, Postgres, query_builder::QueryBuilder};

use crate::domain::{utils::Offset, user::{model::*, repository::UserRepository}};

use super::error::Result;

pub struct PgUserRepository<'a> {
        tx: PgTransaction<'a>
}

impl<'a> PgUserRepository<'a> {
        pub fn new(tx: PgTransaction<'a>) -> Self {
                Self { tx }
        }

        pub async fn commit(self) -> Result<()> {
                self.tx.commit().await
        }
}

impl UserRepository for PgUserRepository<'_> {
        type Error = sqlx::Error;

        async fn get(&mut self, id: UserId) -> Result<Option<UserModel>> {
                sqlx::query_as!(
                        UserModel,
                        r#"
                        SELECT id, login, password_hash, role AS "role: UserRole", created_at, updated_at
                        FROM "user"
                        WHERE id = $1
                        "#,
                        id
                )
                .fetch_optional(self.tx.as_mut())
                .await
        }

        async fn list(&mut self, offset: Offset, filters: &[UserFilter], order_by: &[UserOrder]) -> Result<Vec<UserModel>> {
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

                query_builder.build_query_as().fetch_all(self.tx.as_mut()).await
        }

        async fn create(&mut self, credentials: UserCredentials) -> Result<UserModel> {
                sqlx::query_as!(
                        UserModel,
                        r#"
                        INSERT INTO "user" (login, password_hash)
                        VALUES ($1, $2)
                        RETURNING id, login, password_hash, role AS "role: UserRole", created_at, updated_at
                        "#,
                        credentials.login,
                        credentials.password
                )
                .fetch_one(self.tx.as_mut())
                .await
        }

        async fn update(&mut self, id: UserId, changes: UserUpdate) -> Result<Option<UserModel>> {
                let mut query_builder =
                        QueryBuilder::<Postgres>::new(r#"UPDATE "user" SET "#);

                match changes {
                        UserUpdate::Login(login) => query_builder.push("login = ").push_bind(login).push(' '),
                        UserUpdate::Password(password) => query_builder.push("password_hash = ").push_bind(password).push(' '),
                        UserUpdate::Role(role) => query_builder.push("role = ").push_bind(role).push(' ')
                };

                query_builder.push("WHERE id = ").push_bind(id).push(' ');
                query_builder.push(
                        r#"RETURNING id, login, password_hash, role, created_at, updated_at"#
                );
                query_builder.build_query_as().fetch_optional(self.tx.as_mut()).await
        }

        async fn delete(&mut self, id: UserId) -> Result<Option<UserModel>> {
                sqlx::query_as!(
                        UserModel,
                        r#"
                        DELETE FROM "user"
                        WHERE id = $1
                        RETURNING id, login, password_hash, role AS "role: UserRole", created_at, updated_at
                        "#,
                        id
                )
                .fetch_optional(self.tx.as_mut())
                .await
        }
}
