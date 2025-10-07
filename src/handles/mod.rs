pub mod user;
pub mod event;
pub mod favorite;
mod error;
pub mod auth;

use actix_web::{HttpResponse, get, http::header::ContentType};

#[utoipa::path(responses((status = OK)))]
#[get("/ping")]
pub async fn ping() -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body("pong")
}