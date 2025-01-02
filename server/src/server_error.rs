pub enum ServerErrorKind {
   MaxConnectionsReached,
}

pub struct ServerError {
    kind: ServerErrorKind,
    message: String,
    code: u16,
}

impl ServerError {
    pub fn new(kind: ServerErrorKind, message: String, code: u16) -> Self {
        ServerError {
            kind,
            message,
            code,
        }
    }

    pub fn kind(&self) -> &ServerErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn max_connections_reached() -> Self {
        Self::new(
            ServerErrorKind::MaxConnectionsReached,
            "Max connections reached".to_string(),
            100
        )
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}