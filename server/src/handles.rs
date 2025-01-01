use tokio::sync::Mutex;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

pub async fn handle_connection(
    socket: tokio::net::TcpStream, 
    connections: Arc<Mutex<HashMap<i32, SocketAddr>>>,
    id: i32
) {
    if let Ok(addr) = socket.peer_addr() {
        println!("{} connection succeed", &addr);
        
        let mut conn_map = connections.lock().await;
        conn_map.insert(id, addr);
   
        println!("Active connections: {:?}", *conn_map);
    }
}