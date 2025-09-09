// External crates that are commonly used
pub use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json,
    Router,
};
pub use serde::{
    Deserialize,
    Serialize,
};
pub use tokio_postgres::{
    Client,
    NoTls,
    Error,
};
pub use uuid::Uuid;

// Standard library items
pub use std::env;
pub use std::sync::Arc;

// Able to convert to and from Json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    pub id: Option<Uuid>, // UUID, optional
    pub make: String,
    pub model: String,
    pub year: i32,
}

// Define the application state struct
pub struct AppState {
    pub db_client: Client,
}