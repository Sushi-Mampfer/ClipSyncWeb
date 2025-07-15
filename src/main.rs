mod datatypes;
mod ws;

use std::net::SocketAddr;

use axum::{routing::any, Router};
use tokio::sync::{broadcast, mpsc};

use crate::datatypes::AppState;
use crate::ws::ws_route;

#[tokio::main]
async fn main() {
    let (sender_tx, sender_rx) = mpsc::channel::<String>(16);
    let (receiver_tx, _receiver_rx) = broadcast::channel::<String>(16);

    let state = AppState {
        tx: sender_tx,
        rx_spawner: receiver_tx.clone()
    };
    
    let app = Router::new()
        .route("/ws", any(ws_route))
        .with_state(state);

    axum_server::bind(SocketAddr::from(([127, 0, 0, 1], 8000))).serve(app.into_make_service()).await.unwrap();
}
