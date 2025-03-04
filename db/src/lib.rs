use common::cfg;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub mod entities;

pub async fn db_connection(
    cfg: cfg::Database,
    show_sql: bool,
) -> Result<DatabaseConnection, DbErr> {
    let conn = cfg.connection_string();
    let mut opt = ConnectOptions::new(conn);
    opt.sqlx_logging(show_sql);
    Database::connect(opt).await
}
