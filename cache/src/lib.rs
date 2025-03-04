use redis::cluster::ClusterClient;
use redis::Client;

fn a() -> Result<ClusterClient, redis::RedisError> {
    let a = ClusterClient::new(vec![""])?;
    let b = Client::open("")?;
    let c = b.get_connection()?;
    Ok(a)
}