//! Comprehensive compliance validator combining all checks.
//!
//! Provides a unified interface for compliance validation that combines
//! jurisdiction rules, certification checks, and business logic validation.

use chrono::Utc;
use super::{
    ComplianceError, ComplianceResult,
    certification::{Certification, CertificationRegistry},
    jurisdiction::{JurisdictionCode, JurisdictionRegistry, JurisdictionRules},
};
use crate::services::reporting::ReportService;

/// Complete compliance validation request.
#[derive(Debug, Clone)]
pub struct ComplianceValidationRequest {
    /// Participant identifier
    pub participant_id: String,
    
    /// Waste type
    pub waste_type: String,
    
    /// Operation type
    pub operation: String,
    
    /// Jurisdiction code
    pub jurisdiction: String,
    
    /// Quantity in kilograms
    pub quantity_kg: f64,
    
    /// Participant certifications (if known)
    pub participant_certifications: Option<Vec<Certification>>,
    
    /// Whether to generate compliance report
    pub generate_report: bool,
}

/// Complete compliance validation result.
#[derive(Debug, Clone)]
pub struct ComplianceValidationResult {
    /// Whether operation is compliant
    pub compliant: bool,
    
    /// Detailed validation messages
    pub messages: Vec<ComplianceMessage>,
    
    /// Recommended actions if non-compliant
    pub recommended_actions: Vec<String>,
    
    /// Expiry date of most restrictive requirement
    pub next_review_date: Option<chrono::DateTime<Utc>>,
    
    /// Report ID if report was generated
    pub report_id: Option<String>,
}

/// Individual compliance validation message.
#[derive(Debug, Clone)]
pub struct ComplianceMessage {
    /// Message level
    pub level: ComplianceLevel,
    
    /// Message category
    pub category: ComplianceCategory,
    
    /// Message text
    pub message: String,
    
    /// Whether this is a blocking issue
    pub blocking: bool,
}

/// Compliance message level.
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// Compliance message category.
#[derive(Debug, Clone)]
pub enum ComplianceCategory {
    Jurisdiction,
    Certification,
    Quantity,
    WasteType,
    Documentation,
    Reporting,
    Other,
}

/// Main compliance validator service.
pub struct ComplianceValidatorService {
    certification_registry: CertificationRegistry,
    jurisdiction_registry: JurisdictionRegistry,
    report_service: Option<Box<dyn ReportService>>,
}

impl ComplianceValidatorService {
    /// Create a new compliance validator service.
    pub fn new() -> Self {
        Self {
            certification_registry: CertificationRegistry::new(),
            jurisdiction_registry: JurisdictionRegistry::new(),
            report_service: None,
        }
    }
    
    /// Set the report service for compliance reporting.
    pub fn with_report_service(mut self, report_service: Box<dyn ReportService>) -> Self {
        self.report_service = Some(report_service);
        self
    }
    
    /// Add a certification to the registry.
    pub fn add_certification(&mut self, certification: Certification) {
        self.certification_registry.add_certification(certification);
    }
    
    /// Perform comprehensive compliance validation.
    pub async fn validate(
        &self,
        request: &ComplianceValidationRequest,
    ) -> ComplianceResult<ComplianceValidationResult> {
        let mut messages = Vec::new();
        
        // 1. Validate jurisdiction
        self.validate_jurisdiction(&request.jurisdiction, &mut messages);
        
        // 2. Validate waste type
        self.validate_waste_type(&request.waste_type, &request.jurisdiction, &mut messages);
        
        // 3. Validate quantity
        self.validate_quantity(request.quantity_kg, &request.jurisdiction, &mut messages);
        
        // 4. Validate certifications
        self.validate_certifications(request, &mut messages).await?;
        
        // 5. Validate operation type
        self.validate_operation(&request.operation, &mut messages);
        
        // Determine if compliant
        let blocking_errors = messages.iter()
            .any(|m| m.blocking && m.level == ComplianceLevel::Error);
        
        let compliant = !blocking_errors;
        
        // Generate recommended actions if non-compliant
        let recommended_actions = if !compliant {
            self.generate_recommended_actions(&messages)
        } else {
            Vec::new()
        };
        
        // Generate report if requested
        let report_id = if request.generate_report {
            self.generate_compliance_report(request, &messages).await.ok()
        } else {
            None
        };
        
        Ok(ComplianceValidationResult {
            compliant,
            messages,
            recommended_actions,
            next_review_date: self.calculate_next_review_date(request),
            report_id,
        })
    }
    
    fn validate_jurisdiction(&self, jurisdiction: &str, messages: &mut Vec<ComplianceMessage>) {
        match JurisdictionCode::parse(jurisdiction) {
            Ok(code) => {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Info,
                    category: ComplianceCategory::Jurisdiction,
                    message: format!("Valid jurisdiction code: {}", code.to_string()),
                    blocking: false,
                });
                
                if !self.jurisdiction_registry.has_rules(jurisdiction) {
                    messages.push(ComplianceMessage {
                        level: ComplianceLevel::Warning,
                        category: ComplianceCategory::Jurisdiction,
                        message: format!("No specific rules found for jurisdiction: {}", jurisdiction),
                        blocking: false,
                    });
                }
            }
            Err(e) => {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Error,
                    category: ComplianceCategory::Jurisdiction,
                    message: format!("Invalid jurisdiction code: {}", e),
                    blocking: true,
                });
            }
        }
    }
    
    fn validate_waste_type(&self, waste_type: &str, jurisdiction: &str, messages: &mut Vec<ComplianceMessage>) {
        if waste_type.is_empty() {
            messages.push(ComplianceMessage {
                level: ComplianceLevel::Error,
                category: ComplianceCategory::WasteType,
                message: "Waste type is required".to_string(),
                blocking: true,
            });
            return;
        }
        
        if let Ok(rules) = self.jurisdiction_registry.validate_jurisdiction(jurisdiction) {
            if rules.is_waste_type_permitted(waste_type) {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Info,
                    category: ComplianceCategory::WasteType,
                    message: format!("Waste type '{}' permitted in jurisdiction", waste_type),
                    blocking: false,
                });
            } else {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Error,
                    category: ComplianceCategory::WasteType,
                    message: format!("Waste type '{}' not permitted in jurisdiction '{}'", waste_type, jurisdiction),
                    blocking: true,
                });
            }
        }
    }
    
    fn validate_quantity(&self, quantity_kg: f64, jurisdiction: &str, messages: &mut Vec<ComplianceMessage>) {
        if quantity_kg <= 0.0 {
            messages.push(ComplianceMessage {
                level: ComplianceLevel::Error,
                category: ComplianceCategory::Quantity,
                message: "Quantity must be positive".to_string(),
                blocking: true,
            });
            return;
        }
        
        if quantity_kg > 10000.0 {
            messages.push(ComplianceMessage {
                level: ComplianceLevel::Error,
                category: ComplianceCategory::Quantity,
                message: "Quantity exceeds maximum system limit (10,000 kg)".to_string(),
                blocking: true,
            });
        }
        
        if let Ok(rules) = self.jurisdiction_registry.validate_jurisdiction(jurisdiction) {
            if let Err(e) = rules.check_quantity_compliance(quantity_kg) {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Error,
                    category: ComplianceCategory::Quantity,
                    message: format!("Quantity validation failed: {}", e),
                    blocking: true,
                });
            } else {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Info,
                    category: ComplianceCategory::Quantity,
                    message: format!("Quantity {} kg within jurisdiction limits", quantity_kg),
                    blocking: false,
                });
            }
        }
        
        if quantity_kg > 1000.0 {
            messages.push(ComplianceMessage {
                level: ComplianceLevel::Warning,
                category: ComplianceCategory::Quantity,
                message: "Large quantity requires additional documentation".to_string(),
                blocking: false,
            });
        }
    }
    
    async fn validate_certifications(
        &self,
        request: &ComplianceValidationRequest,
        messages: &mut Vec<ComplianceMessage>,
    ) -> ComplianceResult<()> {
        // Check if participant has valid certifications
        match self.certification_registry.find_valid_certification(
            &request.participant_id,
            &request.waste_type,
            &request.jurisdiction,
            request.quantity_kg,
        ) {
            Ok(certification) => {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Info,
                    category: ComplianceCategory::Certification,
                    message: format!("Valid certification found: {} (expires {})", 
                        certification.id, certification.expiry_date),
                    blocking: false,
                });
                
                // Check if certification is expiring soon
                let days_until_expiry = (certification.expiry_date - Utc::now()).num_days();
                if days_until_expiry <= 30 {
                    messages.push(ComplianceMessage {
                        level: ComplianceLevel::Warning,
                        category: ComplianceCategory::Certification,
                        message: format!("Certification expires in {} days", days_until_expiry),
                        blocking: false,
                    });
                }
            }
            Err(e) => {
                messages.push(ComplianceMessage {
                    level: ComplianceLevel::Error,
                    category: ComplianceCategory::Certification,
                    message: format!("Certification validation failed: {}", e),
                    blocking: true,
                });
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    fn validate_operation(&self, operation: &str, messages: &mut Vec<ComplianceMessage>) {
        let valid_operations = ["collection", "transfer", "processing", "disposal", "storage"];
        
        if operation.is_empty() {
            messages.push(ComplianceMessage {
                level: ComplianceLevel::Error,
                category: ComplianceCategory::Other,
                message: "Operation type is required".to_string(),
                blocking: true,
            });
            return;
        }
        
        if valid_operations.contains(&operation) {
            messages.push(ComplianceMessage {
                level: ComplianceLevel::Info,
                category: ComplianceCategory::Other,
                message: format!("Valid operation type: {}", operation),
                blocking: false,
            });
        } else {
            messages.push(ComplianceMessage {
                level: ComplianceLevel::Warning,
                category: ComplianceCategory::Other,
                message: format!("Unusual operation type: {}", operation),
                blocking: false,
            });
        }
    }
    
    fn generate_recommended_actions(&self, messages: &[ComplianceMessage]) -> Vec<String> {
        let mut actions = Vec::new();
        
        for message in messages {
            if message.blocking && message.level == ComplianceLevel::Error {
                match &message.category {
                    ComplianceCategory::Jurisdiction => {
                        actions.push("Contact jurisdiction authority for clarification".to_string());
                        actions.push("Verify jurisdiction code format (XX-YY)".to_string());
                    }
                    ComplianceCategory::Certification => {
                        actions.push("Obtain required certifications".to_string());
                        actions.push("Renew expired certifications".to_string());
                        actions.push("Ensure certifications cover the waste type and jurisdiction".to_string());
                    }
                    ComplianceCategory::Quantity => {
                        actions.push("Reduce quantity to within limits".to_string());
                        actions.push("Apply for special permit for larger quantities".to_string());
                    }
                    ComplianceCategory::WasteType => {
                        actions.push("Select a permitted waste type".to_string());
                        actions.push("Apply for waste type exception".to_string());
                    }
                    _ => {
                        actions.push("Review compliance requirements".to_string());
                        actions.push("Consult with compliance officer".to_string());
                    }
                }
            }
        }
        
        // Remove duplicates
        actions.sort();
        actions.dedup();
        
        actions
    }
    
    fn calculate_next_review_date(&self, request: &ComplianceValidationRequest) -> Option<chrono::DateTime<Utc>> {
        // Get the earliest expiry date from participant's certifications
        let certifications = self.certification_registry
            .get_participant_certifications(&request.participant_id);
        
        certifications.iter()
            .map(|c| c.expiry_date)
            .min()
    }
    
    async fn generate_compliance_report(
        &self,
        request: &ComplianceValidationRequest,
        messages: &[ComplianceMessage],
    ) -> ComplianceResult<String> {
        if let Some(report_service) = &self.report_service {
            // In a real implementation, this would generate a detailed compliance report
            // For now, return a placeholder
            Ok("compliance-report-123".to_string())
        } else {
            Err(ComplianceError::InternalError(
                "Report service not available".to_string(),
            ))
        }
    }
    
    /// Get expiring certifications for proactive notification.
    pub fn get_expiring_certifications(&self, within_days: i64) -> Vec<&Certification> {
        self.certification_registry.get_expiring_certifications(within_days)
    }
    
    /// Get jurisdiction rules for a specific jurisdiction.
    pub fn get_jurisdiction_rules(&self, jurisdiction: &str) -> Option<&JurisdictionRules> {
        self.jurisdiction_registry.get_rules(jurisdiction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    
    fn create_test_certification() -> Certification {
        Certification {
            id: "cert-123".to_string(),
            participant_id: "participant-123".to_string(),
            certification_type: "waste_management".to_string(),
            issuing_authority: "Test Agency".to_string(),
            issue_date: Utc::now() - Duration::days(365),
            expiry_date: Utc::now() + Duration::days(60),
            valid_jurisdictions: vec!["US-CA".to_string()],
            covered_waste_types: vec!["plastic".to_string(), "metal".to_string()],
            max_quantity_kg: Some(1000.0),
            active: true,
        }
    }
    
    fn create_test_request() -> ComplianceValidationRequest {
        ComplianceValidationRequest {
            participant_id: "participant-123".to_string(),
            waste_type: "plastic".to_string(),
            operation: "collection".to_string(),
            jurisdiction: "US-CA".to_string(),
            quantity_kg: 100.0,
            participant_certifications: None,
            generate_report: false,
        }
    }
    
    #[tokio::test]
    async fn test_comprehensive_validation_success() {
        let mut validator = ComplianceValidatorService::new();
        validator.add_certification(create_test_certification());
        
        let request = create_test_request();
        let result = validator.validate(&request).await;
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        assert!(validation_result.compliant);
        assert!(validation_result.recommended_actions.is_empty());
    }
    
    #[tokio::test]
    async fn test_comprehensive_validation_missing_certification() {
        let validator = ComplianceValidatorService::new(); // Empty registry
        
        let request = create_test_request();
        let result = validator.validate(&request).await;
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        assert!(!validation_result.compliant);
        assert!(!validation_result.recommended_actions.is_empty());
        assert!(validation_result.messages.iter().any(|m| 
            m.category == ComplianceCategory::Certification && m.blocking
        ));
    }
    
    #[tokio::test]
    async fn test_comprehensive_validation_invalid_jurisdiction() {
        let mut validator = ComplianceValidatorService::new();
        validator.add_certification(create_test_certification());
        
        let mut request = create_test_request();
        request.jurisdiction = "INVALID-CODE".to_string();
        
        let result = validator.validate(&request).await;
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        assert!(!validation_result.compliant);
        assert!(validation_result.messages.iter().any(|m| 
            m.category == ComplianceCategory::Jurisdiction && m.blocking
        ));
    }
    
    #[tokio::test]
    async fn test_comprehensive_validation_invalid_waste_type() {
        let mut validator = ComplianceValidatorService::new();
        
        // Add certification that doesn't cover glass
        let mut cert = create_test_certification();
        cert.covered_waste_types = vec!["plastic".to_string(), "metal".to_string()];
        validator.add_certification(cert);
        
        let mut request = create_test_request();
        request.waste_type = "glass".to_string(); // Not covered by certification
        
        let result = validator.validate(&request).await;
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        assert!(!validation_result.compliant);
        assert!(validation_result.messages.iter().any(|m| 
            m.category == ComplianceCategory::WasteType && m.blocking
        ));
    }
    
    #[tokio::test]
    async fn test_comprehensive_validation_excessive_quantity() {
        let mut validator = ComplianceValidatorService::new();
        validator.add_certification(create_test_certification());
        
        let mut request = create_test_request();
        request.quantity_kg = 1500.0; // Exceeds US-CA limit of 1000.0
        
        let result = validator.validate(&request).await;
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        assert!(!validation_result.compliant);
        assert!(validation_result.messages.iter().any(|m| 
            m.category == ComplianceCategory::Quantity && m.blocking
        ));
    }
    
    #[tokio::test]
    async fn test_comprehensive_validation_negative_quantity() {
        let mut validator = ComplianceValidatorService::new();
        validator.add_certification(create_test_certification());
        
        let mut request = create_test_request();
        request.quantity_kg = -10.0;
        
        let result = validator.validate(&request).await;
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        assert!(!validation_result.compliant);
        assert!(validation_result.messages.iter().any(|m| 
            m.category == ComplianceCategory::Quantity && m.blocking
        ));
    }
    
    #[tokio::test]
    async fn test_generate_recommended_actions() {
        let validator = ComplianceValidatorService::new();
        
        let messages = vec![
            ComplianceMessage {
                level: ComplianceLevel::Error,
                category: ComplianceCategory::Certification,
                message: "Missing certification".to_string(),
                blocking: true,
            },
            ComplianceMessage {
                level: ComplianceLevel::Error,
                category: ComplianceCategory::Quantity,
                message: "Quantity exceeds limit".to_string(),
                blocking: true,
            },
        ];
        
        let actions = validator.generate_recommended_actions(&messages);
        assert!(!actions.is_empty());
        assert!(actions.iter().any(|a| a.contains("certification")));
        assert!(actions.iter().any(|a| a.contains("quantity")));
    }
    
    #[test]
    fn test_get_expiring_certifications() {
        let mut validator = ComplianceValidatorService::new();
        
        // Add certification expiring soon
        let mut cert = create_test_certification();
        cert.expiry_date = Utc::now() + Duration::days(5);
        validator.add_certification(cert);
        
        // Add certification expiring later
        let mut cert = create_test_certification();
        cert.id = "cert-456".to_string();
        cert.expiry_date = Utc::now() + Duration::days(30);
        validator.add_certification(cert);
        
        let expiring = validator.get_expiring_certifications(10);
        assert_eq!(expiring.len(), 1);
        assert_eq!(expiring[0].id, "cert-123");
    }
    
    #[test]
    fn test_get_jurisdiction_rules() {
        let validator = ComplianceValidatorService::new();
        
        let rules = validator.get_jurisdiction_rules("US-CA");
        assert!(rules.is_some());
        
        let rules = validator.get_jurisdiction_rules("XX-YY");
        assert!(rules.is_none());
    }
}