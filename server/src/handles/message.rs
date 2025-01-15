use super::user::User;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

const MESSAGE_TYPE_ERROR: u8 = 0x01;
const MESSAGE_TYPE_WARNING: u8 = 0x02;
const MESSAGE_TYPE_INFO: u8 = 0x03;
const MESSAGE_TYPE_TEXT: u8 = 0x04;
const MESSAGE_TYPE_ALIAS: u8 = 0x05;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Warning,
    Info,
    Error,
    Text,
    Alias,
}

impl MessageType {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            MESSAGE_TYPE_ERROR => MessageType::Error,
            MESSAGE_TYPE_WARNING => MessageType::Warning,
            MESSAGE_TYPE_INFO => MessageType::Info,
            MESSAGE_TYPE_ALIAS => MessageType::Alias,
            _ => MessageType::Text,
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            MessageType::Error => MESSAGE_TYPE_ERROR,
            MessageType::Warning => MESSAGE_TYPE_WARNING,
            MessageType::Info => MESSAGE_TYPE_INFO,
            MessageType::Text => MESSAGE_TYPE_TEXT,
            MessageType::Alias => MESSAGE_TYPE_ALIAS,
        }
    }
}

impl ToString for MessageType {
    fn to_string(&self) -> String {
        match self {
            MessageType::Warning => "warning".to_string(),
            MessageType::Info => "info".to_string(),
            MessageType::Error => "error".to_string(),
            MessageType::Text => "text".to_string(),
            MessageType::Alias => "alias".to_string(),
        }
    }
}

impl From<String> for MessageType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "warning" => MessageType::Warning,
            "info" => MessageType::Info,
            "error" => MessageType::Error,
            "alias" => MessageType::Alias,
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

    pub fn from_buffer(buffer: &[u8], n: usize, sender_id: i32, receiver_id: i32) -> Self {
        let text = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

        let parts: Vec<&str> = text.splitn(2, "|").collect();

        let (message_type, content) = match parts.as_slice() {
            [type_str, content] => {
                let cleaned_type = type_str
                    .trim()
                    .replace(" ", "")
                    .trim_start_matches("0x")
                    .to_lowercase();

                let type_byte = match cleaned_type.as_str() {
                    "01" | "1" => MESSAGE_TYPE_ERROR,
                    "02" | "2" => MESSAGE_TYPE_WARNING,
                    "03" | "3" => MESSAGE_TYPE_INFO,
                    "05" | "5" => MESSAGE_TYPE_ALIAS,
                    _ => MESSAGE_TYPE_TEXT,
                };

                (
                    MessageType::from_byte(type_byte),
                    content.trim().to_string(),
                )
            }
            _ => (MessageType::Text, text),
        };

        Message {
            id: None,
            sender_id,
            receiver_id,
            content,
            content_type: message_type,
            created_at: Some(Utc::now().naive_utc()),
            status: "sent".to_string(),
            is_encrypted: false,
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let formatted = format!("0x{:02x} | {}", self.content_type.to_byte(), self.content);
        formatted.into_bytes()
    }

    pub fn server_info(info_message: String, user: &User) -> Vec<u8> {
        let server_info = Message {
            id: None,
            sender_id: 1,
            receiver_id: user.id,
            content: info_message,
            content_type: MessageType::Info,
            created_at: Some(Utc::now().naive_utc()),
            status: "sent".to_string(),
            is_encrypted: false,
        };

        server_info.to_buffer()
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
