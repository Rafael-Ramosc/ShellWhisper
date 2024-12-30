use tokio::net::TcpListener;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_adress = "127.0.0.1:8080".to_string();
    println!("Server adress: {}", &server_adress);
    let listener = TcpListener::bind(server_adress).await?;

    let connections = Arc::new(Mutex::new(HashMap::new())); 
    let mut id_counter = 0;

    // accept connections
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("{} is connecting...", addr);

        let connections = connections.clone();
        let id = id_counter;
        id_counter += 1;

        // new task
        tokio::spawn(async move {
            handle_connection(socket, connections, id).await;
        });
    }
}

async fn handle_connection(
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