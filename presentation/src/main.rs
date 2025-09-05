use core::model::event::entry::EventEntry;

use actix_web::{App, HttpResponse, HttpServer, Result, web};

#[actix_web::main]
async fn main() -> Result<()> {
        HttpServer::new(||
                App::new()
                        .default_service(
                                web::get().to(|| HttpResponse::Ok())
                        )
        )
        .workers(1)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

        Ok(())
}
