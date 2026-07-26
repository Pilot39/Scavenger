use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: String,
    pub category: String,
    pub description: String,
    pub framework: String,
    pub mandatory: bool,
    pub check_function: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceChecklist {
    pub id: String,
    pub requirements: Vec<ComplianceRequirement>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: ChecklistStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChecklistStatus {
    Draft,
    Active,
    Deprecated,
}

impl ComplianceChecklist {
    pub fn new(id: String) -> Self {
        Self {
            id,
            requirements: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: ChecklistStatus::Active,
        }
    }

    pub fn add_requirement(&mut self, requirement: ComplianceRequirement) {
        self.requirements.push(requirement);
        self.updated_at = Utc::now();
    }

    pub fn remove_requirement(&mut self, requirement_id: &str) {
        self.requirements.retain(|r| r.id != requirement_id);
        self.updated_at = Utc::now();
    }

    pub fn get_requirement(&self, requirement_id: &str) -> Option<&ComplianceRequirement> {
        self.requirements.iter().find(|r| r.id == requirement_id)
    }

    pub fn mandatory_count(&self) -> usize {
        self.requirements.iter().filter(|r| r.mandatory).count()
    }
}
