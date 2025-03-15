use crate::RedisConnectionManager;
use common::cfg::RedisMode;
use r2d2::ManageConnection;
use redis::cluster::ClusterClient;
use redis::{cluster, ConnectionLike, RedisError};

impl ManageConnection for RedisConnectionManager {
    type Connection = cluster::ClusterConnection;
    type Error = RedisError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match &self.config {
            RedisMode::Standalone { node } => {
                unimplemented!()
            }
            RedisMode::Cluster { nodes } => {
                let client = ClusterClient::new(nodes.clone())?;
                let conn = client.get_connection()?;
                // Ok(Box::new(conn))
                Ok(conn)
            }
        }
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let _: () = redis::cmd("PING").query(conn)?;
        Ok(())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_open()
    }
}
