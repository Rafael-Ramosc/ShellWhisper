use super::user::User;
use super::user_ip::UserIp;
use crate::server_error::ServerError;
use crate::server_state::State;
use sqlx::{Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};

pub async fn handle_connection(
    mut socket: tokio::net::TcpStream,
    state: Arc<State>,
    id: u32,
) -> Result<std::net::SocketAddr> {
    let addr = socket.peer_addr().unwrap();

    if !state.can_accept_connection().await {
        let error = ServerError::max_connections_reached();
        println!("{} connection failed: {}", addr, error);
        return Ok(addr);
    }

    if let Ok(addr) = socket.peer_addr() {
        println!("{} connection succeed", &addr);

        user_connection_db(&state.db_pool, &addr).await;

        let mut conn_map = state.connection_list.lock().await;
        conn_map.insert(id, addr);
        let client_total = conn_map.len();

        println!("Active connections ({client_total}): {:?}", *conn_map);

        let welcome_message = "Connection establish!".as_bytes();
        socket.write_all(welcome_message).await?;
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
                print!("{}: {}", addr, text);
            }
            Err(e) => {
                println!("Error reading data: {:?}", e);
                break;
            }
        }
    }

    Ok(addr)
}

async fn user_connection_db(db_pool: &Pool<Postgres>, addr: &SocketAddr) {
    let user = User::new("falano de tal 8".to_string());
    let user_created = user.create(db_pool).await.expect("Failed to create user");

    let user_ip = UserIp::new(user_created.id, *addr);
    user_ip
        .create(db_pool)
        .await
        .expect("Failed to create user ip");
}
