use server::server_error::{ServerError, ServerErrorKind};

#[test]
fn test_error_creation() {
    let error = ServerError::new(
        ServerErrorKind::MaxConnectionsReached,
        "Test message".to_string(),
        123
    );
    
    assert_eq!(error.code(), 123);
    assert_eq!(error.message(), "Test message");
    matches!(error.kind(), ServerErrorKind::MaxConnectionsReached);
}

#[test]
fn test_max_connections_reached_factory() {
    let error = ServerError::max_connections_reached();
    
    assert_eq!(error.code(), 100);
    assert_eq!(error.message(), "Max connections reached");
    matches!(error.kind(), ServerErrorKind::MaxConnectionsReached);
}

#[test]
fn test_display_formatting() {
    let error = ServerError::new(
        ServerErrorKind::MaxConnectionsReached,
        "Test error".to_string(),
        500
    );
    
    assert_eq!(format!("{}", error), "[500] Test error");
}

#[test]
fn test_error_getters() {
    let error = ServerError::new(
        ServerErrorKind::MaxConnectionsReached,
        "Test message".to_string(),
        404
    );
    
    // Testa cada getter individualmente
    assert_eq!(error.code(), 404);
    assert_eq!(error.message(), "Test message");
    assert!(matches!(error.kind(), &ServerErrorKind::MaxConnectionsReached));
}