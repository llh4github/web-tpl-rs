mod endpoint;
pub(crate) mod global;
pub(crate) mod rsp;
pub(crate) mod dto;

use crate::global::AppResources;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use common::cfg::{ApiNetwork, Settings};
use log::error;
use std::net::Ipv4Addr;
use utoipa_actix_web::{scope, AppExt};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn start(cfg: Settings) -> std::io::Result<()> {
    let network = cfg.network;
    let debug = cfg.debug;
    let db_conn = db::db_connection(cfg.database, debug).await.unwrap();
    let resources = AppResources { db: db_conn };

    let server = HttpServer::new(move || {
        let (app, api) = App::new()
            .into_utoipa_app()
            .map(|app| app.wrap(Logger::default()))
            .service(scope::scope(network.prefix.as_str()).configure(|cfg| {
                endpoint::apis(cfg);
            }))
            .app_data(web::Data::new(resources.clone()))
            .split_for_parts();
        app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api))
    })
    .bind((Ipv4Addr::UNSPECIFIED, network.port))?;
    server.run().await?;
    Ok(())
}
pub fn main(cfg: Settings) {
    let _fx_log = flexi_logger::Logger::try_with_env_or_str("debug")
        .unwrap()
        .start()
        .expect("flexi_logger error");
    let result = start(cfg);

    if let Some(err) = result.err() {
        error!("Error: {err}");
    }
}
