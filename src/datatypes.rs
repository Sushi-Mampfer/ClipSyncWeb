use tokio::sync::{mpsc, broadcast};


#[derive(Clone)]
pub struct AppState {
    pub tx: mpsc::Sender<String>,
    pub rx_spawner: broadcast::Sender<String>,
}