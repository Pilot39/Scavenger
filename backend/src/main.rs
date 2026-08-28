mod api;
mod services;

use actix_web::{web, App, HttpServer, HttpResponse};
use api::configure_api_routes;
use services::{
    EmailService, SendGridEmailService, NotificationService, FirebaseNotificationService,
    ReportService, ReportingService, StorageService, S3StorageService,
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let email_service = Arc::new(SendGridEmailService::new(
        std::env::var("SENDGRID_API_KEY").unwrap_or_default(),
        std::env::var("FROM_EMAIL").unwrap_or_else(|_| "noreply@scavenger.io".to_string()),
    ));

    let notification_service = Arc::new(FirebaseNotificationService::new(
        std::env::var("FIREBASE_PROJECT_ID").unwrap_or_default(),
    ));

    let reporting_service = Arc::new(ReportingService::new(
        std::env::var("STORAGE_PATH").unwrap_or_else(|_| "/tmp".to_string()),
    ));

    let storage_service = Arc::new(S3StorageService::new(
        std::env::var("S3_BUCKET").unwrap_or_default(),
        std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
    ));

    log::info!("Starting Scavenger Backend Server on 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(email_service.clone()))
            .app_data(web::Data::new(notification_service.clone()))
            .app_data(web::Data::new(reporting_service.clone()))
            .app_data(web::Data::new(storage_service.clone()))
            .configure(configure_api_routes)
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
