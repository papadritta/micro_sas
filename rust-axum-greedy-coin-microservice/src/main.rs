use axum::extract::Path;
use axum::{routing::get, Json, Router};
use rust_axum_greedy_coin_microservice::greedy_coin_change;
use serde_json::json;

//Root Route for Change Machine
async fn root() -> &'static str {
    "
    Greedy Coin Change Machine

    **Primary Route:**
    /change/dollars/cents
    "
}

async fn change(Path((dollars, cents)): Path<(u32, u32)>) -> impl axum::response::IntoResponse {
    // Convert to cents amount
    let amount = dollars * 100 + cents;
    let change = greedy_coin_change(amount);
    let json = json!({
     "dollars": dollars,
     "cents": cents,
     "change": change
    });
    Json(json)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/change/:dollars/:cents", get(change));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
