use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIp {
    pub id: i32,
    pub user_id: i32,
    pub ip_address: IpAddr,
    pub created_at: NaiveDateTime,
}

impl<'r> sqlx::FromRow<'r, PgRow> for UserIp {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(UserIp {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            ip_address: row.try_get("ip_address")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl UserIp {
    pub fn new(user_id: i32, socket_addr: SocketAddr) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: 0,
            user_id,
            ip_address: socket_addr.ip(),
            created_at: now,
        }
    }

    pub async fn create(self, pool: &PgPool) -> Result<UserIp, sqlx::Error> {
        let user_ip = sqlx::query_as::<_, UserIp>(
            "INSERT INTO chat.user_ip (user_id, ip_address,created_at) 
             VALUES ($1, $2::inet, $3) 
             RETURNING *",
        )
        .bind(self.user_id)
        .bind(self.ip_address.to_string())
        .bind(self.created_at)
        .fetch_one(pool)
        .await?;

        Ok(user_ip)
    }
}
