use actix_web::{HttpResponse, delete, get, patch, post, web::{Data, Json, Path, Query}};
use actix_web_httpauth::middleware::HttpAuthentication;
use di::container::DiContainer;
use domain::models::user::{UserUpdate, UserRole};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use actix_web_grants::protect;

use crate::api::refresh::handles::token_app_config;

use super::{dto::{UserRoleDto, UserCredentialsDto}, types::{ListUsersQuery, UserIdParam, UserResponse, UserVecResponse}};

use super::super::{authentication::validator, error::Result};

pub fn user_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/users")
                .service(get_user)
                .service(create_user)
                .service(list_users)
                .configure(token_app_config)
                .service(scope::scope("")
                        .wrap(HttpAuthentication::bearer(validator))
                        .service(update_user_role)
                        .service(delete_user)
                        .configure(super::super::favorite::handles::favorite_app_config)
                )
        );
}

#[utoipa::path(params(UserIdParam))]
#[get("/{user_id}")]
async fn get_user(container: Data<DiContainer>, path: Path<UserIdParam>) -> Result<HttpResponse> {
        let user_id = path.into_inner().try_into()?;
        let user_service = container.create_user_service();

        let user = user_service.get(user_id).await?;

        let response_body = UserResponse::from(user);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}

#[utoipa::path]
#[post("")]
async fn create_user(container: Data<DiContainer>, body: Json<UserCredentialsDto>) -> Result<HttpResponse> {
        let credentials = body.into_inner().try_into()?;
        let user_service = container.create_user_service();

        let user = user_service.create(credentials).await?;

        let response_body = UserResponse::from(user);
        let response = HttpResponse::Created().json(response_body);
        Ok(response)
}

#[utoipa::path(params(UserIdParam))]
#[delete("/{user_id}")]
#[protect("UserRole::Admin", ty = "UserRole")]
async fn delete_user(container: Data<DiContainer>, path: Path<UserIdParam>) -> Result<HttpResponse> {
        let user_id = path.into_inner().try_into()?;
        let user_service = container.create_user_service();

        let user = user_service.delete(user_id).await?;

        let response_body = UserResponse::from(user);
        let response = HttpResponse::NoContent().json(response_body);
        Ok(response)
}

#[utoipa::path(params(UserIdParam))]
#[patch("/{user_id}")]
#[protect("UserRole::Admin", ty = "UserRole")]
async fn update_user_role(container: Data<DiContainer>, path: Path<UserIdParam>, body: Json<UserRoleDto>) -> Result<HttpResponse> {
        let user_id = path.into_inner().try_into()?;
        let user_role = body.into_inner().try_into()?;
        let user_service = container.create_user_service();

        let user = user_service.update(user_id, UserUpdate::Role(user_role)).await?;

        let response_body = UserResponse::from(user);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}

#[utoipa::path(params(ListUsersQuery))]
#[get("")]
async fn list_users(container: Data<DiContainer>, query: Query<ListUsersQuery>) -> Result<HttpResponse> {
        let query = query.into_inner();
        let user_service = container.create_user_service();

        let users = user_service.list(query.offset.try_into()?, &query.filter, &query.order_by).await?;

        let response_body = UserVecResponse::from(users);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}