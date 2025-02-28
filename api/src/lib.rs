mod demo;
mod errors;
pub(crate) mod rsp;
mod endpoint;

use crate::demo::demo_apis;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::error;
use std::net::Ipv4Addr;
use utoipa_actix_web::{scope, AppExt};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let (app, api) = App::new()
            .into_utoipa_app()
            .map(|app| app.wrap(Logger::default()))
            .service(scope::scope("/api").configure(|cfg| {
                demo_apis(cfg);
                endpoint::registers(cfg);
            }))
            .split_for_parts();
        app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?;
    server.run().await?;
    Ok(())
}
pub fn main() {
    flexi_logger::init();
    let result = start();

    if let Some(err) = result.err() {
        error!("Error: {err}");
    }
}
