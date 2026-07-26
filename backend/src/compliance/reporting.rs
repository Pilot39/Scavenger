use serde::{Deserialize, Serialize};
use chrono::Utc;
use super::monitor::ComplianceMonitor;
use super::checklist::ComplianceChecklist;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub checklist_id: String,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub include_details: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_requirements: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
    pub compliance_score: f64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

pub struct ComplianceReportingService {
    monitor: ComplianceMonitor,
}

impl ComplianceReportingService {
    pub fn new(monitor: ComplianceMonitor) -> Self {
        Self { monitor }
    }

    pub fn generate_report(&self, checklist: &ComplianceChecklist) -> ReportSummary {
        let report = self.monitor.evaluate_checklist(checklist);
        ReportSummary {
            total_requirements: report.summary.total_checks,
            passed: report.summary.passed_checks,
            failed: report.summary.failed_checks,
            skipped: 0,
            compliance_score: report.summary.compliance_score,
            generated_at: Utc::now(),
        }
    }

    pub fn generate_detailed_report(&self, checklist: &ComplianceChecklist) -> (ReportSummary, Vec<super::monitor::CheckResult>) {
        let report = self.monitor.evaluate_checklist(checklist);
        (
            ReportSummary {
                total_requirements: report.summary.total_checks,
                passed: report.summary.passed_checks,
                failed: report.summary.failed_checks,
                skipped: 0,
                compliance_score: report.summary.compliance_score,
                generated_at: Utc::now(),
            },
            report.results,
        )
    }

    pub fn get_latest_report_summary(&self) -> Option<ReportSummary> {
        self.monitor.get_latest_report().map(|r| ReportSummary {
            total_requirements: r.summary.total_checks,
            passed: r.summary.passed_checks,
            failed: r.summary.failed_checks,
            skipped: 0,
            compliance_score: r.summary.compliance_score,
            generated_at: r.generated_at,
        })
    }
}
