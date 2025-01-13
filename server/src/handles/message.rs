use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Warning,
    Info,
    Error,
    Text,
}

impl ToString for MessageType {
    fn to_string(&self) -> String {
        match self {
            MessageType::Warning => "warning".to_string(),
            MessageType::Info => "info".to_string(),
            MessageType::Error => "error".to_string(),
            MessageType::Text => "text".to_string(),
        }
    }
}

impl From<String> for MessageType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "warning" => MessageType::Warning,
            "info" => MessageType::Info,
            "error" => MessageType::Error,
            _ => MessageType::Text,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<i64>,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub content_type: MessageType,
    pub created_at: Option<NaiveDateTime>,
    pub status: String,
    pub is_encrypted: bool,
}

impl Message {
    pub fn new(sender_id: i32, receiver_id: i32, content: String) -> Self {
        Message {
            id: None,
            sender_id,
            receiver_id,
            content,
            content_type: MessageType::Text,
            created_at: Some(Utc::now().naive_utc()),
            status: "sent".to_string(),
            is_encrypted: false,
        }
    }

    pub async fn insert(&self, pool: &PgPool) -> Result<Message, sqlx::Error> {
        let content_type_str = self.content_type.to_string();

        let record = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO chat.message (
                sender_id, receiver_id, content, content_type, 
                status, is_encrypted
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING 
                id, sender_id, receiver_id, content, 
                content_type as "content_type: String",
                created_at,
                status, is_encrypted
            "#,
            self.sender_id,
            self.receiver_id,
            self.content,
            content_type_str,
            self.status,
            self.is_encrypted
        )
        .fetch_one(pool)
        .await?;

        Ok(Message {
            content_type: MessageType::from(record.content_type),
            ..record
        })
    }
}
