use reqwasm::http;

use super::{
    types::{ErrorResponse, MultipleTaskResponse, SingleTaskResponse, Task},
    API_ROOT,
};

pub async fn api_get_tasks() -> Result<Vec<Task>, String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::get(&format!("{}/api/tasks", api_root))
        // .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        // .body(task_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<MultipleTaskResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.tasks),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_get_single_task(id: String) -> Result<Task, String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::get(&format!("{}/api/tasks/{}", api_root, id))
        // .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        // .body(task_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<SingleTaskResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.task),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_create_task(task_data: &str) -> Result<Task, String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::post(&format!("{}/api/tasks", api_root))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(task_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 201 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<SingleTaskResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.task),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_update_task(id: String, task_data: &str) -> Result<Task, String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::patch(&format!("{}/api/tasks/{}", api_root, id))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(task_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<SingleTaskResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.task),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_delete_task(id: String) -> Result<(), String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::delete(&format!("{}/api/tasks/{}", api_root, id))
        // .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        // .body(task_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 204 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    Ok(())
    // let res_json = response.json::<SingleTaskResponse>().await;
    // match res_json {
    //     Ok(data) => Ok(data.data.task),
    //     Err(_) => Err("Failed to parse response".to_string()),
    // }
}
