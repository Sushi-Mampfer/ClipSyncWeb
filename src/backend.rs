use tokio::sync::mpsc::Receiver;

use serde_json::{from_str, to_string};

use crate::{datatypes::WsMsg, SENDER};

pub async fn recv_loop(mut receiver: Receiver<String>) {
    while let Some(msg) = receiver.recv().await {
        match from_str::<WsMsg>(&msg) {
            Ok(msg) => {
                if msg.id == "clip" {
                    send(msg).await;
                }
            }
            _ => ()
        }
    }

}

pub async fn send(msg: WsMsg) {
    let sender = SENDER.get().unwrap().lock().await;
    sender.send(to_string(&msg).unwrap()).unwrap();
}