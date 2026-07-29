use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAlert {
    pub id: String,
    pub rule_id: String,
    pub severity: AlertSeverity,
    pub status: AlertStatus,
    pub message: String,
    pub details: serde_json::Value,
    pub triggered_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub condition: AlertCondition,
    pub enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    pub metric: String,
    pub operator: String,
    pub threshold: f64,
    pub window_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    Open,
    Acknowledged,
    Resolved,
    Suppressed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    pub name: String,
    pub channel_type: String,
    pub destination: String,
    pub enabled: bool,
}

pub struct ComplianceAlertingService {
    alerts: Vec<ComplianceAlert>,
    rules: Vec<AlertRule>,
    channels: Vec<AlertChannel>,
}

impl ComplianceAlertingService {
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            rules: Vec::new(),
            channels: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.push(rule);
    }

    pub fn evaluate(&mut self, metric_name: &str, value: f64) -> Option<ComplianceAlert> {
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            if rule.condition.metric == metric_name {
                let triggered = match rule.condition.operator.as_str() {
                    ">" => value > rule.condition.threshold,
                    "<" => value < rule.condition.threshold,
                    ">=" => value >= rule.condition.threshold,
                    "<=" => value <= rule.condition.threshold,
                    "==" => value == rule.condition.threshold,
                    _ => false,
                };
                if triggered {
                    let alert = ComplianceAlert {
                        id: uuid::Uuid::new_v4().to_string(),
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        status: AlertStatus::Open,
                        message: format!("{} triggered: {} {}", rule.name, metric_name, value),
                        details: serde_json::json!({"value": value, "threshold": rule.condition.threshold}),
                        triggered_at: Utc::now(),
                        resolved_at: None,
                    };
                    self.alerts.push(alert.clone());
                    return Some(alert);
                }
            }
        }
        None
    }

    pub fn add_channel(&mut self, channel: AlertChannel) {
        self.channels.push(channel);
    }

    pub fn get_active_alerts(&self) -> Vec<&ComplianceAlert> {
        self.alerts.iter().filter(|a| a.status == AlertStatus::Open).collect()
    }

    pub fn acknowledge_alert(&mut self, alert_id: &str) -> Option<()> {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.status = AlertStatus::Acknowledged;
            Some(())
        } else {
            None
        }
    }

    pub fn resolve_alert(&mut self, alert_id: &str) -> Option<()> {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(Utc::now());
            Some(())
        } else {
            None
        }
    }

    pub fn get_rules(&self) -> &Vec<AlertRule> {
        &self.rules
    }

    pub fn get_channels(&self) -> &Vec<AlertChannel> {
        &self.channels
    }
}

impl Default for ComplianceAlertingService {
    fn default() -> Self {
        Self::new()
    }
}
