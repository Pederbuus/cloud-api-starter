use axum::{
    routing::{get, post}, Json, Router
};

// import of module
mod vehicle;
use vehicle::{vehicle_get, vehicle_post};

const ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let mut requests_counter: u16 = 0;
    // Routers
    //let app = Router::new().route("/", get(|| async { "This is the home page" }));
    
    let vehicle = Router::new()
        .route("/vehicle", get(vehicle_get))
        .route("/vehicle", post(vehicle_post))
    ;

    // Port listener
    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    axum::serve(listener, vehicle).await.unwrap();
}