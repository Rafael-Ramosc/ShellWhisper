use tokio::sync::Mutex;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};


pub struct State {
    pub connection_list: Arc<Mutex<HashMap<u32, SocketAddr>>>,
    pub id_counter: Arc<Mutex<u32>>,
    max_connections: usize, 
}

impl State {
    pub fn new() -> Self {
        State {
            connection_list: Arc::new(Mutex::new(HashMap::new())),
            id_counter: Arc::new(Mutex::new(0)),
            max_connections: 30, //TODO: receber esse id da interface na hora de criar o servidor
        }
    }

    pub async fn can_accept_connection(&self) -> bool {
        let connections = self.connection_list.lock().await;
        connections.len() < self.max_connections
    }

    pub async fn id_increment(&self) -> u32 {
        let mut counter = self.id_counter.lock().await;
        let id = *counter;
        *counter += 1;
        id
    }
}