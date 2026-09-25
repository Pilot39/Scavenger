//! Compliance enforcement for regulatory requirements.
//!
//! This module handles certification validation, jurisdiction checks,
//! and regulatory boundary enforcement for waste management operations.
//!
//! # Key responsibilities
//! - Validate participant certifications (expiry, scope, jurisdiction)
//! - Enforce waste-type restrictions per jurisdiction
//! - Track and report compliance events for audit trails

pub mod certification;
pub mod jurisdiction;
pub mod validator;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};

/// Core compliance error type.
#[derive(Debug, Error)]
pub enum ComplianceError {
    #[error("Certification expired: {0}")]
    CertificationExpired(String),
    
    #[error("Invalid jurisdiction: {0}")]
    InvalidJurisdiction(String),
    
    #[error("Waste type not permitted: {0}")]
    WasteTypeNotPermitted(String),
    
    #[error("Missing required certification: {0}")]
    MissingCertification(String),
    
    #[error("Internal compliance error: {0}")]
    InternalError(String),
}

/// Result alias for compliance operations.
pub type ComplianceResult<T> = Result<T, ComplianceError>;

/// Core compliance check request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckRequest {
    /// Participant identifier
    pub participant_id: String,
    
    /// Waste type being processed
    pub waste_type: String,
    
    /// Operation type (collection, transfer, processing, disposal)
    pub operation: String,
    
    /// Jurisdiction code (ISO 3166-2 format)
    pub jurisdiction: String,
    
    /// Quantity in kilograms
    pub quantity_kg: f64,
}

/// Compliance check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    /// Whether the operation is permitted
    pub permitted: bool,
    
    /// List of compliance warnings (non-blocking)
    pub warnings: Vec<String>,
    
    /// List of compliance violations (blocking)
    pub violations: Vec<String>,
    
    /// Expiry of the most restrictive certification
    pub next_review_date: Option<DateTime<Utc>>,
    
    /// Jurisdiction-specific requirements
    pub jurisdiction_requirements: Vec<String>,
}

/// Main compliance validator.
#[derive(Debug, Clone)]
pub struct ComplianceValidator {
    /// Whether to enforce strict mode (reject on warnings)
    pub strict_mode: bool,
    
    /// Allowed waste types per jurisdiction
    pub waste_type_rules: Vec<WasteTypeRule>,
}

/// Rule defining allowed waste types in a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteTypeRule {
    pub jurisdiction: String,
    pub waste_types: Vec<String>,
    pub max_quantity_kg: Option<f64>,
    pub special_permit_required: bool,
}

impl ComplianceValidator {
    /// Create a new compliance validator with default rules.
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            waste_type_rules: vec![
                WasteTypeRule {
                    jurisdiction: "US-CA".to_string(),
                    waste_types: vec!["plastic".to_string(), "metal".to_string(), "paper".to_string()],
                    max_quantity_kg: Some(1000.0),
                    special_permit_required: false,
                },
                WasteTypeRule {
                    jurisdiction: "US-NY".to_string(),
                    waste_types: vec!["plastic".to_string(), "metal".to_string(), "paper".to_string(), "glass".to_string()],
                    max_quantity_kg: Some(500.0),
                    special_permit_required: true,
                },
                WasteTypeRule {
                    jurisdiction: "GB-LND".to_string(),
                    waste_types: vec!["plastic".to_string(), "metal".to_string(), "paper".to_string()],
                    max_quantity_kg: None,
                    special_permit_required: false,
                },
            ],
        }
    }
    
    /// Perform a compliance check for an operation.
    pub fn check_operation(&self, request: &ComplianceCheckRequest) -> ComplianceCheckResult {
        let mut result = ComplianceCheckResult {
            permitted: true,
            warnings: Vec::new(),
            violations: Vec::new(),
            next_review_date: None,
            jurisdiction_requirements: Vec::new(),
        };
        
        // Check jurisdiction
        self.check_jurisdiction(request, &mut result);
        
        // Check waste type
        self.check_waste_type(request, &mut result);
        
        // Check quantity limits
        self.check_quantity(request, &mut result);
        
        // Determine if operation is permitted
        result.permitted = result.violations.is_empty() && (!self.strict_mode || result.warnings.is_empty());
        
        result
    }
    
    fn check_jurisdiction(&self, request: &ComplianceCheckRequest, result: &mut ComplianceCheckResult) {
        if request.jurisdiction.is_empty() {
            result.violations.push("Jurisdiction is required".to_string());
        } else if !request.jurisdiction.contains('-') {
            result.warnings.push(format!("Jurisdiction format may be invalid: {}", request.jurisdiction));
        }
        
        // Add jurisdiction-specific requirements
        result.jurisdiction_requirements.push("Proper waste documentation required".to_string());
        result.jurisdiction_requirements.push("Periodic reporting mandatory".to_string());
    }
    
    fn check_waste_type(&self, request: &ComplianceCheckRequest, result: &mut ComplianceCheckResult) {
        if request.waste_type.is_empty() {
            result.violations.push("Waste type is required".to_string());
            return;
        }
        
        // Find jurisdiction rule
        let rule = self.waste_type_rules.iter()
            .find(|r| r.jurisdiction == request.jurisdiction);
            
        match rule {
            Some(rule) => {
                if !rule.waste_types.contains(&request.waste_type) {
                    result.violations.push(format!(
                        "Waste type '{}' not permitted in jurisdiction '{}'",
                        request.waste_type, request.jurisdiction
                    ));
                }
                
                if rule.special_permit_required {
                    result.warnings.push("Special permit may be required for this jurisdiction".to_string());
                }
            }
            None => {
                result.warnings.push(format!(
                    "No specific rules found for jurisdiction '{}', applying default restrictions",
                    request.jurisdiction
                ));
            }
        }
    }
    
    fn check_quantity(&self, request: &ComplianceCheckRequest, result: &mut ComplianceCheckResult) {
        if request.quantity_kg <= 0.0 {
            result.violations.push("Quantity must be positive".to_string());
            return;
        }
        
        if request.quantity_kg > 10000.0 {
            result.violations.push("Quantity exceeds maximum limit (10,000 kg)".to_string());
            return;
        }
        
        // Check jurisdiction-specific limits
        let rule = self.waste_type_rules.iter()
            .find(|r| r.jurisdiction == request.jurisdiction);
            
        if let Some(rule) = rule {
            if let Some(max_quantity) = rule.max_quantity_kg {
                if request.quantity_kg > max_quantity {
                    result.violations.push(format!(
                        "Quantity {} kg exceeds jurisdiction limit of {} kg",
                        request.quantity_kg, max_quantity
                    ));
                }
            }
        }
        
        if request.quantity_kg > 1000.0 {
            result.warnings.push("Large quantity requires additional documentation".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    #[test]
    fn test_compliance_validator_creation() {
        let validator = ComplianceValidator::new();
        assert!(!validator.strict_mode);
        assert!(!validator.waste_type_rules.is_empty());
    }
    
    #[test]
    fn test_valid_operation() {
        let validator = ComplianceValidator::new();
        let request = ComplianceCheckRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "plastic".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "US-CA".to_string(),
            quantity_kg: 100.0,
        };
        
        let result = validator.check_operation(&request);
        assert!(result.permitted);
        assert!(result.violations.is_empty());
    }
    
    #[test]
    fn test_invalid_waste_type() {
        let validator = ComplianceValidator::new();
        let request = ComplianceCheckRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "hazardous".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "US-CA".to_string(),
            quantity_kg: 100.0,
        };
        
        let result = validator.check_operation(&request);
        assert!(!result.permitted);
        assert!(!result.violations.is_empty());
        assert!(result.violations[0].contains("not permitted"));
    }
    
    #[test]
    fn test_excessive_quantity() {
        let validator = ComplianceValidator::new();
        let request = ComplianceCheckRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "plastic".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "US-CA".to_string(),
            quantity_kg: 1500.0,
        };
        
        let result = validator.check_operation(&request);
        assert!(!result.permitted);
        assert!(!result.violations.is_empty());
        assert!(result.violations[0].contains("exceeds jurisdiction limit"));
    }
    
    #[test]
    fn test_empty_jurisdiction() {
        let validator = ComplianceValidator::new();
        let request = ComplianceCheckRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "plastic".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "".to_string(),
            quantity_kg: 100.0,
        };
        
        let result = validator.check_operation(&request);
        assert!(!result.permitted);
        assert!(!result.violations.is_empty());
        assert!(result.violations[0].contains("Jurisdiction is required"));
    }
    
    #[test]
    fn test_negative_quantity() {
        let validator = ComplianceValidator::new();
        let request = ComplianceCheckRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "plastic".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "US-CA".to_string(),
            quantity_kg: -10.0,
        };
        
        let result = validator.check_operation(&request);
        assert!(!result.permitted);
        assert!(!result.violations.is_empty());
        assert!(result.violations[0].contains("must be positive"));
    }
    
    #[test]
    fn test_strict_mode() {
        let mut validator = ComplianceValidator::new();
        validator.strict_mode = true;
        
        let request = ComplianceCheckRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "plastic".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "US-NY".to_string(), // Requires special permit
            quantity_kg: 100.0,
        };
        
        let result = validator.check_operation(&request);
        // In strict mode, warnings also cause rejection
        assert!(!result.permitted);
        assert!(!result.warnings.is_empty());
        assert!(result.warnings[0].contains("Special permit"));
    }
    
    #[test]
    fn test_unknown_jurisdiction_warning() {
        let validator = ComplianceValidator::new();
        let request = ComplianceCheckRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "plastic".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "XX-YY".to_string(), // Unknown jurisdiction
            quantity_kg: 100.0,
        };
        
        let result = validator.check_operation(&request);
        assert!(result.permitted); // Should still be permitted (just warning)
        assert!(!result.warnings.is_empty());
        assert!(result.warnings[0].contains("No specific rules found"));
    }
}