use actix_web::{HttpResponse, delete, get, http::header::ContentType, patch, post, web::{Data, Json, Path}};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::db::user::{self, UserCredentials, UserModel, UserRole};

use super::error::Result;
use utoipa::ToSchema;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

#[derive(Debug, Serialize, ToSchema)]
struct UserResponse {
        user: UserModel
}

#[utoipa::path]
#[get("/users/{id}")]
pub async fn get_user(pool: Data<Pool<Postgres>>, id: Path<i64>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let user = user::get_by_id(transaction.as_mut(), *id).await?;
        transaction.commit().await?;

        let res = match user {
                Some(user) => {
                        HttpResponse::Ok()
                                .content_type(ContentType::json())
                                .body(serde_json::to_string(&UserResponse {user})?)
                }
                None => {
                        HttpResponse::NotFound()
                                .content_type(ContentType::json())
                                .body(format!("User {id} was not found"))
                }
        };

        Ok(res)
}

#[utoipa::path]
#[post("/users")]
pub async fn create_user(pool: Data<Pool<Postgres>>, mut credentials: Json<UserCredentials>) -> Result<HttpResponse> {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash =
                Argon2::default()
                        .hash_password(credentials.password.as_bytes(), &salt)?
                        .to_string();
        credentials.password = password_hash;

        let mut transaction = pool.begin().await?;
        let user =
                user::insert(transaction.as_mut(), credentials.into_inner())
                        .await
                        .map_err(Into::into);
        transaction.commit().await?;

        match user {
                Ok(user) => {
                        Ok(HttpResponse::Created()
                                .content_type(ContentType::json())
                                .body(serde_json::to_string(&UserResponse {user})?))
                }
                Err(err) => {
                        if let super::error::HandlerError::Db(db_err) = &err  {
                                if db_err == &sqlx::error::ErrorKind::UniqueViolation {
                                        return Ok(HttpResponse::Conflict()
                                                .content_type(ContentType::json())
                                                .body(format!("User already exists")))
                                }
                        }

                        return Err(err)
                }
        }
}

#[utoipa::path]
#[delete("/users/{id}")]
pub async fn delete_user(pool: Data<Pool<Postgres>>, id: Path<i64>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let user = user::delete_by_id(transaction.as_mut(), *id).await?;
        transaction.commit().await?;

        let res = match user {
                Some(user) => {
                        HttpResponse::NoContent()
                                .content_type(ContentType::json())
                                .body(serde_json::to_string(&UserResponse {user})?)
                }
                None => {
                        HttpResponse::NotFound()
                                .content_type(ContentType::json())
                                .body(format!("User {id} was not found"))
                }
        };

        Ok(res)
}

#[derive(Debug, Deserialize, ToSchema)]
struct UserRoleBody {
        role: UserRole
}

#[utoipa::path]
#[patch("/users/{id}")]
pub async fn update_user_role(pool: Data<Pool<Postgres>>, id: Path<i64>, role: Json<UserRoleBody>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let user = user::update_role(transaction.as_mut(), *id, role.into_inner().role).await?;
        transaction.commit().await?;

        let res = match user {
                Some(user) => {
                        HttpResponse::Ok()
                                .content_type(ContentType::json())
                                .body(serde_json::to_string(&UserResponse {user})?)
                }
                None => {
                        HttpResponse::NotFound()
                                .content_type(ContentType::json())
                                .body(format!("User {id} was not found"))
                }
        };

        Ok(res)
}