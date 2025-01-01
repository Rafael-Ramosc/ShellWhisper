use tokio::sync::Mutex;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};


pub struct State {
    pub connecting: Arc<Mutex<HashMap<i32, SocketAddr>>>,
    pub id_counter: Arc<Mutex<i32>>,
}

impl State {
    pub fn new() -> Self {
        State {
            connecting: Arc::new(Mutex::new(HashMap::new())),
            id_counter: Arc::new(Mutex::new(0)),
        }
    }
}