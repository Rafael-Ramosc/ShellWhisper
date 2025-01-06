use tokio::sync::Mutex;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use sqlx::PgPool;


pub struct State {
    pub connection_list: Arc<Mutex<HashMap<u32, SocketAddr>>>,
    pub id_counter: Arc<Mutex<u32>>,
    max_connections: u32,
    pub db_pool: PgPool,
}

impl State {

    pub async fn new(server_limit_connection: u32, database_url: &str) ->  Result<Self, sqlx::Error> {

        let pool = PgPool::connect(database_url).await?;

        Ok(State {
            connection_list: Arc::new(Mutex::new(HashMap::new())),
            id_counter: Arc::new(Mutex::new(0)),
            max_connections: server_limit_connection,
            db_pool: pool,
        })
    }

    pub async fn can_accept_connection(&self) -> bool {
        let connections = self.connection_list.lock().await;
        connections.len() < self.max_connections as usize
    }

    pub async fn id_increment(&self) -> u32 {
        let mut counter = self.id_counter.lock().await;
        let id = *counter;
        *counter += 1;
        id
    }

    pub async fn test_connection(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.db_pool).await?;
        Ok(())
    }

}