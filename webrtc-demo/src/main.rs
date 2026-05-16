mod signaling;

use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::sync::{mpsc, Mutex};
use tower_http::cors::CorsLayer;

pub struct WaitingPeer {
    pub outgoing_tx: mpsc::Sender<String>,
    pub partner_tx: Arc<Mutex<Option<mpsc::Sender<String>>>>,
}

pub struct AppState {
    pub waiting: Mutex<Option<WaitingPeer>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_level(true)
        .init();

    let state = Arc::new(AppState {
        waiting: Mutex::new(None),
    });

    let app = Router::new()
        .route("/", get(signaling::index_handler))
        .route("/ws", get(signaling::ws_handler))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Listening on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
