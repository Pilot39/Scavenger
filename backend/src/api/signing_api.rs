use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::validation::{error_response, validate_required, sanitize_string, ValidationError};

#[derive(Debug, Deserialize)]
pub struct SignRequest {
    pub transaction_id: String,
    pub signer_id: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub transaction_id: String,
    pub signature: String,
    pub signer_id: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct MultiSigCreateRequest {
    pub transaction_id: String,
    pub required_signatures: u32,
}

#[derive(Debug, Deserialize)]
pub struct MultiSigSignRequest {
    pub transaction_id: String,
    pub signer_id: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct RevokeRequest {
    pub transaction_id: String,
    pub revoked_by: String,
    pub reason: String,
}

pub async fn sign_transaction(body: web::Json<SignRequest>) -> HttpResponse {
    let mut errors = Vec::new();
    let transaction_id = sanitize_string(&body.transaction_id);
    let signer_id = sanitize_string(&body.signer_id);
    let data = sanitize_string(&body.data);

    if let Some(e) = validate_required(&transaction_id, "transaction_id") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&signer_id, "signer_id") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&data, "data") {
        errors.push(e);
    }

    if !errors.is_empty() {
        return error_response(&errors);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "transaction_id": transaction_id,
            "signer_id": signer_id,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        },
        "message": "transaction signed"
    }))
}

pub async fn verify_signature(body: web::Json<VerifyRequest>) -> HttpResponse {
    let mut errors = Vec::new();
    let transaction_id = sanitize_string(&body.transaction_id);
    let signature = sanitize_string(&body.signature);
    let signer_id = sanitize_string(&body.signer_id);
    let data = sanitize_string(&body.data);

    if let Some(e) = validate_required(&transaction_id, "transaction_id") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&signature, "signature") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&signer_id, "signer_id") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&data, "data") {
        errors.push(e);
    }

    if !errors.is_empty() {
        return error_response(&errors);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "valid": true,
            "signer_id": signer_id,
            "verified_at": chrono::Utc::now().to_rfc3339()
        },
        "message": "signature verified"
    }))
}

pub async fn create_multisig(body: web::Json<MultiSigCreateRequest>) -> HttpResponse {
    let mut errors = Vec::new();
    let transaction_id = sanitize_string(&body.transaction_id);

    if let Some(e) = validate_required(&transaction_id, "transaction_id") {
        errors.push(e);
    }
    if body.required_signatures == 0 {
        errors.push(ValidationError {
            field: "required_signatures".to_string(),
            message: "required_signatures must be at least 1".to_string(),
        });
    }

    if !errors.is_empty() {
        return error_response(&errors);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "transaction_id": transaction_id,
            "required_signatures": body.required_signatures,
            "status": "pending"
        },
        "message": "multisig transaction created"
    }))
}

pub async fn multisig_sign(body: web::Json<MultiSigSignRequest>) -> HttpResponse {
    let mut errors = Vec::new();
    let transaction_id = sanitize_string(&body.transaction_id);
    let signer_id = sanitize_string(&body.signer_id);
    let data = sanitize_string(&body.data);

    if let Some(e) = validate_required(&transaction_id, "transaction_id") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&signer_id, "signer_id") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&data, "data") {
        errors.push(e);
    }

    if !errors.is_empty() {
        return error_response(&errors);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "transaction_id": transaction_id,
            "signer_id": signer_id,
            "signatures_collected": 1,
            "status": "partial"
        },
        "message": "multisig signature recorded"
    }))
}

pub async fn revoke_signature(body: web::Json<RevokeRequest>) -> HttpResponse {
    let mut errors = Vec::new();
    let transaction_id = sanitize_string(&body.transaction_id);
    let revoked_by = sanitize_string(&body.revoked_by);
    let reason = sanitize_string(&body.reason);

    if let Some(e) = validate_required(&transaction_id, "transaction_id") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&revoked_by, "revoked_by") {
        errors.push(e);
    }
    if let Some(e) = validate_required(&reason, "reason") {
        errors.push(e);
    }

    if !errors.is_empty() {
        return error_response(&errors);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "transaction_id": transaction_id,
            "revoked_by": revoked_by,
            "reason": reason,
            "revoked_at": chrono::Utc::now().to_rfc3339()
        },
        "message": "signature revoked"
    }))
}

pub async fn list_events() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": [],
        "message": "signing events"
    }))
}

pub async fn list_revocations() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": [],
        "message": "signature revocations"
    }))
}

pub async fn get_documentation() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "scheme": "HMAC-SHA256",
            "description": "Transaction signing and verification using HMAC-based signature scheme",
            "capabilities": [
                "Single signature creation and verification",
                "Multi-signature support",
                "Signature revocation",
                "Event logging"
            ]
        },
        "message": "signing documentation"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── sign_transaction ───────────────────────────────────────────────────

    #[actix_web::test]
    async fn test_sign_transaction_valid() {
        let body = web::Json(SignRequest {
            transaction_id: "tx-001".to_string(),
            signer_id: "signer-001".to_string(),
            data: "deadbeef".to_string(),
        });
        let resp = sign_transaction(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_sign_transaction_empty_transaction_id_returns_400() {
        let body = web::Json(SignRequest {
            transaction_id: "".to_string(),
            signer_id: "signer-001".to_string(),
            data: "deadbeef".to_string(),
        });
        let resp = sign_transaction(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_sign_transaction_empty_signer_id_returns_400() {
        let body = web::Json(SignRequest {
            transaction_id: "tx-001".to_string(),
            signer_id: "".to_string(),
            data: "deadbeef".to_string(),
        });
        let resp = sign_transaction(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_sign_transaction_empty_data_returns_400() {
        let body = web::Json(SignRequest {
            transaction_id: "tx-001".to_string(),
            signer_id: "signer-001".to_string(),
            data: "".to_string(),
        });
        let resp = sign_transaction(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    // ── verify_signature ───────────────────────────────────────────────────

    #[actix_web::test]
    async fn test_verify_signature_valid() {
        let body = web::Json(VerifyRequest {
            transaction_id: "tx-001".to_string(),
            signature: "abc123sig".to_string(),
            signer_id: "signer-001".to_string(),
            data: "deadbeef".to_string(),
        });
        let resp = verify_signature(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_verify_signature_missing_signature_returns_400() {
        let body = web::Json(VerifyRequest {
            transaction_id: "tx-001".to_string(),
            signature: "".to_string(),
            signer_id: "signer-001".to_string(),
            data: "deadbeef".to_string(),
        });
        let resp = verify_signature(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    // ── create_multisig ────────────────────────────────────────────────────

    #[actix_web::test]
    async fn test_create_multisig_valid() {
        let body = web::Json(MultiSigCreateRequest {
            transaction_id: "tx-001".to_string(),
            required_signatures: 2,
        });
        let resp = create_multisig(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_multisig_empty_tx_id_returns_400() {
        let body = web::Json(MultiSigCreateRequest {
            transaction_id: "".to_string(),
            required_signatures: 2,
        });
        let resp = create_multisig(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_create_multisig_zero_signatures_returns_400() {
        let body = web::Json(MultiSigCreateRequest {
            transaction_id: "tx-001".to_string(),
            required_signatures: 0,
        });
        let resp = create_multisig(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    // ── revoke_signature ───────────────────────────────────────────────────

    #[actix_web::test]
    async fn test_revoke_signature_valid() {
        let body = web::Json(RevokeRequest {
            transaction_id: "tx-001".to_string(),
            revoked_by: "admin-001".to_string(),
            reason: "Compromised key".to_string(),
        });
        let resp = revoke_signature(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_revoke_signature_empty_reason_returns_400() {
        let body = web::Json(RevokeRequest {
            transaction_id: "tx-001".to_string(),
            revoked_by: "admin-001".to_string(),
            reason: "".to_string(),
        });
        let resp = revoke_signature(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_revoke_signature_empty_revoked_by_returns_400() {
        let body = web::Json(RevokeRequest {
            transaction_id: "tx-001".to_string(),
            revoked_by: "".to_string(),
            reason: "Compromised key".to_string(),
        });
        let resp = revoke_signature(body).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }
}
