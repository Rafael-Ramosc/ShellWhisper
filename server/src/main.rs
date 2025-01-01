mod server_state;
mod handles;

use tokio::net::TcpListener;
use std::sync::Arc;
use server_state::State;
use handles::handle_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_adress = "127.0.0.1:8080".to_string();
    println!("Server adress: {}", &server_adress);
    let listener = TcpListener::bind(server_adress).await?;

    let state = Arc::new(State::new());

    // accept connections
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("{} is connecting...", addr);

        let state = state.clone();
       
        // new task
        tokio::spawn(async move {
            let id = {
                let mut counter = state.id_counter.lock().await;
                let id = *counter;
                *counter += 1;
                id
            };
            handle_connection(socket, state.connecting.clone(), id).await;
        });
    }
}

