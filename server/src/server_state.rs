use tokio::sync::Mutex;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};


pub struct State {
    pub connection_list: Arc<Mutex<HashMap<u32, SocketAddr>>>,
    pub id_counter: Arc<Mutex<u32>>,
    max_connections: u32, 
}

impl State {
    pub fn new(server_limit_connection: u32) -> Self {
        State {
            connection_list: Arc::new(Mutex::new(HashMap::new())),
            id_counter: Arc::new(Mutex::new(0)),
            max_connections: server_limit_connection,
        }
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
}