use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;

use super::error::Result;

pub type UserId = i64;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCredentials {
        pub login: String,
        pub password: String
}

#[derive(Debug, PartialEq, Eq, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "user_role", rename_all = "PascalCase")]
pub enum UserRole {
        User,
        Organizer,
        Admin
}

#[derive(Debug, PartialEq, Eq, Serialize, ToSchema)]
pub struct UserModel {
        id: UserId,
        login: Box<str>,
        role: UserRole,
        created_at: time::PrimitiveDateTime,
        updated_at: time::PrimitiveDateTime
}

pub async fn insert(conn: &mut PgConnection, credentials: UserCredentials) -> Result<UserModel> {
        sqlx::query_as!(
                UserModel,
                r#"
                        INSERT INTO "user" (login, password)
                        VALUES ($1, $2)
                        RETURNING id, login, role AS "role: UserRole", created_at, updated_at
                "#,
                credentials.login,
                credentials.password
        )
        .fetch_one(conn)
        .await
}

pub async fn update_role(conn: &mut PgConnection, id: UserId, role: UserRole) -> Result<Option<UserModel>> {
        sqlx::query_as!(
                UserModel,
                r#"
                        UPDATE "user"
                        SET role = $1
                        WHERE id = $2
                        RETURNING id, login, role AS "role: UserRole", created_at, updated_at
                "#,
                role as _,
                id
        )
        .fetch_optional(conn)
        .await
}

pub async fn get_by_login(conn: &mut PgConnection, login: &str) -> Result<Option<UserModel>> {
        sqlx::query_as!(
                UserModel,
                r#"
                        SELECT id, login, role AS "role: UserRole", created_at, updated_at
                        FROM "user"
                        WHERE login = $1
                "#,
                login
        )
        .fetch_optional(conn)
        .await
}

pub async fn get_by_id(conn: &mut PgConnection, id: UserId) -> Result<Option<UserModel>> {
        sqlx::query_as!(
                UserModel,
                r#"
                        SELECT id, login, role AS "role: UserRole", created_at, updated_at
                        FROM "user"
                        WHERE id = $1
                "#,
                id
        )
        .fetch_optional(conn)
        .await
}

pub async fn get_by_credentials(conn: &mut PgConnection, credentials: UserCredentials) -> Result<Option<UserModel>> {
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
        .fetch_optional(conn)
        .await
}

pub async fn delete_by_id(conn: &mut PgConnection, id: UserId) -> Result<Option<UserModel>> {
        sqlx::query_as!(
                UserModel,
                r#"
                        DELETE FROM "user"
                        WHERE id = $1
                        RETURNING id, login, role AS "role: UserRole", created_at, updated_at
                "#,
                id
        )
        .fetch_optional(conn)
        .await
}