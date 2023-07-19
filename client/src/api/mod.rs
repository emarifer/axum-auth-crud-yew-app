pub mod task_api;
pub mod types;
pub mod user_api;

#[allow(unused)]
const API_ROOT: Option<&'static str> = std::option_env!("API_ROOT");
