use server::server_state::State;
use std::{net::SocketAddr, sync::Arc};

#[tokio::test]
async fn test_new_state() {
    let state = State::new(1);
    
    assert!(state.can_accept_connection().await, "Should accept connections initially");
    
    let connections = state.connection_list.lock().await;
    assert_eq!(connections.len(), 0, "Connection list should start empty");
    
    let counter = state.id_counter.lock().await;
    assert_eq!(*counter, 0, "Counter should start at zero");
}

#[tokio::test]
async fn test_can_accept_connection() {
    let state = State::new(1);
    
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
    let state = State::new(2); 
    
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
    let state = State::new(1);
    
    assert_eq!(state.id_increment().await, 0, "First ID should be 0");
    assert_eq!(state.id_increment().await, 1, "Second ID should be 1");
    assert_eq!(state.id_increment().await, 2, "Third ID should be 2");
}

#[tokio::test]
async fn test_concurrent_id_increment() {
    let state = Arc::new(State::new(1));
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
    let state = State::new(1);
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