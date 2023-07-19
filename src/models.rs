use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskModel {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub user_id: Uuid,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Local>,
}
