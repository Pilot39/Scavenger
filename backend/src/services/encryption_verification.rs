use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionVerification {
    pub id: String,
    pub key_id: String,
    pub verified: bool,
    pub verified_at: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
    pub data_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMonitoring {
    pub total_encryptions: u64,
    pub total_decryptions: u64,
    pub verification_successes: u64,
    pub verification_failures: u64,
    pub key_rotations: u64,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl Default for EncryptionMonitoring {
    fn default() -> Self {
        Self {
            total_encryptions: 0,
            total_decryptions: 0,
            verification_successes: 0,
            verification_failures: 0,
            key_rotations: 0,
            last_activity: chrono::Utc::now(),
        }
    }
}

pub struct EncryptionMonitoringService {
    metrics: std::sync::Arc<std::sync::Mutex<EncryptionMonitoring>>,
    history: std::sync::Arc<std::sync::Mutex<Vec<EncryptionVerification>>>,
}

impl EncryptionMonitoringService {
    pub fn new() -> Self {
        Self {
            metrics: std::sync::Arc::new(std::sync::Mutex::new(EncryptionMonitoring::default())),
            history: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn record_encryption(&self) {
        self.metrics.lock().unwrap().total_encryptions += 1;
        self.metrics.lock().unwrap().last_activity = chrono::Utc::now();
    }

    pub fn record_decryption(&self) {
        self.metrics.lock().unwrap().total_decryptions += 1;
        self.metrics.lock().unwrap().last_activity = chrono::Utc::now();
    }

    pub fn record_verification(&self, success: bool) {
        if success {
            self.metrics.lock().unwrap().verification_successes += 1;
        } else {
            self.metrics.lock().unwrap().verification_failures += 1;
        }
        self.metrics.lock().unwrap().last_activity = chrono::Utc::now();
    }

    pub fn record_key_rotation(&self) {
        self.metrics.lock().unwrap().key_rotations += 1;
    }

    pub fn add_verification(&self, verification: EncryptionVerification) {
        self.history.lock().unwrap().push(verification);
    }

    pub fn get_metrics(&self) -> EncryptionMonitoring {
        self.metrics.lock().unwrap().clone()
    }

    pub fn get_history(&self, limit: usize) -> Vec<EncryptionVerification> {
        let history = self.history.lock().unwrap();
        history.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for EncryptionMonitoringService {
    fn default() -> Self {
        Self::new()
    }
}
