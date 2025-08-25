use axum::{
    extract::Query, routing::{get, post}, Json, Router
};

// Able to convert to and from Json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
// Must use pub (public) for other files to access it
pub struct Vehicle {
    id: Option<String>, // UUID, optional
    make: String,
    model: String,
    year: u16,
}

pub async fn vehicle_get() -> Json<Vehicle> {

    println!("GET /vehicle"); //check the livelyness of the endpoint

    Json::from(
        Vehicle {
            id: Some(uuid::Uuid::new_v4().to_string()),
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            year: 2020,
        }
    )
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