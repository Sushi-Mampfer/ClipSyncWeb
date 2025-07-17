use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, broadcast};


#[derive(Clone)]
pub struct AppState {
    pub tx: mpsc::Sender<String>,
    pub rx_spawner: broadcast::Sender<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WsMsg {
    pub id: String,
    pub data: String,
}