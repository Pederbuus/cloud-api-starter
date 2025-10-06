pub mod utils;
use crate::utils::*;
mod vehicle;
use vehicle::{vehicle_get, vehicle_post, vehicle_get_id, vehicle_put, vehicle_post_query, vehicle_delete};

#[tokio::main]
async fn main() {
    // Establish the database connection
    let db_client = establish_db_connection().await.expect("Failed to connect to database");

    // Create the shared application state
    let shared_state = Arc::new(AppState {
        db_client: db_client,
    });

    // Build the Axum router
    let app = Router::new()
    .route("/", get(|| async { "Welcome to the Vehicle API" }))
    .route("/vehicle", get(vehicle_get))
    .route("/vehicle", post(vehicle_post))
    .route("/vehicle/{capture}", put(vehicle_put)) // Alternative route with {id}
    .route("/vehicle/query", post(vehicle_post_query))
    .route("/vehicle/{capture}", get(vehicle_get_id))
    .route("/vehicle/{capture}", delete(vehicle_delete))
    .route("/ping", get(ping)) //ping endpoint
    .route("/vehicle/total", get(vehicle_total))
        .with_state(shared_state); // Pass the shared state (the db) to Axum

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> &'static str {
    "pong"
}

async fn vehicle_total(
    State(state): State<Arc<AppState>>,
) -> String {
    // Example: Use the client to execute a query
    let rows = state.db_client.query("SELECT * FROM vehicle", &[]).await.unwrap();
    let result = rows.len();
    format!("Rows in the database: {}", result)
}


async fn establish_db_connection() -> Result<Client, Error> {
    // Read the DATABASE_URL from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(
        &database_url,
        NoTls,
    ).await?;

    // Spawn a task to handle the background connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}