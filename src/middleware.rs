use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tokio::sync::RwLock;

use crate::{models::UserModel, response::ErrorResponse, schemas::TokenClaims, AppState};

/// Axum JWT Authentication Middleware.
pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<RwLock<AppState>>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token") // We try to get the token from the cookie
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            // Otherwise, we try to get it from the authorization header
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    // If the token is none, we return UNAUTHORIZED.
    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.clone().read().await.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        // We return UNAUTHORIZED if the token fails validation for some reason.
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    // We get the user ID from the token.
    // We try to parse the ID, stored in the token as a String, as a Uuid.
    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        // If the id is incorrectly formed, we return an error.
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    // With a valid user_id we verify that the user still exists in the database.
    let response = data
        .clone()
        .read()
        .await
        .client
        .from("users")
        .select("*")
        .eq("id", &user_id.to_string())
        .execute()
        .await
        .map_err(|err| {
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching user from database: {}", err),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;

    let user_response = response.text().await.map_err(|err| {
        let json_error = ErrorResponse {
            status: "fail",
            message: format!("Error parsing json response: {}", err),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
    })?;

    let user_data: Vec<UserModel> = serde_json::from_str(&user_response).map_err(|err| {
        let json_error = ErrorResponse {
            status: "fail",
            message: format!("Error deserializing response: {}", err),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
    })?;

    // We get the user with the given user_id or we return an error.
    let Some(user_found) = user_data.into_iter().next() else {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        
        return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
    };

    // Once the check is done, we could just return the user_id, but in this case,
    // we return the entire UserModel object.
    req.extensions_mut().insert(user_found);
    Ok(next.run(req).await)
}
