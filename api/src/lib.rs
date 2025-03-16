pub(crate) mod dto;
mod endpoint;
pub(crate) mod global;
mod middleware;
pub(crate) mod rsp;
mod util;

use crate::global::AppResources;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use cache::create_redis_pool;
use common::cfg::Settings;
use flexi_logger::{
    Age, Cleanup, Criterion, Duplicate, FileSpec, FlexiLoggerError, LevelFilter, Naming,
};
use log::{error, info};
use std::error::Error;
use std::net::Ipv4Addr;
use utoipa_actix_web::{scope, AppExt};
use utoipa_swagger_ui::SwaggerUi;
#[actix_web::main]
async fn start(cfg: Settings) -> Result<(), Box<dyn Error>> {
    let debug = cfg.debug;
    let db_conn = db::db_connection(&cfg.database, debug).await?;
    let redis_pool = create_redis_pool(&cfg.redis)?;
    let resources = AppResources {
        db: db_conn,
        redis_pool,
        cfg: cfg.clone(),
    };

    let network = cfg.network;
    let server = HttpServer::new(move || {
        let (app, api) = App::new()
            .app_data(web::Data::new(resources.clone()))
            .wrap(middleware::Jwt)
            .into_utoipa_app()
            .map(|app| app.wrap(Logger::default()))
            .service(scope::scope(network.prefix.as_str()).configure(|cfg| {
                endpoint::apis(cfg);
            }))
            .split_for_parts();
        app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api))
    })
    .bind((Ipv4Addr::UNSPECIFIED, network.port))?;
    server.run().await?;
    Ok(())
}

pub fn main(cfg: Settings) {
    flexi_logger::Logger::try_with_env_or_str("debug")
        .unwrap()
        .rotate(
            Criterion::Age(Age::Day),
            Naming::TimestampsCustomFormat {
                current_infix: None,
                format: "%Y%m%d",
            },
            Cleanup::KeepLogFiles(7),
        )
        .log_to_file(FileSpec::default().directory("logs").basename("web-tpl"))
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .expect("flexi_logger error");

    info!("Starting server ...");
    let result = start(cfg);
    if let Some(err) = result.err() {
        error!("Error: {err}");
    }
}
