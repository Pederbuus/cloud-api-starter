use crate::utils::*;

// Extract the client from state instead of taking &mut Client directly
pub async fn vehicle_get(State(state): State<Arc<AppState>>) -> Json<Vec<Vehicle>> {

    let rows = state.db_client
        .query("SELECT id, make, model, year FROM vehicle", &[])
        .await
        .expect("query failed");

    // Map database rows to Vehicle structs
    let vehicle: Vec<Vehicle> = rows.iter().map(|row| {
        Vehicle {
            id: Some(Uuid::new_v4()),
            make: row.get(1),
            model: row.get(2),
            year: row.get::<usize, i32>(3),
        }
    }).collect();

    // Return the vehicle as JSON to the API caller
    Json(vehicle)
}

#[axum::debug_handler]
pub async fn vehicle_post(
    State(state): State<Arc<AppState>>,
    Json(mut payload): Json<Vehicle>
) -> Json<Vehicle> {

    println!("Manufacturer: {}", payload.make);

    // In a real application, you would save the vehicle to a database here
    let id = uuid::Uuid::new_v4();
    payload.id = Some(id);
    // Save the payload to the database
    let query = "INSERT INTO vehicle (id, make, model, year) VALUES ($1, $2, $3, $4)";
    state.db_client
        .execute(
            query,
            &[
                &payload.id,
                &payload.make,
                &payload.model,
                &(payload.year as i32),
            ],
        )
        .await
        .expect("Failed to insert vehicle");
    println!("POST /vehicle, id: {:?}", payload.id); //check the livelyness of the endpoint

    Json::from(payload)
}

#[derive(Debug, serde::Deserialize)]
pub struct Customer {
    name: String,
    email: String,
}

#[axum::debug_handler]
pub async fn vehicle_post_query(
    State(state): State<Arc<AppState>>,
    Query(mut payload): Query<Vehicle>,
    Query(c): Query<Customer>,
) -> Json<Vehicle> {

    println!("Manufacturer: {}", payload.make);
    println!("Customer Name: {}", c.name);
    println!("Customer Email: {}", c.email);

    // In a real application, you would save the vehicle to a database here
    let id = uuid::Uuid::new_v4();
    payload.id = Some(id);
    let query = "INSERT INTO vehicle (id, make, model, year) VALUES ($1, $2, $3, $4)";
    state.db_client
        .execute(
            query,
            &[
                &payload.id,
                &payload.make,
                &payload.model,
                &(payload.year as i32),
            ],
        )
        .await
        .expect("Failed to insert vehicle");
    println!("POST /vehicle, id: {:?}", payload.id); //check the livelyness of the endpoint

    Json::from(payload)
}