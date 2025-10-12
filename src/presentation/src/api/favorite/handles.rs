use actix_web::{HttpResponse, delete, get, post, web::{Data, Path, Query}};
use di::container::DiContainer;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::api::user::types::UserIdParam;

use super::{types::{FavoriteIdParam, FavoriteEventResponse, FavoriteEventVecResponse, FavoriteResponse, ListFavoriteEventsQuery}};

use super::super::error::Result;

pub fn favorite_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/{user_id}/favorite")
                .service(get_favorite)
                .service(list_favorites)
                .service(create_favorite)
                .service(delete_favorite)
        );
}

#[utoipa::path(params(FavoriteIdParam))]
#[get("/{event_id}")]
async fn get_favorite(container: Data<DiContainer>, path: Path<FavoriteIdParam>) -> Result<HttpResponse> {
        let favorite_id = path.into_inner().try_into()?;
        let favorite_service = container.create_favorite_service();

        let favorite_event = favorite_service.get(favorite_id).await?;

        let response_body = FavoriteEventResponse::from(favorite_event);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}

#[utoipa::path(params(FavoriteIdParam))]
#[post("/{event_id}")]
async fn create_favorite(container: Data<DiContainer>, path: Path<FavoriteIdParam>) -> Result<HttpResponse> {
        let favorite_id = path.into_inner().try_into()?;
        let favorite_service = container.create_favorite_service();

        let favorite = favorite_service.create(favorite_id).await?;

        let response_body = FavoriteResponse::from(favorite);
        let response = HttpResponse::Created().json(response_body);
        Ok(response)
}

#[utoipa::path(params(FavoriteIdParam))]
#[delete("/{event_id}")]
async fn delete_favorite(container: Data<DiContainer>, path: Path<FavoriteIdParam>) -> Result<HttpResponse> {
        let favorite_id = path.into_inner().try_into()?;
        let favorite_service = container.create_favorite_service();

        let favorite = favorite_service.delete(favorite_id).await?;

        let response_body = FavoriteResponse::from(favorite);
        let response = HttpResponse::NoContent().json(response_body);
        Ok(response)
}

#[utoipa::path(params(UserIdParam, ListFavoriteEventsQuery))]
#[get("")]
async fn list_favorites(container: Data<DiContainer>, path: Path<UserIdParam>, query: Query<ListFavoriteEventsQuery>) -> Result<HttpResponse> {
        let user_id = path.into_inner().try_into()?;
        let query = query.into_inner();
        let favorite_service = container.create_favorite_service();

        let favorite_events = favorite_service.list(user_id, query.offset.try_into()?, &query.filter, &query.order_by).await?;

        let response_body = FavoriteEventVecResponse::from(favorite_events);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}