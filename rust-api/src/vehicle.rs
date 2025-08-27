use std::sync::{Arc};
use axum::{
    extract::{Query, State}, Json
};

use tokio_postgres::Client;

use uuid::Uuid;

// Able to convert to and from Json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
// Must use pub (public) for other files to access it
pub struct Vehicle {
    id: Option<String>, // UUID, optional
    make: String,
    model: String,
    year: u16,
}

// Define the application state struct
pub struct AppState {
    pub db_client: Client,
}

// Extract the client from state instead of taking &mut Client directly
pub async fn vehicle_get(State(state): State<Arc<AppState>>) -> Json<Vec<Vehicle>> {

    let rows = state.db_client
        .query("SELECT id, make, model, year FROM vehicle", &[])
        .await
        .expect("query failed");

    // Map database rows to Vehicle structs
    let vehicles: Vec<Vehicle> = rows.iter().map(|row| {
        Vehicle {
            id: Some(row.get::<usize, String>(0)),
            make: row.get(1),
            model: row.get(2),
            year: row.get::<usize, i32>(3) as u16,
        }
    }).collect();

    // Return the vehicles as JSON to the API caller
    Json(vehicles)
}

// #[axum::debug_handler]
// pub async fn vehicle_post(Json(mut payload): Json<Vehicle>) -> Json<Vehicle> {

//     println!("Manufacturer: {}", payload.make);

//     // In a real application, you would save the vehicle to a database here
//     let id = uuid::Uuid::new_v4().to_string();
//     payload.id = Some(id.clone());
//     println!("POST /vehicle, id: {:?}", payload.id); //check the livelyness of the endpoint

//     Json::from(payload)
// }

#[derive(Debug, serde::Deserialize)]
pub struct Customer {
    name: String,
    email: String,
}

#[axum::debug_handler]
pub async fn vehicle_post(
    Query(mut payload): Query<Vehicle>,
    Query(c): Query<Customer>,
) -> Json<Vehicle> {

    println!("Manufacturer: {}", payload.make);
    println!("Customer Name: {}", c.name);
    println!("Customer Email: {}", c.email);

    // In a real application, you would save the vehicle to a database here
    let id = uuid::Uuid::new_v4().to_string();
    payload.id = Some(id.clone());
    println!("POST /vehicle, id: {:?}", payload.id); //check the livelyness of the endpoint

    Json::from(payload)
}