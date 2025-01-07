use sqlx::types::{chrono::{DateTime, Utc}, IpAddr};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserIp {
    pub id: i32,
    pub user_id: i32,
    pub ip_address: IpAddr,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

impl UserIp {
    pub fn new(user_id: i32, socket_addr: SocketAddr) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            user_id,
            ip_address: socket_addr.ip(),
            first_seen_at: now,
            last_seen_at: now,
        }
    }

    pub async fn create(self, pool: &PgPool) -> Result<UserIp, sqlx::Error> {
        let user_ip = sqlx::query_as::<_, UserIp>(
            "INSERT INTO chat.user_ip (user_id, ip_address, first_seen_at, last_seen_at) 
             VALUES ($1, $2, $3, $4) 
             ON CONFLICT (user_id, ip_address) 
             DO UPDATE SET last_seen_at = EXCLUDED.last_seen_at 
             RETURNING *"
        )
        .bind(self.user_id)
        .bind(&self.ip_address)
        .bind(self.first_seen_at)
        .bind(self.last_seen_at)
        .fetch_one(pool)
        .await?;

        Ok(user_ip)
    }

    pub async fn update_last_seen(&mut self, pool: &PgPool) -> Result<(), sqlx::Error> {
        self.last_seen_at = Utc::now();

        sqlx::query(
            "UPDATE chat.user_ip 
             SET last_seen_at = $1 
             WHERE id = $2"
        )
        .bind(self.last_seen_at)
        .bind(self.id)
        .execute(pool)
        .await?;

        Ok(())
    }
}