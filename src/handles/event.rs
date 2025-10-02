use actix_web::{HttpResponse, delete, get, patch, post, web::{Data, Json, Path}};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::db::{utils::Offset, event::{self, EventModel, NewEvent, Status}};

use super::error::Result;

pub fn event_app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/events")
        .service(get_event)
        .service(create_event)
        .service(update_event_status)
        .service(list_events)
        .service(delete_event)
        );
}

#[derive(Debug, Serialize, ToSchema)]
struct EventResponse {
        event: EventModel
}

#[utoipa::path]
#[get("/{id}")]
async fn get_event(pool: Data<Pool<Postgres>>, id: Path<i64>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let event = event::get_by_id(transaction.as_mut(), *id).await?;
        transaction.commit().await?;

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
async fn create_event(pool: Data<Pool<Postgres>>, event: Json<NewEvent>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let event =
                event::insert(transaction.as_mut(), event.into_inner())
                        .await
                        .map_err(Into::into);
        transaction.commit().await?;

        match event {
                Ok(event) => {
                        Ok(HttpResponse::Created().json(EventResponse {event}))
                }
                Err(err) => {
                        if let super::error::HandlerError::Db(db_err) = &err {
                                if db_err == &sqlx::error::ErrorKind::UniqueViolation {
                                        return Ok(HttpResponse::Conflict().finish())
                                }
                        }

                        return Err(err)
                }
        }
}

#[utoipa::path]
#[delete("/{id}")]
async fn delete_event(pool: Data<Pool<Postgres>>, id: Path<i64>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let event = event::delete_by_id(transaction.as_mut(), *id).await?;
        transaction.commit().await?;

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
        status: Status
}

#[utoipa::path]
#[patch("/{id}")]
async fn update_event_status(pool: Data<Pool<Postgres>>, id: Path<i64>, status: Json<EventStatusBody>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let event = event::update_status(transaction.as_mut(), *id, status.status.clone()).await?;
        transaction.commit().await?;

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

#[derive(Debug, Deserialize, ToSchema)]
struct ListEventsBody {
        status: Status,
        #[serde(default)]
        offset: Offset
}

#[derive(Debug, Serialize, ToSchema)]
struct ListEventsResponse {
        events: Vec<EventModel>
}

#[utoipa::path]
#[get("")]
async fn list_events(pool: Data<Pool<Postgres>>, body: Json<ListEventsBody>) -> Result<HttpResponse> {
        let mut transaction = pool.begin().await?;
        let events = event::get_all_by_status(transaction.as_mut(), body.status.clone(), body.offset.clone()).await?;
        transaction.commit().await?;

        Ok(HttpResponse::Ok().json(ListEventsResponse {events}))
}