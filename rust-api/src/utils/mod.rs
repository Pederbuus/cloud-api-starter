// External crates that are commonly used
pub use axum::{
    extract::{Query, State, Json},
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
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
pub use axum::extract::Path; // for extracting path parameters, /vehicle/{id}

// Standard library items
pub use std::env;
pub use std::sync::Arc;

// Able to convert to and from Json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    pub id: Uuid,
    pub make: String,
    pub model: String,
    pub year: i32,
}
impl Vehicle {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Vehicle {
            id: row.get("id"),
            make: row.get("make"),
            model: row.get("model"),
            year: row.get("year"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VehicleCreateDto {
    // No UUID when creating a new vehicle
    pub make: String,
    pub model: String,
    pub year: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VehiclePut {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
}

// Define the application state struct
pub struct AppState {
    pub db_client: Client,
}