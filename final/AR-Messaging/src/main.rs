use actix_web::{App, HttpServer, Responder};
use message_controller::{get_all, get_message_thread, ApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod message_controller;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(get_all)
            .service(get_message_thread)
    })
    .bind("127.0.0.1:8006")?
    .run()
    .await
}
