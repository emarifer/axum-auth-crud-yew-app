//! Request Validation Structs.
//! Define structures that allow us to deserialize and validate the request bodies.
//! Validation ensures that the necessary fields are present in the request body and that they have the correct data types.

use serde::{Deserialize, Serialize};

/****** Tasks entities ******/

/// It is used as the type for an extractor through which data is sent to create a task.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTaskBody {
    pub title: String,
    pub description: String,
}

/// Used as the template type through which data is sent to the database to create a task.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTaskSchema {
    pub title: String,
    pub description: String,
    pub user_id: String,
}

/// Used as the type for an extractor through which data is sent to update a task.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskBody {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

/// Used as the template type through which data is sent to the database to update a task.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskSchema {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

/****** Users entities ******/

/// It is used as a template through which data is sent to create a user in the database (registration).
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUserSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// It is used as a extractor through which data is sent to login a user.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserBody {
    pub email: String,
    pub password: String,
}

/****** Token Data ******/

/// It is used as the template type through which the authentication token is sent using a cookieo.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
