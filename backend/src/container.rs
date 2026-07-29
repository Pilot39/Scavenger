//! Dependency injection container (#914).
//!
//! `AppContainer` is the single place where every service is constructed and
//! held behind an `Arc`.  Handlers receive individual `Arc<dyn Trait>` slices
//! of the container via actix-web's `web::Data` — none of them construct their
//! own dependencies.
//!
//! # Usage (in `main.rs`)
//! ```ignore
//! let container = AppContainer::from_env()
//!     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
//! let container = Arc::new(container);
//!
//! HttpServer::new(move || {
//!     App::new()
//!         .app_data(container.email.clone())
//!         .app_data(container.webhook.clone())
//!         // …
//! })
//! ```

use std::sync::Arc;

use crate::{
    cache::{Cache, CacheInvalidationManager},
    rpc::{StellarRpcClient, StellarRpcConfig},
    search::{SearchClient, SearchClientConfig},
    services::{
        AuditService, ArchivalService,
        EmailService, SendGridEmailService,
        FileSystemArchivalStorage,
        NotificationService, FirebaseNotificationService,
        ReportService, ReportingService,
        StorageService, S3StorageService,
        VerificationService, DefaultVerificationService,
        WebhookManager,
    },
};

/// Central DI container — built once at startup, then shared via `Arc`.
pub struct AppContainer {
    pub email: Arc<dyn EmailService>,
    pub notification: Arc<dyn NotificationService>,
    pub reporting: Arc<dyn ReportService>,
    pub storage: Arc<dyn StorageService>,
    pub webhook: Arc<WebhookManager>,
    pub cache: Cache,
    pub cache_invalidation: Arc<CacheInvalidationManager>,
    pub audit: Arc<AuditService>,
    pub verification: Arc<dyn VerificationService>,
    pub search: Arc<SearchClient>,
    pub archival: Arc<ArchivalService>,
    pub stellar: Arc<StellarRpcClient>,
}

impl AppContainer {
    /// Construct the container from environment variables.
    pub fn from_env() -> Result<Self, String> {
        let email_service: Arc<dyn EmailService> = Arc::new(SendGridEmailService::new(
            std::env::var("SENDGRID_API_KEY").unwrap_or_default(),
            std::env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@scavenger.io".to_string()),
        ));

        let notification_service: Arc<dyn NotificationService> =
            Arc::new(FirebaseNotificationService::new(
                std::env::var("FIREBASE_PROJECT_ID").unwrap_or_default(),
            ));

        let reporting_service: Arc<dyn ReportService> = Arc::new(ReportingService::new(
            std::env::var("STORAGE_PATH").unwrap_or_else(|_| "/tmp".to_string()),
        ));

        let storage_service: Arc<dyn StorageService> = Arc::new(S3StorageService::new(
            std::env::var("S3_BUCKET").unwrap_or_default(),
            std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
        ));

        let webhook_manager = Arc::new(WebhookManager::new());
        let cache = Cache::new(300);
        let cache_invalidation = Arc::new(CacheInvalidationManager::new());
        let audit_service = Arc::new(AuditService::new());
        let verification_service: Arc<dyn VerificationService> =
            Arc::new(DefaultVerificationService::new());

        // Stellar RPC
        let stellar_rpc_config = StellarRpcConfig::from_env();
        let stellar_client = Arc::new(
            StellarRpcClient::new(stellar_rpc_config)
                .map_err(|e| format!("Stellar RPC init failed: {e}"))?,
        );

        // Search client
        let search_config = SearchClientConfig {
            url: std::env::var("ELASTICSEARCH_URL")
                .unwrap_or_else(|_| "http://localhost:9200".to_string()),
            username: std::env::var("ELASTICSEARCH_USERNAME").ok(),
            password: std::env::var("ELASTICSEARCH_PASSWORD").ok(),
            timeout_seconds: 30,
            validate_certificates: true,
        };
        let search_client = Arc::new(
            SearchClient::new(search_config)
                .map_err(|e| format!("Search client init failed: {e}"))?,
        );

        // Archival service
        let archival_storage_path = std::env::var("ARCHIVAL_STORAGE_PATH")
            .unwrap_or_else(|_| "/tmp/archives".to_string());
        let archival_storage = Arc::new(FileSystemArchivalStorage::new(
            std::path::PathBuf::from(archival_storage_path),
        ));
        let archival_service = Arc::new(ArchivalService::new(archival_storage));

        Ok(Self {
            email: email_service,
            notification: notification_service,
            reporting: reporting_service,
            storage: storage_service,
            webhook: webhook_manager,
            cache,
            cache_invalidation,
            audit: audit_service,
            verification: verification_service,
            search: search_client,
            archival: archival_service,
            stellar: stellar_client,
        })
    }
}

// ── Fake implementations for unit tests ──────────────────────────────────────

#[cfg(test)]
pub mod fakes {
    //! In-memory fakes for every service trait.  Import these in unit tests
    //! instead of spinning up real connections.

    use std::collections::HashMap;
    use std::sync::Mutex;

    use crate::services::{
        email::{EmailError, TransactionalEmail, DigestEmail},
        notifications::{DeviceToken, NotificationError, NotificationPreference, PushNotification, ScheduledNotification},
        reporting::{Report, ReportError, ReportRequest, ReportTemplate, ScheduledReport},
        storage::{FileMetadata, SignedUrlRequest, StorageError, UploadRequest},
        verification::{Document, ParticipantVerification, VerificationChecklist},
        EmailService, NotificationService, ReportService, StorageService, VerificationService,
    };

    // ── Email ──────────────────────────────────────────────────────────────
    pub struct FakeEmailService {
        pub transactional: Mutex<Vec<TransactionalEmail>>,
    }

    impl FakeEmailService {
        pub fn new() -> Self {
            Self { transactional: Mutex::new(vec![]) }
        }
    }

    #[async_trait::async_trait]
    impl EmailService for FakeEmailService {
        async fn send_transactional(&self, email: TransactionalEmail) -> Result<String, EmailError> {
            self.transactional.lock().unwrap().push(email);
            Ok("fake-msg-id".to_string())
        }
        async fn send_digest(&self, _email: DigestEmail) -> Result<String, EmailError> {
            Ok("fake-digest-id".to_string())
        }
        async fn add_to_unsubscribe_list(&self, _email: &str) -> Result<(), EmailError> {
            Ok(())
        }
        async fn is_unsubscribed(&self, _email: &str) -> Result<bool, EmailError> {
            Ok(false)
        }
    }

    // ── Notifications ──────────────────────────────────────────────────────
    pub struct FakeNotificationService {
        pub sent: Mutex<Vec<(String, PushNotification)>>,
    }

    impl FakeNotificationService {
        pub fn new() -> Self {
            Self { sent: Mutex::new(vec![]) }
        }
    }

    #[async_trait::async_trait]
    impl NotificationService for FakeNotificationService {
        async fn register_device(&self, _token: DeviceToken) -> Result<String, NotificationError> {
            Ok("fake-device-id".to_string())
        }
        async fn send_notification(
            &self,
            device_token: &str,
            notification: PushNotification,
        ) -> Result<String, NotificationError> {
            self.sent.lock().unwrap().push((device_token.to_string(), notification));
            Ok("fake-message-id".to_string())
        }
        async fn set_preferences(&self, _pref: NotificationPreference) -> Result<(), NotificationError> {
            Ok(())
        }
        async fn get_preferences(&self, user_id: &str) -> Result<NotificationPreference, NotificationError> {
            Ok(NotificationPreference {
                user_id: user_id.to_string(),
                enabled: true,
                categories: vec![],
            })
        }
        async fn schedule_notification(&self, sn: ScheduledNotification) -> Result<String, NotificationError> {
            self.sent.lock().unwrap().push((sn.device_token, sn.notification));
            Ok("fake-scheduled-id".to_string())
        }
    }

    // ── Storage ────────────────────────────────────────────────────────────
    pub struct FakeStorageService {
        pub uploads: Mutex<Vec<String>>,
    }

    impl FakeStorageService {
        pub fn new() -> Self {
            Self { uploads: Mutex::new(vec![]) }
        }
    }

    #[async_trait::async_trait]
    impl StorageService for FakeStorageService {
        async fn upload_file(&self, request: UploadRequest) -> Result<FileMetadata, StorageError> {
            self.uploads.lock().unwrap().push(request.filename.clone());
            Ok(FileMetadata {
                file_id: "fake-id".to_string(),
                filename: request.filename,
                content_type: request.content_type,
                size: request.data.len() as u64,
                created_at: chrono::Utc::now().to_rfc3339(),
                url: "https://fake-storage/fake-id".to_string(),
            })
        }
        async fn delete_file(&self, file_id: &str) -> Result<(), StorageError> {
            self.uploads.lock().unwrap().retain(|k| k != file_id);
            Ok(())
        }
        async fn get_signed_url(&self, req: SignedUrlRequest) -> Result<String, StorageError> {
            Ok(format!("https://fake-storage/{}?signed=1", req.file_id))
        }
        async fn get_file_metadata(&self, file_id: &str) -> Result<FileMetadata, StorageError> {
            Ok(FileMetadata {
                file_id: file_id.to_string(),
                filename: "fake.pdf".to_string(),
                content_type: "application/pdf".to_string(),
                size: 0,
                created_at: chrono::Utc::now().to_rfc3339(),
                url: format!("https://fake-storage/{file_id}"),
            })
        }
    }

    // ── Reporting ──────────────────────────────────────────────────────────
    pub struct FakeReportService {
        pub generated: Mutex<Vec<String>>,
    }

    impl FakeReportService {
        pub fn new() -> Self {
            Self { generated: Mutex::new(vec![]) }
        }
    }

    #[async_trait::async_trait]
    impl ReportService for FakeReportService {
        async fn generate_report(&self, request: ReportRequest) -> Result<Report, ReportError> {
            self.generated.lock().unwrap().push(request.report_type.clone());
            Ok(Report {
                id: "fake-report-id".to_string(),
                report_type: request.report_type,
                format: request.format,
                status: "completed".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                file_url: Some("https://fake-storage/report.pdf".to_string()),
            })
        }
        async fn get_report(&self, report_id: &str) -> Result<Report, ReportError> {
            Ok(Report {
                id: report_id.to_string(),
                report_type: "waste".to_string(),
                format: "pdf".to_string(),
                status: "completed".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                file_url: None,
            })
        }
        async fn schedule_report(&self, _: ScheduledReport) -> Result<String, ReportError> {
            Ok("fake-schedule-id".to_string())
        }
        async fn get_templates(&self) -> Result<Vec<ReportTemplate>, ReportError> {
            Ok(vec![])
        }
        async fn cache_report(&self, _report_id: &str, _data: Vec<u8>) -> Result<(), ReportError> {
            Ok(())
        }
    }

    // ── Verification ───────────────────────────────────────────────────────
    pub struct FakeVerificationService {
        pub approvals: Mutex<Vec<String>>,
        pub rejections: Mutex<Vec<String>>,
    }

    impl FakeVerificationService {
        pub fn new() -> Self {
            Self {
                approvals: Mutex::new(vec![]),
                rejections: Mutex::new(vec![]),
            }
        }
    }

    fn make_fake_verification(participant_id: &str, status: crate::services::verification::VerificationStatus) -> ParticipantVerification {
        ParticipantVerification {
            participant_id: participant_id.to_string(),
            status,
            documents: vec![],
            checklist: VerificationChecklist {
                id: "fake-checklist".to_string(),
                participant_id: participant_id.to_string(),
                checks: HashMap::new(),
                completed_at: None,
            },
            notes: None,
            submitted_at: chrono::Utc::now(),
            reviewed_at: None,
            reviewed_by: None,
            retry_count: 0,
            last_retry_at: None,
        }
    }

    #[async_trait::async_trait]
    impl VerificationService for FakeVerificationService {
        async fn start_verification(&self, participant_id: String) -> Result<ParticipantVerification, String> {
            Ok(make_fake_verification(&participant_id, crate::services::verification::VerificationStatus::Pending))
        }
        async fn submit_document(&self, participant_id: String, doc_type: String, url: String) -> Result<Document, String> {
            Ok(Document {
                id: "fake-doc".to_string(),
                participant_id,
                doc_type,
                url,
                uploaded_at: chrono::Utc::now(),
                verified: false,
                verification_notes: None,
            })
        }
        async fn verify_document(&self, doc_id: String) -> Result<Document, String> {
            Ok(Document {
                id: doc_id,
                participant_id: "fake-participant".to_string(),
                doc_type: "id".to_string(),
                url: "https://fake/doc".to_string(),
                uploaded_at: chrono::Utc::now(),
                verified: true,
                verification_notes: None,
            })
        }
        async fn get_verification_status(&self, participant_id: String) -> Result<ParticipantVerification, String> {
            Ok(make_fake_verification(&participant_id, crate::services::verification::VerificationStatus::Pending))
        }
        async fn submit_checklist(&self, participant_id: String, checks: HashMap<String, bool>) -> Result<VerificationChecklist, String> {
            Ok(VerificationChecklist {
                id: "fake-checklist".to_string(),
                participant_id,
                checks,
                completed_at: Some(chrono::Utc::now()),
            })
        }
        async fn create_review_queue_item(&self, _participant_id: String) -> Result<String, String> {
            Ok("fake-queue-item".to_string())
        }
        async fn approve_participant(&self, participant_id: String, _reviewer_id: String) -> Result<(), String> {
            self.approvals.lock().unwrap().push(participant_id);
            Ok(())
        }
        async fn reject_participant(&self, participant_id: String, _reason: String, _reviewer_id: String) -> Result<(), String> {
            self.rejections.lock().unwrap().push(participant_id);
            Ok(())
        }
        async fn get_pending_reviews(&self) -> Result<Vec<ParticipantVerification>, String> {
            Ok(vec![])
        }
        async fn retry_verification(&self, participant_id: String) -> Result<ParticipantVerification, String> {
            Ok(make_fake_verification(&participant_id, crate::services::verification::VerificationStatus::Pending))
        }
        async fn send_approval_notification(&self, _participant_id: String) -> Result<(), String> {
            Ok(())
        }
        async fn send_rejection_notification(&self, _participant_id: String, _reason: String) -> Result<(), String> {
            Ok(())
        }
    }
}
