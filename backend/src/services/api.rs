use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

pub struct ApiBuilder;

impl ApiBuilder {
    pub fn success_response<T: Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Self::current_timestamp(),
        }
    }

    pub fn error_response<T: Serialize>(error: String) -> ApiResponse<T> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Self::current_timestamp(),
        }
    }

    pub fn paginated_response<T>(
        items: Vec<T>,
        total: u32,
        page: u32,
        limit: u32,
    ) -> PaginatedResponse<T> {
        PaginatedResponse {
            items,
            total,
            page,
            limit,
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiBuilder::success_response("test_data".to_string());
        assert!(response.success);
        assert_eq!(response.data, Some("test_data".to_string()));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_error_response() {
        let response: ApiResponse<String> =
            ApiBuilder::error_response("Something went wrong".to_string());
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("Something went wrong".to_string()));
    }

    #[test]
    fn test_paginated_response() {
        let items = vec!["item1".to_string(), "item2".to_string()];
        let response = ApiBuilder::paginated_response(items.clone(), 100, 1, 10);
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.total, 100);
        assert_eq!(response.page, 1);
        assert_eq!(response.limit, 10);
    }

    #[test]
    fn test_pagination_params() {
        let params = PaginationParams {
            page: 2,
            limit: 20,
        };
        assert_eq!(params.page, 2);
        assert_eq!(params.limit, 20);
    }

    #[test]
    fn test_api_error() {
        let error = ApiError {
            code: "INVALID_INPUT".to_string(),
            message: "Invalid input provided".to_string(),
            details: Some("Field 'weight' must be positive".to_string()),
        };
        assert_eq!(error.code, "INVALID_INPUT");
    }
}
