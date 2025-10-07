use actix_web::{HttpResponse, delete, get, patch, post, web::{Data, Json, Path, Query}};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, StringWithSeparator, formats::CommaSeparator};
use serde_aux::field_attributes::deserialize_default_from_empty_object;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
        domain::{event::{model::*, repository::EventRepository}, user::model::UserRole, utils::Offset}, handles::auth::{AuthenticatedUser, validator}, infrastructure::{db::event::PgEventRepository, provider::PgProvider}
};

use super::error::Result;
use utoipa::ToSchema;

pub fn event_app_config(cfg: &mut ServiceConfig) {
        cfg
                .service(scope::scope("/events")
                .service(get_event)
                .service(list_events)
                .service(scope::scope("")
                        .wrap(HttpAuthentication::bearer(validator))
                        .service(create_event)
                        .service(update_event_status)
                        .service(delete_event)
                )
        );
}

#[derive(Debug, Serialize, utoipa::ToResponse)]
struct EventResponse {
        event: EventModel
}

#[utoipa::path(params(("event_id" = EventId, Path)))]
#[get("/{event_id}")]
async fn get_event(provider: Data<PgProvider>, id: Path<EventId>) -> Result<HttpResponse> {
        let repo = provider.provide_event_repository::<PgEventRepository>();
        let event = repo.get(*id).await?;

        let res = match event {
                Some(event) => {
                        HttpResponse::Ok().json(EventResponse {event})
                }
                None => {
                        HttpResponse::NotFound().finish()
                }
        };

        Ok(res)
}

#[utoipa::path]
#[post("")]
async fn create_event(provider: Data<PgProvider>, auth_user: AuthenticatedUser, event: Json<NewEvent>) -> Result<HttpResponse> {
        if auth_user.role == UserRole::User || auth_user.id != event.organizer_id {
                return Ok(HttpResponse::Forbidden().finish());
        }

        let repo = provider.provide_event_repository::<PgEventRepository>();

        let event =
                repo.create(event.into_inner())
                        .await
                        .map_err(Into::into);

        match event {
                Ok(event) => {
                        Ok(HttpResponse::Created().json(EventResponse {event}))
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

#[utoipa::path(params(("event_id" = EventId, Path)))]
#[delete("/{event_id}")]
async fn delete_event(provider: Data<PgProvider>, auth_user: AuthenticatedUser, id: Path<EventId>) -> Result<HttpResponse> {
        let repo = provider.provide_event_repository::<PgEventRepository>();
        if auth_user.role == UserRole::User || (auth_user.role == UserRole::Organizer && repo.get(*id).await?.is_none_or(|v| v.organizer_id == *id)) {
                return Ok(HttpResponse::Forbidden().finish());
        }

        let event = repo.delete(*id).await?;

        let res = match event {
                Some(event) => {
                        HttpResponse::NoContent().json(EventResponse {event})
                }
                None => {
                        HttpResponse::NotFound().finish()
                }
        };

        Ok(res)
}

#[derive(Debug, Deserialize, ToSchema)]
struct EventStatusBody {
        status: EventStatus
}

#[utoipa::path(params(("event_id" = EventId, Path)))]
#[patch("/{event_id}")]
async fn update_event_status(provider: Data<PgProvider>, auth_user: AuthenticatedUser, id: Path<EventId>, status: Json<EventStatusBody>) -> Result<HttpResponse> {
        if auth_user.role < UserRole::Admin {
                return Ok(HttpResponse::Forbidden().finish());
        }

        let repo = provider.provide_event_repository::<PgEventRepository>();
        let event = repo.update(*id, EventUpdate::Status(status.into_inner().status)).await?;

        let res = match event {
                Some(event) => {
                        HttpResponse::Ok().json(EventResponse {event})
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
struct ListEventsQuery {
        #[param(required = false)]
        #[serde(flatten, deserialize_with = "deserialize_default_from_empty_object")]
        offset: Offset,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, EventFilter>")]
        filter: Vec<EventFilter>,
        #[param(value_type = String)]
        #[serde(default)]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, EventOrder>")]
        order_by: Vec<EventOrder>
}

#[derive(Debug, Serialize, utoipa::ToResponse)]
struct ListEventsResponse {
        events: Vec<EventModel>
}

#[utoipa::path(params(ListEventsQuery))]
#[get("")]
async fn list_events(provider: Data<PgProvider>, query: Query<ListEventsQuery>) -> Result<HttpResponse> {
        let repo = provider.provide_event_repository::<PgEventRepository>();

        let events = repo.list(query.offset.clone(), &query.filter, &query.order_by).await?;

        Ok(HttpResponse::Ok().json(ListEventsResponse {events}))
}