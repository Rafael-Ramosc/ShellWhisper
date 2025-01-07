use sqlx::types::chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub alias: String,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub status: String,
}

impl User {
    pub fn new(alias: String) -> Self {
        Self {
            id: 0,
            alias,
            created_at: Utc::now(),
            last_login_at: None,
            status: "offline".to_string(),
        }
    }

    pub async fn create(self, pool: &PgPool) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO chat.user (alias, created_at, status) 
             VALUES ($1, $2, $3) 
             RETURNING *"
        )
        .bind(&self.alias)
        .bind(self.created_at)
        .bind(&self.status)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn set_online(&mut self, pool: &PgPool) -> Result<(), sqlx::Error> {
        self.status = "online".to_string();
        self.last_login_at = Some(Utc::now());

        sqlx::query(
            "UPDATE chat.user 
             SET status = $1, last_login_at = $2 
             WHERE id = $3"
        )
        .bind(&self.status)
        .bind(self.last_login_at)
        .bind(self.id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
