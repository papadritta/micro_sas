use axum::{extract::Path, routing::get, Json, Router};
use rust_axum_greedy_coin_microservice::greedy_coin_change;
use serde_json::json;

// Root route for the Change Machine
async fn root() -> &'static str {
    "
    Greedy Coin Change Machine

    **Primary Route:**
    /change/{dollars}/{cents}
    "
}

// Handler for the /change/{dollars}/{cents} route
async fn change(Path((dollars, cents)): Path<(u32, u32)>) -> Json<serde_json::Value> {
    let amount = (dollars * 100 + cents) as u32; // Convert dollars and cents to total cents
    let coins = greedy_coin_change(amount); // Call the greedy algorithm for coin change
    Json(json!({ "coins": coins }))
}

#[tokio::main]
async fn main() {
    // Build the router
    let app = Router::new()
        .route("/", get(root)) // Root route
        .route("/change/{dollars}/{cents}", get(change)); // Change route with parameters

    // Run the server on 0.0.0.0:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
