use axum::{
    async_trait,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestExt,
};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterEntry {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginEntry {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TaskEntry {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
}

/// Use this to encapsulate fields that require validation
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<J>(pub J);

#[async_trait]
impl<S, B, J> FromRequest<S, B> for ValidatedRequest<J>
where
    S: Send + Sync,
    B: Send + 'static,
    J: Validate + 'static,
    Json<J>: FromRequest<(), B>,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req
            .extract::<Json<J>, _>()
            .await
            .map_err(|err| err.into_response())?;

        data.validate().map_err(|err| {
            let message = format!("Input validation error: [{}]", err).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message).into_response()
        })?;

        Ok(Self(data))
    }
}

/*
 * CREACIÓN DE UN EXTRACTOR PERSONALIZADO PARA LA VALIDACIÓN DE DATOS EN EL BACKEND. VER:
 * https://dev.to/ayush1325/validating-json-request-in-axum-2n34
 * https://docs.rs/axum/0.6.18/axum/trait.RequestExt.html#tymethod.extract
 * https://github.com/tokio-rs/axum/blob/main/examples/validator/src/main.rs
 */
