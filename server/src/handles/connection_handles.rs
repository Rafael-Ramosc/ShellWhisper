use super::message::{Message, MessageType};
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
) -> Result<(std::net::SocketAddr, User)> {
    let addr = socket.peer_addr().unwrap();

    if !state.can_accept_connection().await {
        let error = ServerError::max_connections_reached();
        println!("{} connection failed: {}", addr, error);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Max connections reached",
        ));
    }

    let mut buffer = [0; 1024];
    let user = read_alias(&mut buffer, &mut socket, &state, &addr).await?;

    let user_alias = user.alias.clone();

    if let Ok(addr) = socket.peer_addr() {
        println!("{} connection succeed", &addr);

        // Store the created user
        Some(user_connection_db(&state.db_pool, &addr, &user_alias).await);

        let mut conn_map = state.connection_list.lock().await;
        conn_map.insert(id, addr);
        let client_total = conn_map.len();

        println!("Active connections ({client_total}): {:?}", *conn_map);

        let welcome_message = "Connection establish!".as_bytes();
        socket.write_all(welcome_message).await?;
    }

    let user_id = user.id;

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                let mut conn_map = state.connection_list.lock().await;
                conn_map.remove(&id);
                break;
            }
            Ok(n) => {
                let message = Message::from_buffer(&buffer, n, user_id, 1);
                let message = message
                    .insert(&state.db_pool)
                    .await
                    .expect("Failed to insert message");

                println!(
                    "{}: {} (Type: {:?})",
                    user_alias, message.content, message.content_type
                );
            }
            Err(e) => {
                println!("Error reading data: {:?}", e);
                break;
            }
        }
    }

    Ok((addr, user))
}

async fn user_connection_db(
    db_pool: &Pool<Postgres>,
    addr: &SocketAddr,
    user_selected_alias: &String,
) -> User {
    let user = User::new(user_selected_alias.to_string());
    let user_created = user.create(db_pool).await.expect("Failed to create user");

    let user_ip = UserIp::new(user_created.id, *addr);
    user_ip
        .create(db_pool)
        .await
        .expect("Failed to create user ip");

    user_created
}

// async fn receive_message(n: usize, buffer: &[u8], user_id: i32, pool: &Pool<Postgres>) -> String {
//     let text = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

//     // Insere a mensagem no banco de dados
//     insert_message(text.clone(), user_id, pool).await;

//     text
// }

// async fn insert_message(text: String, sender_id: i32, pool: &Pool<Postgres>) {
//     let message = Message::new(sender_id, 1, text);
//     message
//         .insert(pool)
//         .await
//         .expect("Failed to insert message");
// }

async fn read_alias(
    buffer: &mut [u8],
    socket: &mut tokio::net::TcpStream,
    state: &Arc<State>,
    addr: &SocketAddr,
) -> Result<User> {
    match socket.read(buffer).await {
        Ok(n) => {
            let message = Message::from_buffer(buffer, n, 0, 1);
            if message.content_type == MessageType::Alias {
                let user = user_connection_db(&state.db_pool, addr, &message.content).await;

                let welcome_message = "Connection established!".as_bytes();
                socket.write_all(welcome_message).await?;

                Ok(user)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "First message must be alias",
                ))
            }
        }
        Err(e) => {
            println!("Error reading alias: {:?}", e);
            Err(e)
        }
    }
}
