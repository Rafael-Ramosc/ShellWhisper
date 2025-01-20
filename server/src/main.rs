mod handles;
mod server_state;
mod ui;

use dotenv::dotenv;
use handles::connection_handles::handle_connection;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::EnableMouseCapture,
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use server_state::State;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server_adress = "127.0.0.1:8080".to_string();
    println!("Server adress: {}", &server_adress);
    let listener = TcpListener::bind(server_adress).await?;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let server_limit_connections: u32 = 30;
    let state = Arc::new(
        State::new(server_limit_connections, &database_url)
            .await
            .expect("Failed to create state with database connection"),
    );

    state
        .test_connection()
        .await
        .expect("Failed to connect to database");

    println!("Successfully connected to database");

    let server_state = state.clone();

    let server_task = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    println!("{} is connecting...", addr);

                    let state = server_state.clone();

                    tokio::spawn(async move {
                        let id = State::id_increment(&state).await;
                        match handle_connection(socket, state.clone(), id).await {
                            Ok((addr, mut user)) => {
                                if let Err(e) = user.set_status(&state.db_pool).await {
                                    println!("Error updating user status: {:?}", e);
                                }
                                println!("{} Connection closed", user.alias)
                            }
                            Err(e) => println!("Error: {:?}", e),
                        };
                    });
                }
                Err(e) => {
                    println!("Error accepting connection: {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
        }
    });

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    ui::run_app(&mut terminal, state.clone()).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        EnableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
