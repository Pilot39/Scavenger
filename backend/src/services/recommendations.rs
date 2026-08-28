use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteRecommendation {
    pub waste_type: String,
    pub confidence_score: f64,
    pub collection_location: (f64, f64),
    pub estimated_reward: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRequest {
    pub participant_id: String,
    pub location: (f64, f64),
    pub waste_history: Vec<String>,
}

pub struct RecommendationEngine;

impl RecommendationEngine {
    pub fn generate_recommendations(
        request: RecommendationRequest,
    ) -> Vec<WasteRecommendation> {
        let mut recommendations = Vec::new();

        // Simple ML-based recommendation logic
        let waste_types = vec!["plastic", "metal", "paper", "glass"];
        let base_reward = 100u128;

        for waste_type in waste_types {
            let confidence = Self::calculate_confidence(&request.waste_history, waste_type);
            if confidence > 0.3 {
                recommendations.push(WasteRecommendation {
                    waste_type: waste_type.to_string(),
                    confidence_score: confidence,
                    collection_location: request.location,
                    estimated_reward: (base_reward as f64 * confidence) as u128,
                });
            }
        }

        recommendations.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap());
        recommendations
    }

    fn calculate_confidence(history: &[String], waste_type: &str) -> f64 {
        let count = history.iter().filter(|w| w.contains(waste_type)).count();
        let base_confidence = (count as f64) / (history.len().max(1) as f64);
        (base_confidence * 0.8 + 0.2).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_recommendations() {
        let request = RecommendationRequest {
            participant_id: "user1".to_string(),
            location: (40.7128, -74.0060),
            waste_history: vec![
                "plastic".to_string(),
                "plastic".to_string(),
                "metal".to_string(),
            ],
        };

        let recommendations = RecommendationEngine::generate_recommendations(request);
        assert!(!recommendations.is_empty());
        assert!(recommendations[0].confidence_score > 0.3);
    }

    #[test]
    fn test_confidence_calculation() {
        let history = vec!["plastic".to_string(), "plastic".to_string(), "metal".to_string()];
        let confidence = RecommendationEngine::calculate_confidence(&history, "plastic");
        assert!(confidence > 0.5);
    }

    #[test]
    fn test_empty_history() {
        let request = RecommendationRequest {
            participant_id: "user2".to_string(),
            location: (51.5074, -0.1278),
            waste_history: vec![],
        };

        let recommendations = RecommendationEngine::generate_recommendations(request);
        assert!(recommendations.len() <= 4);
    }
}
