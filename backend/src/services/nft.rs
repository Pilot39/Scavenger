use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTCertificate {
    pub token_id: String,
    pub participant_id: String,
    pub waste_type: String,
    pub weight: u128,
    pub timestamp: u64,
    pub metadata_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTMintRequest {
    pub participant_id: String,
    pub waste_type: String,
    pub weight: u128,
}

pub struct NFTManager;

impl NFTManager {
    pub fn mint_certificate(request: NFTMintRequest) -> NFTCertificate {
        let token_id = Self::generate_token_id(&request.participant_id);
        let timestamp = Self::current_timestamp();

        NFTCertificate {
            token_id,
            participant_id: request.participant_id,
            waste_type: request.waste_type,
            weight: request.weight,
            timestamp,
            metadata_uri: format!("ipfs://metadata/{}", timestamp),
        }
    }

    pub fn verify_certificate(certificate: &NFTCertificate) -> bool {
        !certificate.token_id.is_empty()
            && !certificate.participant_id.is_empty()
            && certificate.weight > 0
    }

    fn generate_token_id(participant_id: &str) -> String {
        format!("nft_{}", participant_id)
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
    fn test_mint_certificate() {
        let request = NFTMintRequest {
            participant_id: "recycler1".to_string(),
            waste_type: "plastic".to_string(),
            weight: 500,
        };

        let cert = NFTManager::mint_certificate(request);
        assert!(!cert.token_id.is_empty());
        assert_eq!(cert.waste_type, "plastic");
        assert_eq!(cert.weight, 500);
    }

    #[test]
    fn test_verify_certificate() {
        let cert = NFTCertificate {
            token_id: "nft_123".to_string(),
            participant_id: "recycler1".to_string(),
            waste_type: "metal".to_string(),
            weight: 1000,
            timestamp: 1234567890,
            metadata_uri: "ipfs://metadata/1234567890".to_string(),
        };

        assert!(NFTManager::verify_certificate(&cert));
    }

    #[test]
    fn test_verify_invalid_certificate() {
        let cert = NFTCertificate {
            token_id: "".to_string(),
            participant_id: "recycler1".to_string(),
            waste_type: "metal".to_string(),
            weight: 0,
            timestamp: 1234567890,
            metadata_uri: "ipfs://metadata/1234567890".to_string(),
        };

        assert!(!NFTManager::verify_certificate(&cert));
    }
}
