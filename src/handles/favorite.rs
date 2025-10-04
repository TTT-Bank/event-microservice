use actix_web::{HttpResponse, delete, get, post, web::{Data, Json, Path, Query}};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};
use serde_aux::field_attributes::deserialize_default_from_empty_object;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
        domain::{event::model::{EventId, EventModel}, favorite::{model::*, repository::FavoriteRepository}, user::model::UserId, utils::Offset}, infrastructure::{db::favorite::PgFavoriteRepository, provider::PgProvider}
};

use super::error::Result;
use utoipa::ToSchema;

pub fn favorite_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/{user_id}/favorites")
        .service(get_favorite)
        .service(create_favorite)
        .service(delete_favorite)
        .service(list_favorite)
        );
}

#[derive(Debug, Serialize, utoipa::ToResponse)]
struct FavoriteEventResponse {
        favorite_event: EventModel
}

#[utoipa::path(params(("user_id" = UserId, Path), ("event_id" = EventId, Path)))]
#[get("/{event_id}")]
async fn get_favorite(provider: Data<PgProvider>, id: Path<(UserId, EventId)>) -> Result<HttpResponse> {
        let repo = provider.provide_favorite_repository::<PgFavoriteRepository>();
        let id = *id;
        let favorite_event = repo.get(id.0, id.1).await?;

        let res = match favorite_event {
                Some(favorite_event) => {
                        HttpResponse::Ok().json(FavoriteEventResponse {favorite_event})
                }
                None => {
                        HttpResponse::NotFound().finish()
                }
        };

        Ok(res)
}

#[derive(Debug, Deserialize, ToSchema)]
struct EventIdBody {
        event_id: EventId
}

#[derive(Debug, Serialize, utoipa::ToResponse)]
struct FavoriteResponse {
        favorite: FavoriteModel
}

#[utoipa::path(params(("user_id" = UserId, Path)))]
#[post("")]
async fn create_favorite(provider: Data<PgProvider>, user_id: Path<UserId>, event_id: Json<EventIdBody>) -> Result<HttpResponse> {
        let repo = provider.provide_favorite_repository::<PgFavoriteRepository>();

        log::info!("{user_id} + {event_id:?}");
        let favorite =
                repo.create(*user_id, event_id.event_id)
                        .await
                        .map_err(Into::into);

        match favorite {
                Ok(favorite) => {
                        Ok(HttpResponse::Created().json(FavoriteResponse {favorite}))
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

#[utoipa::path(params(("user_id" = UserId, Path), ("event_id" = EventId, Path)))]
#[delete("/{event_id}")]
async fn delete_favorite(provider: Data<PgProvider>, id: Path<(UserId, EventId)>) -> Result<HttpResponse> {
        let repo = provider.provide_favorite_repository::<PgFavoriteRepository>();
        let id = *id;
        let favorite = repo.delete(id.0, id.1).await?;

        let res = match favorite {
                Some(favorite) => {
                        HttpResponse::NoContent().json(FavoriteResponse {favorite})
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
struct ListFavoriteEventsQuery {
        #[param(required = false)]
        #[serde(flatten, deserialize_with = "deserialize_default_from_empty_object")]
        offset: Offset,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, FavoriteFilter>")]
        filter: Vec<FavoriteFilter>,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, FavoriteOrder>")]
        order_by: Vec<FavoriteOrder>
}

#[derive(Debug, Serialize, utoipa::ToResponse)]
struct ListFavoriteEventsResponse {
        favorite_events: Vec<EventModel>
}

#[utoipa::path(params(ListFavoriteEventsQuery, ("user_id" = UserId, Path)))]
#[get("")]
async fn list_favorite(provider: Data<PgProvider>, user_id: Path<UserId>, query: Query<ListFavoriteEventsQuery>) -> Result<HttpResponse> {
        let repo = provider.provide_favorite_repository::<PgFavoriteRepository>();

        let favorite_events = repo.list(*user_id, query.offset.clone(), &query.filter, &query.order_by).await?;

        Ok(HttpResponse::Ok().json(ListFavoriteEventsResponse {favorite_events}))
}