mod db;
mod handles;
mod error;

use std::{net::SocketAddr, sync::LazyLock};

use actix_web::{App, HttpServer, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::header, middleware::{self, Next}};
use utoipa_actix_web::{AppExt, scope, service_config::ServiceConfig};
use error::Result;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

fn app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/api/v1")
                .service(handles::ping)
        );
}

fn openapi_service_factory(api: utoipa::openapi::OpenApi) -> SwaggerUi {
        SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api/openapi.json", api)
}

static SERVER_WORKERS: LazyLock<usize> = LazyLock::new(|| {
        dotenvy::var("SERVER_WORKERS")
                .expect("SERVER_WORKERS var should be set")
                .parse()
                .expect("SERVER_WORKERS should be a number greater than 0")
});

static SERVER_ADDRESS: LazyLock<SocketAddr> = LazyLock::new(|| {
        dotenvy::var("SERVER_ADDRESS")
                .expect("SERVER_ADDRESS var should be set")
                .parse()
                .expect("SERVER_ADDRESS should be valid socket address")
});

#[derive(OpenApi)]
#[openapi(
        info(
                title = "TEST",
                version = "0.1.0",
        ),
        tags([
                name = "User"
        ], [
                name = "Event"
        ])
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<()> {
        dotenvy::dotenv()?;
        simple_logger::init_with_level(log::Level::Debug).unwrap();

        let pool = db::init().await?;

        HttpServer::new(move ||
                App::new()
                        .wrap(middleware::Logger::default())
                        .into_utoipa_app()
                        .openapi(ApiDoc::openapi())
                        .app_data(pool.clone())
                        .configure(app_config)
                        .openapi_service(openapi_service_factory)
                        .into_app()
                )
                .workers(*SERVER_WORKERS)
                .bind(*SERVER_ADDRESS)?
                .run()
                .await?;

        Ok(())
}