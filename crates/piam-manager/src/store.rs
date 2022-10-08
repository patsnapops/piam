use redis::Commands;

use crate::config::REDIS_ADDRESS;

pub async fn get_resource_string(key: &str) -> String {
    let client = redis::Client::open(REDIS_ADDRESS.load().as_str()).unwrap();
    let mut con = client.get_connection().unwrap();
    con.get(format!("piam:{}", key)).unwrap()
}
