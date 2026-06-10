#![allow(unused)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub public_key: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow, sqlx::Type)]
pub struct File {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub file_name: String,
    pub file_size: i64,
}