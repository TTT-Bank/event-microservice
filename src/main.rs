mod infrastructure;
mod handles;
mod error;
mod domain;

use std::{net::SocketAddr, sync::LazyLock};

use actix_web::{App, HttpServer, middleware, web::Data};
use utoipa_actix_web::{AppExt, scope, service_config::ServiceConfig};
use error::Result;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::utils::Offset;

fn app_config(cfg: &mut ServiceConfig) {
        cfg
        .service(scope::scope("/api/v1")
                .service(handles::ping)
                .configure(handles::user::user_app_config)
                .configure(handles::event::event_app_config)
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
#[openapi(components(schemas(Offset)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<()> {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        dotenvy::dotenv().ok();

        let provider = infrastructure::provider::PgProvider::new().await?;

        HttpServer::new(move ||
                App::new()
                        .wrap(middleware::Logger::default())
                        .into_utoipa_app()
                        .openapi(ApiDoc::openapi())
                        .app_data(Data::new(provider.clone()))
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