use crate::RedisConnectionManager;
use common::cfg::RedisMode;
use r2d2::ManageConnection;
use redis::{Client, ConnectionLike, RedisError};

#[cfg(feature = "standalone")]
impl ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = RedisError;
    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match &self.config {
            RedisMode::Standalone { node } => {
                // let client = Client::open(node.to_string())?;
                // let conn = client.get_connection()?;
                let client = Client::open(node.to_string())?;
                let conn = client.get_connection()?;
                Ok(conn)
            }
            RedisMode::Cluster { .. } => {
                unimplemented!()
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
