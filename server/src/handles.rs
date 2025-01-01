use std::sync::Arc;
use crate::server_state::State;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};

pub async fn handle_connection(
    mut socket: tokio::net::TcpStream, 
    state: Arc<State>,
    id: u32
) -> Result<std::net::SocketAddr> {

    let addr = socket.peer_addr().unwrap();

    if !state.can_accept_connection().await {
        println!("Max connections reached"); //TODO: send message to client, maybe create a error kind for server
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
        // println!("{:?} is reading...", &buffer[..n]);
        match socket.read(&mut buffer).await {
            Ok(0) => {
                let mut conn_map = state.connection_list.lock().await;
                conn_map.remove(&id);
                break;
            }
            Ok(n) => {
                println!("{} bytes read", n);
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