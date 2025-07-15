use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, response::Response};
use futures_util::stream::{SplitSink, SplitStream, StreamExt};
use tokio::sync::{broadcast::Receiver, mpsc::Sender};

use crate::datatypes::AppState;

pub async fn ws_route(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(read(receiver, state.tx));
    tokio::spawn(write(sender, state.rx_spawner.subscribe()));
}

async fn read(receiver: SplitStream<WebSocket>, sender: Sender<String>) {

}

async fn write(sender: SplitSink<WebSocket, Message>, receiver: Receiver<String>) {
    
}