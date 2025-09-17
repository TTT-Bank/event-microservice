use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use super::error::Result;

#[derive(Debug, Deserialize)]
pub struct UserCredentials<'a> {
        pub login: &'a str,
        pub password: &'a [u8]
}

#[derive(Debug, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
pub enum UserRole {
        User,
        Organizer,
        Admin
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserModel {
        pub id: i64,
        pub login: Box<str>,
        pub role: UserRole,
        pub created_at: time::PrimitiveDateTime,
        pub updated_at: time::PrimitiveDateTime
}

pub async fn insert(pool: Pool<Postgres>, credentials: UserCredentials<'_>) -> Result<()> {
        sqlx::query!(
                r#"
                        INSERT INTO "user" (login, password)
                        VALUES ($1, $2)
                "#,
                credentials.login,
                credentials.password
        )
        .execute(&pool)
        .await?;

        Ok(())
}

pub async fn update_role(pool: Pool<Postgres>, id: i64, role: UserRole) -> Result<()> {
        sqlx::query!(
                r#"
                        UPDATE "user"
                        SET role = $1
                        WHERE id = $2
                "#,
                role as _,
                id
        )
        .execute(&pool)
        .await?;

        Ok(())
}

pub async fn get_by_login(pool: Pool<Postgres>, login: &str) -> Result<Option<UserModel>> {
        sqlx::query_as!(
                UserModel,
                r#"
                        SELECT id, login, role AS "role: UserRole", created_at, updated_at
                        FROM "user"
                        WHERE login = $1
                "#,
                login
        )
        .fetch_optional(&pool)
        .await
        .map_err(Into::into)
}

pub async fn get_by_id(pool: Pool<Postgres>, id: i64) -> Result<Option<UserModel>> {
        sqlx::query_as!(
                UserModel,
                r#"
                        SELECT id, login, role AS "role: UserRole", created_at, updated_at
                        FROM "user"
                        WHERE id = $1
                "#,
                id
        )
        .fetch_optional(&pool)
        .await
        .map_err(Into::into)
}

pub async fn get_by_credentials(pool: Pool<Postgres>, credentials: UserCredentials<'_>) -> Result<Option<UserModel>> {
        sqlx::query_as!(
                UserModel,
                r#"
                        SELECT id, login, role AS "role: UserRole", created_at, updated_at
                        FROM "user"
                        WHERE login = $1
                                AND password = $2
                "#,
                credentials.login,
                credentials.password
        )
        .fetch_optional(&pool)
        .await
        .map_err(Into::into)
}

pub async fn delete_by_id(pool: Pool<Postgres>, id: i64) -> Result<()> {
        sqlx::query!(
                r#"
                        DELETE FROM "user"
                        WHERE id = $1
                "#,
                id
        )
        .execute(&pool)
        .await?;

        Ok(())
}