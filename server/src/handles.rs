use std::sync::Arc;
use crate::server_state::State;

pub async fn handle_connection(
    socket: tokio::net::TcpStream, 
    state: Arc<State>,
    id: u32
) {

    if !state.can_accept_connection().await {
        println!("Max connections reached"); //TODO: send message to client, maybe create a error kind for server
        return;
    }

    if let Ok(addr) = socket.peer_addr() {
        println!("{} connection succeed", &addr);
        
        let mut conn_map = state.connection_list.lock().await;
        conn_map.insert(id, addr);
   
        println!("Active connections: {:?}", *conn_map);
    }
}

//TODO: criar um função que detecta se a coneção foi fechada pelo cliente e remove o id da lista de conexões ativas
// estudar: stream.read, stream.peek (TcpStream)