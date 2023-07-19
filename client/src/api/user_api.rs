use reqwasm::http;

use super::{
    types::{ErrorResponse, User, UserResponse},
    API_ROOT,
};

pub async fn api_register_user(user_data: &str) -> Result<User, String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::post(&format!("{}/api/auth/register", api_root))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(user_data)
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

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_login_user(credentials: &str) -> Result<User, String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::post(&format!("{}/api/auth/login", api_root))
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
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

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_user_info() -> Result<User, String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::get(&format!("{}/api/users/me", api_root))
        .credentials(http::RequestCredentials::Include)
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

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_logout_user() -> Result<(), String> {
    let api_root = API_ROOT.unwrap_or("http://localhost:8080");

    let response = match http::Request::get(&format!("{}/api/auth/logout", api_root))
        .credentials(http::RequestCredentials::Include)
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
}
