mod server_state;
mod handles;
mod server_error;

use tokio::net::TcpListener;
use std::sync::Arc;
use server_state::State;
use handles::handle_connection;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_adress = "127.0.0.1:8080".to_string();
    println!("Server adress: {}", &server_adress);
    let listener = TcpListener::bind(server_adress).await?; 

    let server_limit_connections: u32 = 30;
    let state = Arc::new(State::new(server_limit_connections));

    // accept connections
    loop {
        let (socket, addr) = listener.accept().await?; //TODO: handle error kind
        println!("{} is connecting...", addr);

        let state = state.clone();
       
        // new task
        tokio::spawn(async move {
           
            let id = State::id_increment(&state).await;
            match handle_connection(socket, state.clone(), id).await {
                Ok(addr) => println!("{} Connection closed", addr),
                Err(e) => println!("Error: {:?}", e),
            };
        });
    }
}

