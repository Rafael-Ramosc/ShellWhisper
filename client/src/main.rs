use std::error::Error;
use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const MESSAGE_TYPE_TEXT: u8 = 0x04;
    const MESSAGE_TYPE_ALIAS: u8 = 0x05;

    let server_address = "127.0.0.1:8080";
    let stream = TcpStream::connect(server_address).await?;

    println!("Connected {}", server_address);

    let (mut read_half, mut write_half) = tokio::io::split(stream);

    let alias = match choose_alias().await {
        Ok(alias) => alias,
        Err(e) => {
            eprintln!("Error choosing alias: {}", e);
            return Ok(());
        }
    };

    let create_alias = format!("0x{:02x} | {}", MESSAGE_TYPE_ALIAS, alias);
    send_message(&mut write_half, &create_alias).await?;

    loop {
        select! {
            result = receive_message(&mut read_half) => {
                match result {
                    Ok(received) => {
                        println!("\nSERVER: {}", received);
                    }
                    Err(e) => {
                        eprintln!("Error when reading server: {}", e);
                        break;
                    }
                }
            }

            result = async {
                let mut input = String::new();
                print!("Send a message: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                let message = input.trim();
                if !message.is_empty() {
                    let message_with_type = format!("0x{:02x} | {}", MESSAGE_TYPE_TEXT, message);
                    send_message(&mut write_half, &message_with_type).await?;
                }
                Ok::<(), Box<dyn Error>>(())
            } => {
                if let Err(e) = result {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn receive_message(
    stream: &mut tokio::io::ReadHalf<TcpStream>,
) -> Result<String, Box<dyn Error>> {
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        return Err("Connection closed by server".into());
    }
    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}

async fn send_message(
    stream: &mut tokio::io::WriteHalf<TcpStream>,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    let message_with_delimiter = format!("{}\n", message);
    stream.write_all(message_with_delimiter.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

async fn choose_alias() -> Result<String, Box<dyn Error>> {
    let mut alias = String::new();
    print!("Choose an alias: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut alias)?;
    Ok(alias.trim().to_string())
}
