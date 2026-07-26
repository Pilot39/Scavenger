use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::services::api::{ApiBuilder, PaginatedResponse};
use crate::validation::{error_response, validate_pagination, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteResponse {
    pub id: String,
    pub waste_type: String,
    pub weight: u128,
    pub status: String,
    pub location: Option<String>,
    pub participant_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantResponse {
    pub id: String,
    pub name: String,
    pub role: String,
    pub location: Option<String>,
    pub reputation: u32,
    pub joined_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStatsResponse {
    pub total_wastes: u64,
    pub total_participants: u64,
    pub total_weight: u128,
    pub recycled_weight: u128,
    pub pending_approvals: u32,
    pub active_participants: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfoResponse {
    pub contract_id: String,
    pub network: String,
    pub version: String,
    pub last_updated: String,
    pub total_transactions: u64,
}

#[derive(Debug, Deserialize)]
pub struct WasteQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<String>,
    pub waste_type: Option<String>,
    pub participant_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ParticipantQueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub role: Option<String>,
    pub search: Option<String>,
}

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn query_string(req: &HttpRequest) -> String {
    let qs = req.query_string();
    if qs.is_empty() {
        "all".to_string()
    } else {
        qs.to_string()
    }
}

pub async fn list_wastes(
    req: HttpRequest,
    cache: web::Data<Cache>,
    query: web::Query<WasteQueryParams>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    let errors = validate_pagination(page, limit);
    if !errors.is_empty() {
        return error_response(&errors);
    }

    let cache_key = format!("contract:wastes:{}", query_string(&req));
    if let Some(cached) = cache.get(&cache_key) {
        if let Ok(response) = serde_json::from_slice::<PaginatedResponse<WasteResponse>>(&cached) {
            return HttpResponse::Ok()
                .insert_header(("X-Cache", "HIT"))
                .json(ApiBuilder::success_response(response));
        }
    }

    let mut items = vec![
        WasteResponse {
            id: "waste-001".to_string(),
            waste_type: "plastic".to_string(),
            weight: 100,
            status: "pending".to_string(),
            location: Some("40.7128,-74.0060".to_string()),
            participant_id: "participant-001".to_string(),
            created_at: now(),
            updated_at: now(),
        },
        WasteResponse {
            id: "waste-002".to_string(),
            waste_type: "metal".to_string(),
            weight: 250,
            status: "approved".to_string(),
            location: Some("34.0522,-118.2437".to_string()),
            participant_id: "participant-002".to_string(),
            created_at: now(),
            updated_at: now(),
        },
        WasteResponse {
            id: "waste-003".to_string(),
            waste_type: "glass".to_string(),
            weight: 75,
            status: "processing".to_string(),
            location: Some("51.5074,-0.1278".to_string()),
            participant_id: "participant-001".to_string(),
            created_at: now(),
            updated_at: now(),
        },
        WasteResponse {
            id: "waste-004".to_string(),
            waste_type: "paper".to_string(),
            weight: 50,
            status: "verified".to_string(),
            location: Some("48.8566,2.3522".to_string()),
            participant_id: "participant-003".to_string(),
            created_at: now(),
            updated_at: now(),
        },
    ];

    if let Some(ref status) = query.status {
        items.retain(|w| w.status == *status);
    }
    if let Some(ref waste_type) = query.waste_type {
        items.retain(|w| w.waste_type == *waste_type);
    }
    if let Some(ref pid) = query.participant_id {
        items.retain(|w| w.participant_id == *pid);
    }

    let total = items.len() as u32;
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(items.len());
    let page_items = if start < items.len() {
        items[start..end].to_vec()
    } else {
        Vec::new()
    };

    let response = ApiBuilder::paginated_response(page_items, total, page, limit);
    if let Ok(json) = serde_json::to_vec(&response) {
        cache.set(cache_key, json);
    }

    HttpResponse::Ok()
        .insert_header(("X-Cache", "MISS"))
        .json(ApiBuilder::success_response(response))
}

pub async fn get_waste(
    cache: web::Data<Cache>,
    path: web::Path<String>,
) -> HttpResponse {
    let waste_id = path.into_inner();
    let cache_key = format!("contract:waste:{}", waste_id);

    if let Some(cached) = cache.get(&cache_key) {
        if let Ok(response) = serde_json::from_slice::<WasteResponse>(&cached) {
            return HttpResponse::Ok()
                .insert_header(("X-Cache", "HIT"))
                .json(ApiBuilder::success_response(response));
        }
    }

    let waste = WasteResponse {
        id: waste_id.clone(),
        waste_type: "plastic".to_string(),
        weight: 100,
        status: "pending".to_string(),
        location: Some("40.7128,-74.0060".to_string()),
        participant_id: "participant-001".to_string(),
        created_at: now(),
        updated_at: now(),
    };

    if let Ok(json) = serde_json::to_vec(&waste) {
        cache.set(cache_key, json);
    }

    HttpResponse::Ok()
        .insert_header(("X-Cache", "MISS"))
        .json(ApiBuilder::success_response(waste))
}

pub async fn list_participants(
    req: HttpRequest,
    cache: web::Data<Cache>,
    query: web::Query<ParticipantQueryParams>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    let errors = validate_pagination(page, limit);
    if !errors.is_empty() {
        return error_response(&errors);
    }

    let cache_key = format!("contract:participants:{}", query_string(&req));
    if let Some(cached) = cache.get(&cache_key) {
        if let Ok(response) =
            serde_json::from_slice::<PaginatedResponse<ParticipantResponse>>(&cached)
        {
            return HttpResponse::Ok()
                .insert_header(("X-Cache", "HIT"))
                .json(ApiBuilder::success_response(response));
        }
    }

    let mut items = vec![
        ParticipantResponse {
            id: "participant-001".to_string(),
            name: "Green Recycling Co".to_string(),
            role: "collector".to_string(),
            location: Some("New York, NY".to_string()),
            reputation: 85,
            joined_at: now(),
        },
        ParticipantResponse {
            id: "participant-002".to_string(),
            name: "Eco Waste Management".to_string(),
            role: "processor".to_string(),
            location: Some("Los Angeles, CA".to_string()),
            reputation: 92,
            joined_at: now(),
        },
        ParticipantResponse {
            id: "participant-003".to_string(),
            name: "Sustainable Materials Inc".to_string(),
            role: "collector".to_string(),
            location: Some("London, UK".to_string()),
            reputation: 78,
            joined_at: now(),
        },
    ];

    if let Some(ref role) = query.role {
        items.retain(|p| p.role == *role);
    }
    if let Some(ref search) = query.search {
        items.retain(|p| p.name.to_lowercase().contains(&search.to_lowercase()));
    }

    let total = items.len() as u32;
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(items.len());
    let page_items = if start < items.len() {
        items[start..end].to_vec()
    } else {
        Vec::new()
    };

    let response = ApiBuilder::paginated_response(page_items, total, page, limit);
    if let Ok(json) = serde_json::to_vec(&response) {
        cache.set(cache_key, json);
    }

    HttpResponse::Ok()
        .insert_header(("X-Cache", "MISS"))
        .json(ApiBuilder::success_response(response))
}

pub async fn get_participant(
    cache: web::Data<Cache>,
    path: web::Path<String>,
) -> HttpResponse {
    let participant_id = path.into_inner();
    let cache_key = format!("contract:participant:{}", participant_id);

    if let Some(cached) = cache.get(&cache_key) {
        if let Ok(response) = serde_json::from_slice::<ParticipantResponse>(&cached) {
            return HttpResponse::Ok()
                .insert_header(("X-Cache", "HIT"))
                .json(ApiBuilder::success_response(response));
        }
    }

    let participant = ParticipantResponse {
        id: participant_id,
        name: "Green Recycling Co".to_string(),
        role: "collector".to_string(),
        location: Some("New York, NY".to_string()),
        reputation: 85,
        joined_at: now(),
    };

    if let Ok(json) = serde_json::to_vec(&participant) {
        cache.set(cache_key, json);
    }

    HttpResponse::Ok()
        .insert_header(("X-Cache", "MISS"))
        .json(ApiBuilder::success_response(participant))
}

pub async fn get_contract_stats(cache: web::Data<Cache>) -> HttpResponse {
    let cache_key = "contract:stats".to_string();

    if let Some(cached) = cache.get(&cache_key) {
        if let Ok(response) = serde_json::from_slice::<ContractStatsResponse>(&cached) {
            return HttpResponse::Ok()
                .insert_header(("X-Cache", "HIT"))
                .json(ApiBuilder::success_response(response));
        }
    }

    let stats = ContractStatsResponse {
        total_wastes: 1250,
        total_participants: 340,
        total_weight: 50000,
        recycled_weight: 35000,
        pending_approvals: 45,
        active_participants: 280,
    };

    if let Ok(json) = serde_json::to_vec(&stats) {
        cache.set(cache_key, json);
    }

    HttpResponse::Ok()
        .insert_header(("X-Cache", "MISS"))
        .json(ApiBuilder::success_response(stats))
}

pub async fn get_contract_info(cache: web::Data<Cache>) -> HttpResponse {
    let cache_key = "contract:info".to_string();

    if let Some(cached) = cache.get(&cache_key) {
        if let Ok(response) = serde_json::from_slice::<ContractInfoResponse>(&cached) {
            return HttpResponse::Ok()
                .insert_header(("X-Cache", "HIT"))
                .json(ApiBuilder::success_response(response));
        }
    }

    let info = ContractInfoResponse {
        contract_id: "CAZTLQY7YZ6J7XOFY6Q6Y6Q6Y6Q6Y6Q6Y6Q6Y6Q6Y6".to_string(),
        network: "testnet".to_string(),
        version: "1.0.0".to_string(),
        last_updated: now(),
        total_transactions: 15234,
    };

    if let Ok(json) = serde_json::to_vec(&info) {
        cache.set(cache_key, json);
    }

    HttpResponse::Ok()
        .insert_header(("X-Cache", "MISS"))
        .json(ApiBuilder::success_response(info))
}

pub async fn invalidate_waste_cache(
    cache: web::Data<Cache>,
    path: web::Path<String>,
) -> HttpResponse {
    let waste_id = path.into_inner();
    cache.invalidate(&format!("contract:waste:{}", waste_id));
    HttpResponse::Ok().json(ApiBuilder::success_response("cache invalidated"))
}

pub async fn invalidate_all_cache(cache: web::Data<Cache>) -> HttpResponse {
    cache.clear();
    HttpResponse::Ok().json(ApiBuilder::success_response("all cache invalidated"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_list_wastes_default_pagination() {
        let cache = Cache::new(60);
        let req = test::TestRequest::default().to_http_request();
        let query = web::Query(WasteQueryParams {
            page: None,
            limit: None,
            status: None,
            waste_type: None,
            participant_id: None,
            sort_by: None,
            sort_order: None,
        });
        let resp = list_wastes(req, web::Data::new(cache), query).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_list_wastes_invalid_pagination() {
        let cache = Cache::new(60);
        let req = test::TestRequest::default().to_http_request();
        let query = web::Query(WasteQueryParams {
            page: Some(0),
            limit: Some(0),
            status: None,
            waste_type: None,
            participant_id: None,
            sort_by: None,
            sort_order: None,
        });
        let resp = list_wastes(req, web::Data::new(cache), query).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_list_wastes_filter_by_status() {
        let cache = Cache::new(60);
        let req = test::TestRequest::default().to_http_request();
        let query = web::Query(WasteQueryParams {
            page: Some(1),
            limit: Some(10),
            status: Some("approved".to_string()),
            waste_type: None,
            participant_id: None,
            sort_by: None,
            sort_order: None,
        });
        let resp = list_wastes(req, web::Data::new(cache), query).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_waste() {
        let cache = Cache::new(60);
        let resp = get_waste(web::Data::new(cache), web::Path::from("waste-001".to_string())).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_contract_stats() {
        let cache = Cache::new(60);
        let resp = get_contract_stats(web::Data::new(cache)).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_cache_hit_miss() {
        let cache = Cache::new(60);
        let resp1 = get_contract_stats(web::Data::new(cache.clone())).await;
        assert_eq!(
            resp1.headers()
                .get("X-Cache")
                .and_then(|v| v.to_str().ok()),
            Some("MISS")
        );

        let resp2 = get_contract_stats(web::Data::new(cache)).await;
        assert_eq!(
            resp2.headers()
                .get("X-Cache")
                .and_then(|v| v.to_str().ok()),
            Some("HIT")
        );
    }
}
