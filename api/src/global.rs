use sea_orm::DatabaseConnection;

/// Application resources
#[derive(Debug, Clone)]
pub struct AppResources {
    pub db: DatabaseConnection,
}
