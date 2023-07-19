use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    models::{TaskModel, UserModel},
    schemas::{CreateTaskSchema, UpdateTaskBody, UpdateTaskSchema},
    validators::{TaskEntry, ValidatedRequest},
    AppState,
};

// const USER_ID: &str = "405f1394-e5eb-470d-8174-14020f55e881";

/// Axum Route Handler to Add a Record.
// Important: the first 2 extractors must be "server state";
// the latter corresponds to the body of the request.
// SEE: order of the extractors in the note below.
// https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
// [«consumes the request body and thus must be the last extractor»]
pub async fn create_task_handler(
    State(data): State<Arc<RwLock<AppState>>>,
    Extension(user): Extension<UserModel>,
    ValidatedRequest(body): ValidatedRequest<TaskEntry>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let client = &data.write().await.client;

    // We get the user_id of the request extensions that, thanks to the
    // route protection middleware, carry the UserModel object.
    let task_schema = CreateTaskSchema {
        title: body.title,
        description: body.description,
        user_id: user.id.to_string(),
    };

    // println!("body: {}", json!(task_body).to_string());

    let response = client
        .from("tasks")
        .insert(&json!(task_schema).to_string())
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let task_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let task_data: Vec<TaskModel> = serde_json::from_str(&task_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    match task_data.iter().next() {
        Some(task) => {
            let created_task = json!({ "status": "success", "data": json!({
                "task": json!(task)
            })});

            Ok((StatusCode::CREATED, Json(created_task)))
        }
        None => {
            let error_response = json!({
                "status": "error",
                "message": format!("Something bad happened while fetching the task")
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

/// Axum Route Handler to Fetch All Records.
pub async fn get_tasks_handler(
    State(data): State<Arc<RwLock<AppState>>>,
    Extension(user): Extension<UserModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let client = &data.read().await.client;

    // We get the user_id of the request extensions that, thanks to the
    // route protection middleware, carry the UserModel object.
    let response = client
        .from("tasks")
        .select("*")
        .eq("user_id", user.id.to_string())
        .order("created_at.desc")
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let tasks_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let tasks_data: Vec<TaskModel> = serde_json::from_str(&tasks_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let retrieved_tasks = json!({ "status": "success", "data": json!({
        "tasks": json!(tasks_data)
    })});

    Ok((StatusCode::OK, Json(retrieved_tasks)))
}

/// Axum Route Handler to Retrieve a Single Record.
pub async fn get_single_task_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let client = &data.read().await.client;

    let response = client
        .from("tasks")
        .select("*")
        .eq("id", &id.to_string())
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let task_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let task_data: Vec<TaskModel> = serde_json::from_str(&task_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    match task_data.iter().next() {
        Some(task) => {
            let retrieved_task = json!({ "status": "success", "data": json!({
                "task": json!(task)
            })});

            Ok((StatusCode::OK, Json(retrieved_task)))
        }
        None => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Task with ID: {} not found", &id.to_string())
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

/// Axum Route Handler to Edit a Record.
pub async fn update_task_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<RwLock<AppState>>>,
    Json(body): Json<UpdateTaskBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let client = &data.write().await.client;

    let response = client
        .from("tasks")
        .select("*")
        .eq("id", &id.to_string())
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let task_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let task_data: Vec<TaskModel> = serde_json::from_str(&task_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let retrieved_task = task_data.into_iter().next().ok_or_else(|| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Task with ID: {} not found", &id.to_string())
        });

        (StatusCode::NOT_FOUND, Json(error_response))
    })?;

    let task_schema = UpdateTaskSchema {
        title: body.title.to_owned().unwrap_or(retrieved_task.title),
        description: body
            .description
            .to_owned()
            .unwrap_or(retrieved_task.description),
        completed: body
            .completed
            .to_owned()
            .unwrap_or(retrieved_task.completed),
    };

    let response = client
        .from("tasks")
        .update(&json!(task_schema).to_string())
        .eq("id", &id.to_string())
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let task_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let task_data: Vec<TaskModel> = serde_json::from_str(&task_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    match task_data.iter().next() {
        Some(task) => {
            let update_task = json!({ "status": "success", "data": json!({
                "task": json!(task)
            })});

            Ok((StatusCode::OK, Json(update_task)))
        }
        None => {
            let error_response = json!({
                "status": "error",
                "message": format!("Something bad happened while updating the task")
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

/// Axum Route Handler to Delete a Record.
pub async fn delete_task_handler(
    Path(id): Path<Uuid>,
    State(data): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let client = &data.write().await.client;

    let response = client
        .from("tasks")
        .delete()
        .eq("id", &id.to_string())
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let task_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let task_data: Vec<TaskModel> = serde_json::from_str(&task_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    match task_data.iter().next() {
        Some(_task) => Ok(StatusCode::NO_CONTENT),
        None => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Task with ID: {} not found", &id.to_string())
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

/*
 * ORDEN DE LOS EXTRACTORES. VER:
 * https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
 * (VIENE DE ESTE ERROR: https://github.com/tokio-rs/axum/discussions/641)
 */
