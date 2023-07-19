use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use tokio::sync::RwLock;

use crate::{
    handlers::tasks_handlers::{
        create_task_handler, delete_task_handler, get_single_task_handler, get_tasks_handler,
        update_task_handler,
    },
    middleware::auth,
    AppState,
};

pub fn tasks_router(app_state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route(
            "/api/tasks",
            get(get_tasks_handler)
                .post(create_task_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/tasks/:id",
            get(get_single_task_handler)
                .patch(update_task_handler)
                .delete(delete_task_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}

/*
 * MULTIPLE METHODS FOR THE SAME ROUTE. SEE:
 * https://docs.rs/axum/latest/axum/routing/struct.Router.html#accepting-multiple-methods
 * (in this case the import is not necessary either)
 *
 * ALTHOUGH IT ALSO WORKS BY ADDING THE ROUTING METHODS ONE BY ONE, EVEN IF THEY ARE FOR THE SAME
 * ROUTE. SEE:
 * https://github.com/tokio-rs/axum/discussions/2050
 */
