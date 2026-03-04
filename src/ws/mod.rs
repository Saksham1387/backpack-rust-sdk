use crate::BackpackClient;
const BASE_URL_WS: &str = "wss://ws.backpack.exchange";
use crate::error::{BackpackError, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

fn is_private_stream(stream: &str) -> bool {
    stream.starts_with("account.")
}

impl BackpackClient {
    // subscribe
    // subscribe to multiple

    pub async fn subscribe<T>(&self, stream: &str, tx: Sender<T>) -> Result<()>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(&[&stream], tx).await
    }

    pub async fn subscribe_multiple<T>(&self, streams: &[&str], tx: Sender<T>) -> Result<()>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(streams, tx).await
    }

    pub async fn internal_subscribe<T>(&self, streams: &[&str], tx: Sender<T>) -> Result<()>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        // TODO: Support the private streams as well in here.
        let subscribe_message = json!({
            "method": "SUBSCRIBE",
            "params": streams,
        });

        let (mut ws_stream, _) = connect_async(BASE_URL_WS)
            .await
            .map_err(|e| BackpackError::WebSocket(e.to_string()))?;

        println!("[WS] Connected, sending: {}", subscribe_message);

        ws_stream
            .send(Message::Text(subscribe_message.to_string().into()))
            .await
            .map_err(|e| BackpackError::WebSocket(e.to_string()))?;

        tracing::debug!("Subscribed to {streams:#?}");
        println!("[WS] Waiting for messages...");

        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    println!("[WS] Received: {}", text);
                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                        if let Some(payload) = value.get("data") {
                            match T::deserialize(payload) {
                                Ok(data) => {
                                    if tx.send(data).await.is_err() {
                                        break;
                                    }
                                }

                                Err(err) => {
                                    tracing::error!("Could not deserialize ws payload: {err}");
                                }
                            }
                        } else if let Some(error) = value.get("error") {
                            tracing::error!(?error, "WebSocket error response from server");
                        }
                    }
                }

                Ok(Message::Close(_)) => {
                    break;
                }

                Err(erro) => {
                    break;
                }

                _ => {
                    // Ping pong is handled by the tokio-tungstenite be default
                }
            }
        }
        Ok(())
    }
}
