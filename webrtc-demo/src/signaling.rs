use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, Response},
};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::{AppState, WaitingPeer};

pub async fn ws_handler(State(state): State<Arc<AppState>>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let (outgoing_tx, mut outgoing_rx) = mpsc::channel::<String>(16);

    let partner_tx: Arc<Mutex<Option<mpsc::Sender<String>>>> = Arc::new(Mutex::new(None));

    {
        let mut slot = state.waiting.lock().await;
        if let Some(other) = slot.take() {
            *other.partner_tx.lock().await = Some(outgoing_tx.clone());
            *partner_tx.lock().await = Some(other.outgoing_tx.clone());

            let _ = other
                .outgoing_tx
                .send(r#"{"type":"paired","initiator":true}"#.to_string())
                .await;
            let _ = outgoing_tx
                .send(r#"{"type":"paired","initiator":false}"#.to_string())
                .await;

            tracing::info!("Two peers paired — signaling relay active");
        } else {
            *slot = Some(WaitingPeer {
                outgoing_tx: outgoing_tx.clone(),
                partner_tx: partner_tx.clone(),
            });
            let _ = outgoing_tx
                .send(r#"{"type":"wait"}"#.to_string())
                .await;

            tracing::info!("First peer waiting for partner");
        }
    }

    loop {
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Some(tx) = partner_tx.lock().await.as_ref() {
                            let _ = tx.send(text.to_string()).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
            msg = outgoing_rx.recv() => {
                match msg {
                    Some(text) => {
                        if socket.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
    }
}

pub async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}