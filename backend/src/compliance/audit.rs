use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAuditEntry {
    pub id: String,
    pub compliance_id: String,
    pub action: String,
    pub actor: String,
    pub details: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

pub struct ComplianceAuditTrail {
    entries: Mutex<Vec<ComplianceAuditEntry>>,
}

impl ComplianceAuditTrail {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
        }
    }

    pub fn record(&self, compliance_id: &str, action: &str, actor: &str, details: &str) {
        let entry = ComplianceAuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            compliance_id: compliance_id.to_string(),
            action: action.to_string(),
            actor: actor.to_string(),
            details: details.to_string(),
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        };
        self.entries.lock().unwrap().push(entry);
    }

    pub fn get_entries(&self, limit: usize) -> Vec<ComplianceAuditEntry> {
        let entries = self.entries.lock().unwrap();
        entries.iter().rev().take(limit).cloned().collect()
    }

    pub fn get_entries_for_compliance(&self, compliance_id: &str) -> Vec<ComplianceAuditEntry> {
        let entries = self.entries.lock().unwrap();
        entries.iter().filter(|e| e.compliance_id == compliance_id).cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.entries.lock().unwrap().len()
    }
}

impl Default for ComplianceAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}
