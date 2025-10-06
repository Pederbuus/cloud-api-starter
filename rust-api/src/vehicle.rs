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
            id: row.get(0),
            make: row.get(1),
            model: row.get(2),
            year: row.get::<usize, i32>(3),
        }
    }).collect();

    // Return the vehicle as JSON to the API caller
    Json(vehicle)
}

pub async fn vehicle_get_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Json<Option<Vehicle>> {

    let row_result = state.db_client
        .query_one("SELECT id, make, model, year FROM vehicle WHERE id = $1", &[&id])
        .await;
    
    let row = match row_result {
        Ok(row) => Some(Vehicle::from_row(&row)),
        Err(_) => return Json(None),
    };

    Json(row)
}

pub async fn vehicle_put(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<VehiclePut>,
) -> impl axum::response::IntoResponse {

    //println!("Updating vehicle: {:?}", payload);

    // Save the payload to the database
    let query = "UPDATE vehicle SET make = COALESCE($2, make), model = COALESCE($3, model), year = COALESCE($4, year) WHERE id = $1 RETURNING id, make, model, year";
    let row_result = state.db_client
        .query_opt(
            query,
            &[
                &id,
                &payload.make,
                &payload.model,
                &payload.year,
            ],
        )
        .await;
    // Assuming a POST (Create) operation
    match row_result { // row_result comes from a query_opt or query_one with RETURNING clause
        Ok(Some(row)) => {
            // Success: Item created and returned.
            let vehicle = Vehicle::from_row(&row);
            Ok((StatusCode::CREATED, Json(vehicle))) // 201 CREATED
        }
        Ok(None) => {
            // This case is unlikely if your query used RETURNING, but useful for robustness.
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Creation failed silently".to_string())) // 500
        }
        Err(e) => {
            // Unexpected Error: DB connection, invalid input that broke the query, etc.
            // A common database error when creating is a uniqueness violation (e.g., ID already exists).
            // You might check 'e' for specific error codes to return a 409 CONFLICT.
            eprintln!("DB error: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())) // 500
        }
    }
}

//     Json(payload)
// }

#[axum::debug_handler]
pub async fn vehicle_post(
    State(state): State<Arc<AppState>>,
    Json(payload_user): Json<VehicleCreateDto>
) -> Json<Vehicle> {

    let payload = Vehicle {
        id: Uuid::new_v4(),
        make: payload_user.make,
        model: payload_user.model,
        year: payload_user.year,
    };

    println!("Manufacturer: {}", payload.make);

    // Save the payload to the database
    let query = "INSERT INTO vehicle (id, make, model, year) VALUES ($1, $2, $3, $4)";
    state.db_client
        .execute(
            query,
            &[
                &payload.id,
                &payload.make,
                &payload.model,
                &payload.year,
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
    Query(payload_user): Query<VehicleCreateDto>,
    Query(c): Query<Customer>,
) -> Json<Vehicle> {

    let payload = Vehicle {
        id: Uuid::new_v4(),
        make: payload_user.make,
        model: payload_user.model,
        year: payload_user.year,
    };

    println!("Manufacturer: {}", payload.make);
    println!("Customer Name: {}", c.name);
    println!("Customer Email: {}", c.email);

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

pub async fn vehicle_delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl axum::response::IntoResponse {

    let result = state.db_client
        .execute("DELETE FROM vehicle WHERE id = $1", &[&id])
        .await;

    match result {
        Ok(rows_deleted) => {
            if rows_deleted == 0 {
                // No rows deleted means the item was not found
                Err((StatusCode::NOT_FOUND, "Vehicle not found".to_string())) // 404
            } else {
                // Successfully deleted
                Ok((StatusCode::NO_CONTENT, ())) // 204 No Content
            }
        }
        Err(e) => {
            // Unexpected Error: DB connection, invalid input that broke the query, etc.
            eprintln!("DB error: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())) // 500
        }
    }
}