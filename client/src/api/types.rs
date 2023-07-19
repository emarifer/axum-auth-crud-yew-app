use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/****** Types for authentication ******/

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

/****** Task Model ******/

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub user_id: Uuid,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskData {
    pub task: Task,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TasksData {
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleTaskResponse {
    pub status: String,
    pub data: TaskData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleTaskResponse {
    pub status: String,
    pub data: TasksData,
}

/****** Error messages ******/

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

/*
 * ERROR AL USAR EL CRATE UUID EN WEBASSEMBLY. VER:
 * https://github.com/rust-random/getrandom/issues/208
 * https://docs.rs/getrandom/0.2.2/getrandom/#webassembly-support
 */
