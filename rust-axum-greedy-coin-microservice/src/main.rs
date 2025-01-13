use axum::{extract::Path, routing::get, Json, Router};
use serde_json::json;
use std::net::SocketAddr;

// Greedy algorithm to calculate coin change
fn greedy_coin_change(amount: u32) -> Vec<u32> {
    let coins = [25, 10, 5, 1]; // Coin denominations
    let mut amount = amount;
    let mut result = vec![];

    for &coin in &coins {
        let count = amount / coin;
        result.push(count);
        amount %= coin;
    }

    result
}

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

    // Define the address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    // Serve the application using axum_server
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
