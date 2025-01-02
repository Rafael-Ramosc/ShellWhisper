use std::sync::Arc;
use crate::server_state::State;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use crate::server_error::ServerError;

pub async fn handle_connection(
    mut socket: tokio::net::TcpStream, 
    state: Arc<State>,
    id: u32
) -> Result<std::net::SocketAddr> {

    let addr = socket.peer_addr().unwrap();

    if !state.can_accept_connection().await {
        let error = ServerError::max_connections_reached();
        println!("{} connection failed: {}", addr, error);
        return Ok(addr);
    }

    if let Ok(addr) = socket.peer_addr() {
        println!("{} connection succeed", &addr);
        
        let mut conn_map = state.connection_list.lock().await;
        conn_map.insert(id, addr);
        let client_total = conn_map.len();
   
        println!("Active connections ({client_total}): {:?}", *conn_map);
    }

    let mut buffer = [0; 1024];
    

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                let mut conn_map = state.connection_list.lock().await;
                conn_map.remove(&id);
                break;
            }
            Ok(n) => {
                let text = String::from_utf8_lossy(&buffer[..n]);
                println!("{}: {}", addr, text);
                socket.write_all(&buffer[..n]).await?;
            }
            Err(e) => {
                println!("Error reading data: {:?}", e);
                break;
            }
        }
    }

    Ok(addr)

}