use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::types::{AppError, FieldError, ValidationError};

/// Wire format sent to API clients for every error response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorBody {
    /// Stable dot-separated code, e.g. `validation.field_error`
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// HTTP status (mirrored for clients that can't read status lines)
    pub status: u16,
    /// Category for structured handling, e.g. `validation`
    pub category: String,
    /// Present only for validation errors — per-field detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldError>>,
}

impl AppError {
    /// Serialise to the canonical JSON wire format.
    pub fn to_response_body(&self) -> ErrorResponse {
        ErrorResponse {
            error: ErrorBody {
                code: self.code(),
                message: self.to_string(),
                status: self.status_code(),
                category: self.category().as_str().to_string(),
                fields: validation_fields(self),
            },
        }
    }

    /// Serialise directly to `serde_json::Value` for embedding in larger responses.
    pub fn to_json(&self) -> Value {
        json!(self.to_response_body())
    }
}

fn validation_fields(e: &AppError) -> Option<Vec<FieldError>> {
    match e {
        AppError::Validation(ValidationError::Multiple(fields)) => Some(fields.clone()),
        AppError::Validation(ValidationError::Field { field, message }) => Some(vec![FieldError {
            field: field.clone(),
            message: message.clone(),
        }]),
        _ => None,
    }
}

// ── Actix-web ResponseError ───────────────────────────────────────────────────

use actix_web::{HttpResponse, ResponseError};

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(AppError::status_code(self))
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        let status = ResponseError::status_code(self);
        HttpResponse::build(status).json(self.to_response_body())
    }
}
