use actix_web::{HttpResponse, get};

#[utoipa::path(responses((status = OK)))]
#[get("/ping")]
pub async fn ping() -> HttpResponse {
        HttpResponse::Ok().body("pong")
}