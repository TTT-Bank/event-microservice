use actix_web::{HttpResponse, delete, get, patch, post, web::{Data, Json, Path, Query}};
use actix_web_httpauth::middleware::HttpAuthentication;
use di::container::DiContainer;
use domain::models::user::{UserRole, UserUpdate};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::user::{dto::UserCredentialsDto, types::{ListUsersQuery, UserIdParam, UserResponse, UserVecResponse}};

use super::{auth::validator, error::Result};

pub fn user_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/users")
                .service(get_user)
                .service(create_user)
                .service(list_users)
                // .service(login)
                .service(scope::scope("")
                        .wrap(HttpAuthentication::bearer(validator))
                        .service(update_user_role)
                        .service(delete_user)
                        // .configure(super::favorite::favorite_app_config)
                )
        );
}

#[utoipa::path]
#[get("/{user_id}")]
async fn get_user(container: Data<DiContainer>, user_id: Path<UserIdParam>) -> Result<HttpResponse> {
        let user = container.create_user_service().get(user_id.0).await?;

        Ok(HttpResponse::Ok().json(UserResponse(user.into())))
}

#[utoipa::path]
#[post("")]
async fn create_user(container: Data<DiContainer>, credentials: Json<UserCredentialsDto>) -> Result<HttpResponse> {
        let user = container.create_user_service().create(credentials.into_inner().into()).await?;

        Ok(HttpResponse::Created().json(UserResponse(user.into())))
}

#[utoipa::path]
#[delete("/{user_id}")]
async fn delete_user(container: Data<DiContainer>, user_id: Path<UserIdParam>) -> Result<HttpResponse> {
        let user = container.create_user_service().delete(user_id.0).await?;

        Ok(HttpResponse::NoContent().json(UserResponse(user.into())))
}

#[utoipa::path]
#[patch("/{user_id}")]
async fn update_user_role(container: Data<DiContainer>, user_id: Path<UserIdParam>, role: Json<UserRole>) -> Result<HttpResponse> {
        let user = container.create_user_service().update(user_id.0, UserUpdate::Role(role.into_inner().into())).await?;

        Ok(HttpResponse::Ok().json(UserResponse(user.into())))
}

#[utoipa::path(params(ListUsersQuery))]
#[get("")]
async fn list_users(container: Data<DiContainer>, query: Query<ListUsersQuery>) -> Result<HttpResponse> {
        let users = container.create_user_service().list(query.offset.clone().into(), &query.filter, &query.order_by).await?;

        Ok(HttpResponse::Ok().json(UserVecResponse(users.into_iter().map(Into::into).collect())))
}