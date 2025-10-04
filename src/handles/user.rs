use actix_web::{HttpResponse, delete, get, patch, post, web::{Data, Json, Path, Query}};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};
use serde_aux::field_attributes::deserialize_default_from_empty_object;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
        domain::{user::{model::*, repository::UserRepository}, utils::Offset}, infrastructure::{db::user::PgUserRepository, provider::PgProvider}
};

use super::error::Result;
use utoipa::ToSchema;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

pub fn user_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/users")
                .service(get_user)
                .service(create_user)
                .service(delete_user)
                .service(update_user_role)
                .service(list_users)
                .configure(super::favorite::favorite_app_config)
        );
}

#[derive(Debug, Serialize, utoipa::ToResponse)]
struct UserResponse {
        user: UserModel
}

#[utoipa::path(params(("user_id" = UserId, Path)))]
#[get("/{user_id}")]
async fn get_user(provider: Data<PgProvider>, id: Path<UserId>) -> Result<HttpResponse> {
        let repo = provider.provide_user_repository::<PgUserRepository>();
        let user = repo.get(*id).await?;

        let res = match user {
                Some(user) => {
                        HttpResponse::Ok().json(UserResponse {user})
                }
                None => {
                        HttpResponse::NotFound().finish()
                }
        };

        Ok(res)
}

#[utoipa::path]
#[post("")]
async fn create_user(provider: Data<PgProvider>, mut credentials: Json<UserCredentials>) -> Result<HttpResponse> {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash =
                Argon2::default()
                        .hash_password(credentials.password.as_bytes(), &salt)?
                        .to_string();
        credentials.password = password_hash.into();

        let repo = provider.provide_user_repository::<PgUserRepository>();

        let user =
                repo.create(credentials.into_inner())
                        .await
                        .map_err(Into::into);

        match user {
                Ok(user) => {
                        Ok(HttpResponse::Created().json(UserResponse {user}))
                }
                Err(err) => {
                        if let super::error::HandlerError::Db(db_err) = &err  {
                                if db_err == &sqlx::error::ErrorKind::UniqueViolation {
                                        return Ok(HttpResponse::Conflict().finish())
                                }
                        }

                        return Err(err)
                }
        }
}

#[utoipa::path(params(("user_id" = UserId, Path)))]
#[delete("/{user_id}")]
async fn delete_user(provider: Data<PgProvider>, id: Path<UserId>) -> Result<HttpResponse> {
        let repo = provider.provide_user_repository::<PgUserRepository>();
        let user = repo.delete(*id).await?;

        let res = match user {
                Some(user) => {
                        HttpResponse::NoContent().json(UserResponse {user})
                }
                None => {
                        HttpResponse::NotFound().finish()
                }
        };

        Ok(res)
}

#[derive(Debug, Deserialize, ToSchema)]
struct UserRoleBody {
        role: UserRole
}

#[utoipa::path(params(("user_id" = UserId, Path)))]
#[patch("/{user_id}")]
async fn update_user_role(provider: Data<PgProvider>, id: Path<UserId>, role: Json<UserRoleBody>) -> Result<HttpResponse> {
        let repo = provider.provide_user_repository::<PgUserRepository>();
        let user = repo.update(*id, UserUpdate::Role(role.into_inner().role)).await?;

        let res = match user {
                Some(user) => {
                        HttpResponse::Ok().json(UserResponse {user})
                }
                None => {
                        HttpResponse::NotFound().finish()
                }
        };

        Ok(res)
}

#[serde_as]
#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
struct ListUsersQuery {
        #[param(required = false)]
        #[serde(flatten, deserialize_with = "deserialize_default_from_empty_object")]
        offset: Offset,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, UserFilter>")]
        filter: Vec<UserFilter>,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, UserOrder>")]
        order_by: Vec<UserOrder>
}

#[derive(Debug, Serialize, utoipa::ToResponse)]
struct ListUsersResponse {
        users: Vec<UserModel>
}

#[utoipa::path(params(ListUsersQuery))]
#[get("")]
async fn list_users(provider: Data<PgProvider>, query: Query<ListUsersQuery>) -> Result<HttpResponse> {
        let repo = provider.provide_user_repository::<PgUserRepository>();

        let users = repo.list(query.offset.clone(), &query.filter, &query.order_by).await?;

        Ok(HttpResponse::Ok().json(ListUsersResponse {users}))
}