use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    models::UserModel,
    response::FilteredUser,
    schemas::{RegisterUserSchema, TokenClaims},
    validators::{LoginEntry, RegisterEntry, ValidatedRequest},
    AppState,
};

/// User Registration Handler.
pub async fn register_user_handler(
    State(data): State<Arc<RwLock<AppState>>>,
    ValidatedRequest(body): ValidatedRequest<RegisterEntry>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // We verify that the user does not exist in the database.
    let response = data
        .clone()
        .read()
        .await
        .client
        .from("users")
        .select("*")
        .eq("email", &body.email)
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_data: Vec<UserModel> = serde_json::from_str(&user_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if let Some(user) = user_data.into_iter().next() {
        if user.email == body.email {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that email already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    // We encrypt the password that the user sends us.
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    // We create an entity that will then be inserted into the database.
    let user_schema = RegisterUserSchema {
        username: body.username,
        email: body.email,
        password: hashed_password,
    };

    let response = data
        .clone()
        .write()
        .await
        .client
        .from("users")
        .insert(&json!(user_schema).to_string())
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_data: Vec<UserModel> = serde_json::from_str(&user_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // We get the user created in the database or we return an error.
    let Some(user_created) = user_data.into_iter().next() else {
        let error_response = json!({
                "status": "error",
                "message": format!("Something wrong happened while creating the user.")
            });

           return  Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    };

    // We create the access token.
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_created.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&data.read().await.env.jwt_secret.as_ref()),
    )
    .unwrap();

    // println!("Token creado: {:?}", token);

    // We create a container for the cookie that will send in the created token.
    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    // We filter the UserModel type to remove the password.
    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_record(&user_created)
    })});

    // Finally, we build a response and in the header we insert the cookie with the token
    // and in the body we attach the response with the created user.
    let mut response = Response::new(user_response.to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

/// User Login Handler.
pub async fn login_user_handler(
    State(data): State<Arc<RwLock<AppState>>>,
    ValidatedRequest(body): ValidatedRequest<LoginEntry>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // We verify that the user's email exists in the database.
    let response = data
        .clone()
        .read()
        .await
        .client
        .from("users")
        .select("*")
        .eq("email", &body.email)
        .execute()
        .await
        .map_err(|err| {
            let error_response = json!({
                "status": "fail",
                "message": format!("Database error: {}", err)
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let user_response = response.text().await.map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error parsing json response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_data: Vec<UserModel> = serde_json::from_str(&user_response).map_err(|err| {
        let error_response = json!({
            "status": "fail",
            "message": format!("Error deserializing response: {}", err)
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // We get the user with the given email or we return an error.
    let Some(user_found) = user_data.into_iter().next() else {
        let error_response = json!({
                "status": "fail",
                "message": "Invalid email or password",
            });

           return  Err((StatusCode::BAD_REQUEST, Json(error_response)));
    };

    // We verify that the password is valid.
    let is_valid = match PasswordHash::new(&user_found.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    // We create the access token.
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_found.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&data.read().await.env.jwt_secret.as_ref()),
    )
    .unwrap();

    // println!("Token creado: {:?}", token);

    // We create a container for the cookie that will send in the created token.
    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    // We filter the UserModel type to remove the password.
    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_record(&user_found)
    })});

    // Finally, we build a response and in the header we insert the cookie with the token
    // and in the body we attach the response with the created user.
    let mut response = Response::new(user_response.to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

/// User Logout Handler.
pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(json!({"status": "success"}).to_string()) // although StatusCode::NO_CONTENT makes the body empty.
        .unwrap();

    // let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}

/// Handler to Fetch Logged-in User.
/// Checking the authentication middleware (auth).
pub async fn get_me_handler(
    Extension(user): Extension<UserModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let json_response = json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });

    Ok(Json(json_response))
}

/// Convenience utility to filter sensitive user data.
fn filter_user_record(user: &UserModel) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        username: user.username.to_owned(),
        email: user.email.to_owned(),
        created_at: user.created_at,
    }
}
