use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub alias: String,
    pub created_at: DateTime<Utc>,
    pub last_login_at: DateTime<Utc>,
    pub status: String,
}

impl User {
    pub fn new(alias: String) -> Self {
        Self {
            id: 0,
            alias,
            created_at: Utc::now(),
            last_login_at: Utc::now(),
            status: "offline".to_string(),
        }
    }

    //TODO: create a verification of user alias, if it is already in use
    pub async fn create(self, pool: &PgPool) -> Result<User, sqlx::Error> {
        let existing_user = sqlx::query_as::<_, User>(
            "UPDATE chat.user 
             SET last_login_at = CURRENT_TIMESTAMP,
                 status = $1
             WHERE alias = $2
             RETURNING id, alias, created_at, last_login_at, status",
        )
        .bind(&self.status)
        .bind(&self.alias)
        .fetch_optional(pool)
        .await?;

        match existing_user {
            Some(user) => Ok(user),
            None => {
                let new_user = sqlx::query_as::<_, User>(
                    "INSERT INTO chat.user (
                        alias, 
                        created_at,
                        last_login_at,
                        status
                    ) 
                    VALUES ($1, $2, CURRENT_TIMESTAMP, $3) 
                    RETURNING id, alias, created_at, last_login_at, status",
                )
                .bind(&self.alias)
                .bind(self.created_at)
                .bind(&self.status)
                .fetch_one(pool)
                .await?;

                Ok(new_user)
            }
        }
    }

    pub async fn set_online(&mut self, pool: &PgPool) -> Result<(), sqlx::Error> {
        self.status = "online".to_string();
        self.last_login_at = Utc::now();

        sqlx::query(
            "UPDATE chat.user 
             SET status = $1, last_login_at = $2 
             WHERE id = $3",
        )
        .bind(&self.status)
        .bind(self.last_login_at)
        .bind(self.id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
