use axum::{routing::{get, post}, Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer}; // NEW

mod models;
mod engine;
mod api;

use engine::{Matchmaker, run_matchmaking_loop};
use api::{join_queue, get_state};

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(Matchmaker::new()));

    let loop_state = Arc::clone(&state);
    tokio::spawn(async move {
        run_matchmaking_loop(loop_state).await;
    });

    // Configure CORS to allow local HTML file to fetch data
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/join", post(join_queue))
        .route("/state", get(get_state)) // NEW
        .layer(Extension(state))
        .layer(cors); // NEW

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}