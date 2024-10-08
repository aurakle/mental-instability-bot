use std::sync::OnceLock;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

use crate::get_config;

pub(crate) type Db = Pool<Postgres>;
pub(crate) type Result<T> = core::result::Result<T, sqlx::Error>;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub(crate) struct Reminder {
    pub id: i32,
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub message: String
}

async fn get_connection_pool() -> &'static Db {
    static POOL: OnceLock<Db> = OnceLock::new();
    let config = get_config();
    POOL.get_or_init(|| {
        println!("Connecting to Postgres database...");
        PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(&format!("postgres://{}:{}@{}/test", config.db_username, config.db_password, config.db_host))
            .expect("Failed to connect to Postgres database")
    })
}

pub(crate) async fn init_db() -> Result<()> {
    let mut trans = get_connection_pool().await.begin().await?;
    sqlx::query!("
CREATE TABLE IF NOT EXISTS reminders (
        id INT PRIMARY KEY,
        user_id BIGINT NOT NULL,
        time TIMESTAMPTZ NOT NULL,
        message NVARCHAR(256) NOT NULL
)
        ")
        .execute(&mut *trans)
        .await?;

    trans.commit().await
}

pub(crate) async fn fetch_reminders() -> Result<Vec<Reminder>> {
    sqlx::query_as::<Postgres, Reminder>("
SELECT * FROM reminders
WHERE date_trunc('hour', time) = CURRENT_TIME
        ")
        .fetch_all(get_connection_pool().await)
        .await
}

//TODO: same issue as noted in the todo below
pub(crate) async fn fetch_reminders_from(user_id: i64) -> Result<Vec<Reminder>> {
    sqlx::query_as::<Postgres, Reminder>("
SELECT * FROM reminders
WHERE reminders.user_id = $1
        ")
        .bind(user_id)
        .fetch_all(get_connection_pool().await)
        .await
}

//TODO: make this take a u64 and apply unchanging conversion
pub(crate) async fn add_reminder(user_id: i64, time: DateTime<Utc>, message: String) -> Result<()> {
    let mut trans = get_connection_pool().await.begin().await?;
    sqlx::query("
INSERT INTO reminders VALUES (DEFAULT, $1, $2, $3)
        ")
        .bind(user_id)
        .bind(time)
        .bind(message)
        .execute(&mut *trans)
        .await?;

    trans.commit().await
}
