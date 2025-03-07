use common::cfg::RedisMode;
use r2d2::{ManageConnection, Pool};
use redis::cluster::ClusterClient;
use redis::{Client, ConnectionLike, RedisError};
use std::{fmt, io};

/// 创建 Redis 连接池
pub fn create_redis_pool(config: RedisMode) -> Result<Pool<RedisConnectionManager>, RedisError> {
    let manager = RedisConnectionManager::new(config);
    let rs = Pool::builder().build(manager);
    if let Err(e) = &rs {
        Err(RedisError::from(io::Error::new(
            io::ErrorKind::NotConnected,
            e.to_string(),
        )))
    } else {
        Ok(rs.unwrap())
    }
}

/// 统一连接类型：兼容单节点和集群连接
type UnifiedConnection = Box<dyn ConnectionLike + Send + 'static>;

/// 自定义连接管理器
pub struct RedisConnectionManager {
    config: RedisMode,
}
impl RedisConnectionManager {
    pub fn new(config: RedisMode) -> Self {
        Self { config }
    }
}
impl fmt::Debug for RedisConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RedisConnectionManager")
    }
}

impl ManageConnection for RedisConnectionManager {
    type Connection = UnifiedConnection;
    type Error = RedisError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match &self.config {
            RedisMode::Standalone { node } => {
                let client = Client::open(node.to_string())?;
                let conn = client.get_connection()?;
                Ok(Box::new(conn))
            }
            RedisMode::Cluster { nodes } => {
                let client = ClusterClient::new(nodes.clone())?;
                let conn = client.get_connection()?;
                Ok(Box::new(conn))
            }
        }
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let _: () = redis::cmd("PING").query(conn.as_mut())?;
        Ok(())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_open()
    }
}

