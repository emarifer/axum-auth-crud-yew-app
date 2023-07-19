// #![allow(unused)] // For beginning only.

mod config;
mod handlers;
mod middleware;
mod models;
mod response;
mod routes;
mod schemas;
mod validators;

use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use axum::{
    body::{boxed, Body},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Request, StatusCode,
    },
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use dotenv::dotenv;
use postgrest::Postgrest;
use tokio::sync::RwLock;
use tower::ServiceExt;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use config::Config;

pub struct AppState {
    client: Postgrest,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let static_file_service = |req: Request<Body>| async {
        let mut resp = match req.uri().to_string().as_str() {
            s if s.ends_with("css") => ServeDir::new("client/dist").oneshot(req).await,
            s if s.ends_with("js") => ServeDir::new("client/dist").oneshot(req).await,
            s if s.ends_with("wasm") => ServeDir::new("client/dist").oneshot(req).await,
            s if s.ends_with("svg") => ServeDir::new("client/dist").oneshot(req).await,
            _ => ServeFile::new("client/dist/index.html").oneshot(req).await,
        };

        if resp.as_mut().unwrap().status() == 404 {
            return Ok::<_, Infallible>(
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from("Something went wrong...\n")))
                    .unwrap(),
            );
        }

        // Ok::<_, Infallible>(resp.into_response().map(|body| BoxBody::new(body)))
        Ok::<_, Infallible>(resp.into_response())
    };

    let config = Config::init();

    let client =
        Postgrest::new(&config.supabase_url).insert_header("apikey", &config.supabase_anon_key);

    let app_state = Arc::new(RwLock::new(AppState {
        client,
        env: config,
    }));

    println!("->> ✅Connection to the database is successful!\n");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let routes_all = Router::new()
        .merge(routes::route_healthchecker::healthchecker_router())
        .merge(routes::routes_tasks::tasks_router(app_state.clone()))
        .merge(routes::routes_users::users_router(app_state.clone()))
        .nest_service("/", get(static_file_service))
        .fallback_service(get(static_file_service))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");

    println!("🚀 Server started successfully!!\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

/*
 * https://www.bradcypert.com/chili-cookoff-with-rust-rocket-render-and-supabase/
 *
 * ORDEN DE LOS EXTRACTORES [«consume el cuerpo de la solicitud y, por lo tanto, debe ser el último extractor»]. VER:
 * https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
 *
 * Clean Code with Rust & Axum. VER:
 * https://www.propelauth.com/post/clean-code-with-rust-and-axum
 */
