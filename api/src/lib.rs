pub(crate) mod dto;
mod endpoint;
pub(crate) mod global;
mod middleware;
pub(crate) mod rsp;
mod util;

use actix_web::middleware::Logger;
use actix_web::rt::net;
use actix_web::{App, HttpServer, web};
use cache::create_redis_pool;
use common::cfg::AppCfgs;
use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Naming};
use log::{error, info};
use std::error::Error;
use std::net::Ipv4Addr;
use utoipa_actix_web::{AppExt, scope};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn start(cfg: &AppCfgs) -> Result<(), Box<dyn Error>> {
    let db_conn = db::db_connection(&cfg.database).await?;
    let redis_pool = create_redis_pool(&cfg.redis)?;
    let setting = cfg.clone();
    let port = cfg.network.port;

    let server = HttpServer::new(move || {
        let db_conn = db_conn.clone();
        let redis_pool = redis_pool.clone();
        let setting = setting.clone();
        let network = setting.network.clone();

        let (app, api) = App::new()
            .app_data(web::Data::new(setting))
            .app_data(web::Data::new(db_conn))
            .app_data(web::Data::new(redis_pool))
            .wrap(middleware::Jwt)
            .into_utoipa_app()
            .map(|app| app.wrap(Logger::default()))
            .service(scope::scope(network.prefix.as_str()).configure(|cfg| {
                endpoint::apis(cfg);
            }))
            .split_for_parts();
        app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api))
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?;
    server.run().await?;
    Ok(())
}

pub fn main(cfg: &AppCfgs) {
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
