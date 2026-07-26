use actix_web::{web, HttpRequest, HttpResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::services::api::ApiBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    Subscribe { channel: String },
    Unsubscribe { channel: String },
    Event { channel: String, data: serde_json::Value },
    Authenticate { token: String },
    AuthSuccess,
    AuthError { message: String },
    Pong,
    Ping,
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsAuthRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsSubscribeRequest {
    pub channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsUnsubscribeRequest {
    pub channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub connection_id: String,
    pub user_id: Option<String>,
    pub subscribed_channels: Vec<String>,
    pub connected_at: String,
    pub last_heartbeat: String,
}

#[derive(Clone)]
pub struct WsConnectionManager {
    pub shutdown_flag: Arc<AtomicBool>,
}

impl WsConnectionManager {
    pub fn new() -> Self {
        Self {
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_shutting_down(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }

    pub fn initiate_shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
        info!("WebSocket server shutdown initiated");
    }
}

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    manager: web::Data<WsConnectionManager>,
) -> Result<HttpResponse, actix_web::Error> {
    if manager.is_shutting_down() {
        return Ok(HttpResponse::ServiceUnavailable()
            .json(ApiBuilder::error_response::<String>("server shutting down")));
    }

    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    let connection_id = uuid::Uuid::new_v4().to_string();
    info!(connection_id = %connection_id, "WebSocket connection established");

    let mut authenticated = false;
    let mut user_id: Option<String> = None;
    let mut subscribed_channels: Vec<String> = Vec::new();
    let connected_at = chrono::Utc::now().to_rfc3339();
    let mut last_heartbeat = std::time::Instant::now();

    actix_web::rt::spawn(async move {
        let heartbeat_interval = std::time::Duration::from_secs(10);
        let mut interval = tokio::time::interval(heartbeat_interval);
        let shutdown_flag = manager.shutdown_flag.clone();

        loop {
            tokio::select! {
                Some(msg) = msg_stream.next() => {
                    match msg {
                        Ok(actix_ws::Message::Text(text)) => {
                            match serde_json::from_str::<WsMessage>(&text) {
                                Ok(WsMessage::Authenticate { token }) => {
                                    if token.len() >= 8 {
                                        authenticated = true;
                                        user_id = Some(format!("user_{}", &token[..8]));
                                        info!(user_id = %user_id.as_ref().unwrap(), "WebSocket client authenticated");
                                        let _ = session
                                            .text(serde_json::to_string(&WsMessage::AuthSuccess).unwrap())
                                            .await;
                                    } else {
                                        warn!("WebSocket authentication failed: invalid token");
                                        let _ = session
                                            .text(
                                                serde_json::to_string(&WsMessage::AuthError {
                                                    message: "invalid token".to_string(),
                                                })
                                                .unwrap(),
                                            )
                                            .await;
                                    }
                                }
                                Ok(WsMessage::Subscribe { channel }) => {
                                    if !authenticated {
                                        let _ = session
                                            .text(
                                                serde_json::to_string(&WsMessage::Error {
                                                    message: "authentication required".to_string(),
                                                })
                                                .unwrap(),
                                            )
                                            .await;
                                        continue;
                                    }
                                    if !subscribed_channels.contains(&channel) {
                                        subscribed_channels.push(channel.clone());
                                        info!(channel = %channel, "Client subscribed to channel");
                                    }
                                    let _ = session
                                        .text(
                                            serde_json::to_string(&serde_json::json!({
                                                "type": "subscribed",
                                                "channel": channel
                                            }))
                                            .unwrap(),
                                        )
                                        .await;
                                }
                                Ok(WsMessage::Unsubscribe { channel }) => {
                                    subscribed_channels.retain(|c| c != &channel);
                                    let _ = session
                                        .text(
                                            serde_json::to_string(&serde_json::json!({
                                                "type": "unsubscribed",
                                                "channel": channel
                                            }))
                                            .unwrap(),
                                        )
                                        .await;
                                }
                                Ok(WsMessage::Ping) | Ok(WsMessage::Pong) => {
                                    last_heartbeat = std::time::Instant::now();
                                    let _ = session
                                        .text(serde_json::to_string(&WsMessage::Pong).unwrap())
                                        .await;
                                }
                                _ => {
                                    let _ = session
                                        .text(
                                            serde_json::to_string(&WsMessage::Error {
                                                message: "unknown message type".to_string(),
                                            })
                                            .unwrap(),
                                        )
                                        .await;
                                }
                            }
                        }
                        Ok(actix_ws::Message::Ping(bytes)) => {
                            last_heartbeat = std::time::Instant::now();
                            let _ = session.pong(&bytes).await;
                        }
                        Ok(actix_ws::Message::Pong(_)) => {
                            last_heartbeat = std::time::Instant::now();
                        }
                        Ok(actix_ws::Message::Close(_)) => {
                            info!(connection_id = %connection_id, "WebSocket connection closed by client");
                            break;
                        }
                        Err(e) => {
                            error!(error = %e, "WebSocket protocol error");
                            break;
                        }
                        _ => {}
                    }
                }
                _ = interval.tick() => {
                    let _ = session.ping(b"").await;

                    if last_heartbeat.elapsed() > std::time::Duration::from_secs(30) {
                        warn!(connection_id = %connection_id, "Client heartbeat timeout, closing connection");
                        break;
                    }

                    if shutdown_flag.load(Ordering::Relaxed) {
                        info!(connection_id = %connection_id, "Server shutting down, closing WebSocket");
                        let _ = session
                            .text(
                                serde_json::to_string(&serde_json::json!({
                                    "type": "shutdown",
                                    "message": "server shutting down"
                                }))
                                .unwrap(),
                            )
                            .await;
                        break;
                    }
                }
            }
        }

        info!(connection_id = %connection_id, "WebSocket connection closed");
    });

    Ok(response)
}

pub async fn ws_health() -> HttpResponse {
    HttpResponse::Ok().json(ApiBuilder::success_response(serde_json::json!({
        "status": "healthy",
        "protocol": "WebSocket",
        "version": "1.0.0"
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_ws_message_serialization() {
        let msg = WsMessage::Subscribe {
            channel: "waste:updates".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("subscribe"));

        let deserialized: WsMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            WsMessage::Subscribe { channel } => assert_eq!(channel, "waste:updates"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_auth_message() {
        let msg = WsMessage::Authenticate {
            token: "test_token_123".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: WsMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            WsMessage::Authenticate { token } => assert_eq!(token, "test_token_123"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_event_message() {
        let msg = WsMessage::Event {
            channel: "test".to_string(),
            data: serde_json::json!({"key": "value"}),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: WsMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            WsMessage::Event { channel, data } => {
                assert_eq!(channel, "test");
                assert_eq!(data, serde_json::json!({"key": "value"}));
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_connection_info() {
        let info = ConnectionInfo {
            connection_id: "conn-1".to_string(),
            user_id: Some("user-1".to_string()),
            subscribed_channels: vec!["channel-1".to_string()],
            connected_at: "2024-01-01T00:00:00Z".to_string(),
            last_heartbeat: "2024-01-01T00:00:10Z".to_string(),
        };
        let json = serde_json::to_string(&info).unwrap();
        let deserialized: ConnectionInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.connection_id, "conn-1");
        assert_eq!(deserialized.user_id, Some("user-1".to_string()));
    }
}
