use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub url: String,
    pub events: Vec<WebhookEvent>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebhookEvent {
    WasteRegistered,
    WasteTransferred,
    WasteVerified,
    IncentiveCreated,
    IncentiveUpdated,
    RewardDistributed,
    ParticipantRegistered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub id: String,
    pub event: WebhookEvent,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<WebhookEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWebhookRequest {
    pub url: Option<String>,
    pub events: Option<Vec<WebhookEvent>>,
    pub active: Option<bool>,
}

pub struct WebhookManager {
    webhooks: std::sync::Arc<std::sync::Mutex<HashMap<String, Webhook>>>,
}

impl WebhookManager {
    pub fn new() -> Self {
        Self {
            webhooks: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    pub fn create(&self, req: CreateWebhookRequest) -> Webhook {
        let webhook = Webhook {
            id: Uuid::new_v4().to_string(),
            url: req.url,
            events: req.events,
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            secret: Uuid::new_v4().to_string(),
        };

        let mut webhooks = self.webhooks.lock().unwrap();
        webhooks.insert(webhook.id.clone(), webhook.clone());
        webhook
    }

    pub fn get(&self, id: &str) -> Option<Webhook> {
        let webhooks = self.webhooks.lock().unwrap();
        webhooks.get(id).cloned()
    }

    pub fn list(&self) -> Vec<Webhook> {
        let webhooks = self.webhooks.lock().unwrap();
        webhooks.values().cloned().collect()
    }

    pub fn update(&self, id: &str, req: UpdateWebhookRequest) -> Option<Webhook> {
        let mut webhooks = self.webhooks.lock().unwrap();
        webhooks.get_mut(id).map(|webhook| {
            if let Some(url) = req.url {
                webhook.url = url;
            }
            if let Some(events) = req.events {
                webhook.events = events;
            }
            if let Some(active) = req.active {
                webhook.active = active;
            }
            webhook.updated_at = Utc::now();
            webhook.clone()
        })
    }

    pub fn delete(&self, id: &str) -> bool {
        let mut webhooks = self.webhooks.lock().unwrap();
        webhooks.remove(id).is_some()
    }

    pub fn get_active_webhooks(&self, event: &WebhookEvent) -> Vec<Webhook> {
        let webhooks = self.webhooks.lock().unwrap();
        webhooks
            .values()
            .filter(|w| w.active && w.events.contains(event))
            .cloned()
            .collect()
    }

    pub async fn trigger(&self, event: WebhookEvent, data: serde_json::Value) {
        let webhooks = self.get_active_webhooks(&event);
        let payload = WebhookPayload {
            id: Uuid::new_v4().to_string(),
            event,
            timestamp: Utc::now(),
            data,
        };

        for webhook in webhooks {
            let payload = payload.clone();
            tokio::spawn(async move {
                let _ = send_webhook(&webhook, &payload).await;
            });
        }
    }
}

async fn send_webhook(webhook: &Webhook, payload: &WebhookPayload) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(payload)?;
    
    client
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .header("X-Webhook-Secret", &webhook.secret)
        .body(body)
        .send()
        .await?;

    Ok(())
}
