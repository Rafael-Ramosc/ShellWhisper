use server::server_state::State;
use std::{net::SocketAddr, sync::Arc};
use dotenv::dotenv;
use std::env;

async fn setup_test_state(limit: u32) -> State {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    State::new(limit, &database_url)
        .await
        .expect("Failed to create test state")
}

#[tokio::test]
async fn test_new_state() {
    let state = setup_test_state(1).await;
    
    assert!(state.can_accept_connection().await, "Should accept connections initially");
    
    let connections = state.connection_list.lock().await;
    assert_eq!(connections.len(), 0, "Connection list should start empty");
    
    let counter = state.id_counter.lock().await;
    assert_eq!(*counter, 0, "Counter should start at zero");

    // Test database connection
    assert!(state.test_connection().await.is_ok(), "Database connection should be successful");
}

#[tokio::test]
async fn test_can_accept_connection() {
    let state = setup_test_state(1).await;
    
    assert!(state.can_accept_connection().await, "Should accept first connection");
    
    {
        let mut connections = state.connection_list.lock().await;
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        connections.insert(1, addr);
    }
    
    assert!(!state.can_accept_connection().await, "Should not accept more connections after limit");
}

#[tokio::test]
async fn test_multiple_connections() {
    let state = setup_test_state(2).await;
    
    assert!(state.can_accept_connection().await, "Should accept first connection");
    
    {
        let mut connections = state.connection_list.lock().await;
        let addr1: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        connections.insert(1, addr1);
    }
    
    assert!(state.can_accept_connection().await, "Should accept second connection");
    
    {
        let mut connections = state.connection_list.lock().await;
        let addr2: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        connections.insert(2, addr2);
    }
    
    assert!(!state.can_accept_connection().await, "Should not accept third connection");
}

#[tokio::test]
async fn test_id_increment() {
    let state = setup_test_state(1).await;
    
    assert_eq!(state.id_increment().await, 0, "First ID should be 0");
    assert_eq!(state.id_increment().await, 1, "Second ID should be 1");
    assert_eq!(state.id_increment().await, 2, "Third ID should be 2");
}

#[tokio::test]
async fn test_concurrent_id_increment() {
    let state = Arc::new(setup_test_state(1).await);
    let state_clone = state.clone();

    let handle1 = tokio::spawn(async move {
        state.id_increment().await
    });

    let handle2 = tokio::spawn(async move {
        state_clone.id_increment().await
    });

    let id1 = handle1.await.unwrap();
    let id2 = handle2.await.unwrap();

    assert_ne!(id1, id2, "Concurrent IDs should be different");
    assert!(id1 == 0 || id1 == 1, "First ID should be 0 or 1");
    assert!(id2 == 0 || id2 == 1, "Second ID should be 0 or 1");
}

#[tokio::test]
async fn test_connection_management() {
    let state = setup_test_state(1).await;
    let addr1: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let addr2: SocketAddr = "127.0.0.1:8081".parse().unwrap();

    assert!(state.can_accept_connection().await, "Should accept connections initially");

    {
        let mut connections = state.connection_list.lock().await;
        connections.insert(1, addr1);
        assert_eq!(connections.len(), 1, "Should have exactly one connection");
    }

    assert!(!state.can_accept_connection().await, "Should not accept more connections");

    {
        let mut connections = state.connection_list.lock().await;
        connections.remove(&1);
        assert_eq!(connections.len(), 0, "Should be empty after removal");
    }

    assert!(state.can_accept_connection().await, "Should accept connections after removal");

    {
        let mut connections = state.connection_list.lock().await;
        connections.insert(2, addr2);
        assert_eq!(connections.len(), 1, "Should have exactly one connection");
    }
}

#[tokio::test]
async fn test_database_connection() {
    let state = setup_test_state(1).await;
    
    // Test the database connection
    let result = state.test_connection().await;
    assert!(result.is_ok(), "Database connection test should succeed");
}