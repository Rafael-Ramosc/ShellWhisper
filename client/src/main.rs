use std::error::Error;
use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const MESSAGE_TYPE_WARNING: u8 = 0x02; //just for test

    let server_address = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(server_address).await?;
    println!("Connected {}", server_address);

    let mut input = String::new();

    loop {
        input.clear();

        match receive_message(&mut stream).await {
            Ok(received) => {
                println!("SERVER: {}", received);
            }
            Err(e) => {
                eprintln!("Erro ao ler do servidor: {}", e);
                break;
            }
        }

        print!("Digite sua mensagem: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let message = input.trim();
        let mensage_with_type: String = format!("0x{:02x} | {}", MESSAGE_TYPE_WARNING, message);
        send_message(&mut stream, message).await?;
        println!("Mensagem enviada: {}", mensage_with_type); //dont work
    }

    Ok(())
}

async fn send_message(stream: &mut TcpStream, message: &str) -> Result<(), Box<dyn Error>> {
    let message_with_delimiter = format!("{}\n", message);
    stream.write_all(message_with_delimiter.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

async fn receive_message(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        return Err("Conexão fechada pelo servidor".into());
    }
    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}
