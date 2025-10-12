use actix_web::{HttpResponse, delete, get, patch, post, web::{Data, Json, Path, Query}};
use actix_web_httpauth::middleware::HttpAuthentication;
use di::container::DiContainer;
use domain::models::{event::{EventUpdate, NewEvent}, user::UserRole};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use actix_web_grants::protect;

use super::{dto::{EventStatusDto, NewEventDto}, types::{EventIdParam, EventResponse, EventVecResponse, ListEventsQuery}};

use super::super::{authentication::{validator, ReqClaims}, error::{HandlerError, Result}};

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

#[utoipa::path(params(EventIdParam))]
#[get("/{event_id}")]
async fn get_event(container: Data<DiContainer>, path: Path<EventIdParam>) -> Result<HttpResponse> {
        let event_id = path.into_inner().try_into()?;
        let event_service = container.create_event_service();

        let event = event_service.get(event_id).await?;

        let response_body = EventResponse::from(event);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}

#[utoipa::path]
#[post("")]
#[protect(any("UserRole::Organizer", "UserRole::Admin"), ty = "UserRole")]
async fn create_event(container: Data<DiContainer>, body: Json<NewEventDto>, claims: ReqClaims) -> Result<HttpResponse> {
        let new_event: NewEvent = body.into_inner().try_into()?;

        if claims.0.sub != new_event.organizer_id {
                return Err(HandlerError::IdMismatch);
        }

        let event_service = container.create_event_service();

        let event = event_service.create(new_event).await?;

        let response_body = EventResponse::from(event);
        let response = HttpResponse::Created().json(response_body);
        Ok(response)
}

#[utoipa::path(params(EventIdParam))]
#[delete("/{event_id}")]
#[protect(any("UserRole::Organizer", "UserRole::Admin"), ty = "UserRole")]
async fn delete_event(container: Data<DiContainer>, path: Path<EventIdParam>, claims: ReqClaims) -> Result<HttpResponse> {
        let event_id = path.into_inner().try_into()?;

        if claims.0.sub != event_id {
                return Err(HandlerError::IdMismatch);
        }

        let event_service = container.create_event_service();

        let event = event_service.delete(event_id).await?;

        let response_body = EventResponse::from(event);
        let response = HttpResponse::NoContent().json(response_body);
        Ok(response)
}

#[utoipa::path(params(EventIdParam))]
#[patch("/{event_id}")]
#[protect("UserRole::Admin", ty = "UserRole")]
async fn update_event_status(container: Data<DiContainer>, path: Path<EventIdParam>, body: Json<EventStatusDto>) -> Result<HttpResponse> {
        let event_id = path.into_inner().try_into()?;
        let event_status = body.into_inner().try_into()?;
        let event_service = container.create_event_service();

        let event = event_service.update(event_id, EventUpdate::Status(event_status)).await?;

        let response_body = EventResponse::from(event);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}

#[utoipa::path(params(ListEventsQuery))]
#[get("")]
async fn list_events(container: Data<DiContainer>, query: Query<ListEventsQuery>) -> Result<HttpResponse> {
        let query = query.into_inner();
        let event_service = container.create_event_service();

        let events = event_service.list(query.offset.try_into()?, &query.filter, &query.order_by).await?;

        let response_body = EventVecResponse::from(events);
        let response = HttpResponse::Ok().json(response_body);
        Ok(response)
}