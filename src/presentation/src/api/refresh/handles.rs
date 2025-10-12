use actix_web::{HttpResponse, post, put, web::{Data, Json}};
use di::container::DiContainer;
use domain::models::token::TokenPair;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::api::{refresh::{dto::TokenPairDto, types::TokenResponse}, user::dto::UserCredentialsDto};

use super::super::error::Result;

pub fn token_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/login")
                .service(create_refresh)
                .service(update_refresh)
        );
}

#[utoipa::path]
#[post("")]
async fn create_refresh(container: Data<DiContainer>, body: Json<UserCredentialsDto>) -> Result<HttpResponse> {
        let credentials = body.into_inner().try_into()?;
        let user_service = container.create_user_service();
        let refresh_service = container.create_refresh_service();

        let user = user_service.get_by_login(credentials).await?;
        let tokens = refresh_service.create(user.id, user.role).await?;

        let response_body = TokenResponse::from(tokens);
        let response = HttpResponse::Created().json(response_body);
        Ok(response)
}

#[utoipa::path]
#[put("")]
async fn update_refresh(container: Data<DiContainer>, body: Json<TokenPairDto>) -> Result<HttpResponse> {
        let tokens = TokenPair::from(body.into_inner());
        let refresh_service = container.create_refresh_service();

        let tokens = refresh_service.update(tokens.refresh_token).await?;

        let response_body = TokenResponse::from(tokens);
        let response = HttpResponse::Created().json(response_body);
        Ok(response)
}