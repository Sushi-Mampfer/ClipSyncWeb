mod datatypes;
mod ws;
mod backend;

use std::net::SocketAddr;
use std::sync::LazyLock;

use axum::{routing::any, Router};
use once_cell::sync::OnceCell;
use tokio::sync::broadcast::Sender;
use tokio::sync::{broadcast, mpsc, Mutex};

use crate::backend::recv_loop;
use crate::datatypes::AppState;
use crate::ws::ws_route;

static SENDER: OnceCell<Mutex<Sender<String>>> = OnceCell::new();
static CONNECTED: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

#[tokio::main]
async fn main() {
    let (sender_tx, sender_rx) = mpsc::channel::<String>(16);
    let (receiver_tx, _receiver_rx) = broadcast::channel::<String>(16);

    let state = AppState {
        tx: sender_tx,
        rx_spawner: receiver_tx.clone()
    };

    SENDER.set(Mutex::new(receiver_tx)).unwrap();
    
    tokio::spawn(recv_loop(sender_rx));

    let app = Router::new()
        .route("/ws", any(ws_route))
        .with_state(state);

    axum_server::bind(SocketAddr::from(([127, 0, 0, 1], 8000))).serve(app.into_make_service()).await.unwrap();
}
