use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, response::Response};
use futures_util::{SinkExt, stream::{SplitSink, SplitStream, StreamExt}};
use tokio::sync::{broadcast::Receiver, mpsc::Sender};

use crate::{backend::send, datatypes::{AppState, WsMsg}, CONNECTED};

pub async fn ws_route(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (sender, receiver) = socket.split();
    {
        let mut count = CONNECTED.lock().await;
        *count += 1;
        send(WsMsg {
            id: "conn".to_string(),
            data: (*count).to_string()
        }).await;
    }
    tokio::spawn(read(receiver, state.tx));
    tokio::spawn(write(sender, state.rx_spawner.subscribe()));
}

async fn read(mut receiver: SplitStream<WebSocket>, sender: Sender<String>) -> Option<()> {
    while let Some(Ok(m)) = receiver.next().await {
        match m.to_text() {
            Ok(t) => sender.send(t.to_string()).await.ok()?,
            _ => ()
        }
    }
    let mut count = CONNECTED.lock().await;
    *count -= 1;
    send(WsMsg {
        id: "conn".to_string(),
        data: (*count).to_string()
    }).await;
    Some(())
}

async fn write(mut sender: SplitSink<WebSocket, Message>, mut receiver: Receiver<String>) -> Option<()> {
    while let Ok(m) = receiver.recv().await {
        sender.send(m.into()).await.ok()?;
    }
    Some(())
}