use std::sync::Arc;
use crate::server_state::State;

pub async fn handle_connection(
    socket: tokio::net::TcpStream, 
    state: Arc<State>,
    id: u32
) {

    if !state.can_accept_connection().await {
        println!("Max connections reached");
        return;
    }

    if let Ok(addr) = socket.peer_addr() {
        println!("{} connection succeed", &addr);
        
        let mut conn_map = state.connection_list.lock().await;
        conn_map.insert(id, addr);
   
        println!("Active connections: {:?}", *conn_map);
    }
}