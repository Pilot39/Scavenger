use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BlockchainNetwork {
    Stellar,
    Ethereum,
    Polygon,
    Arbitrum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    pub network: BlockchainNetwork,
    pub rpc_url: String,
    pub contract_address: String,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainTransaction {
    pub source_chain: BlockchainNetwork,
    pub target_chain: BlockchainNetwork,
    pub transaction_id: String,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

pub struct ChainAbstraction;

impl ChainAbstraction {
    pub fn get_chain_config(network: BlockchainNetwork) -> ChainConfig {
        match network {
            BlockchainNetwork::Stellar => ChainConfig {
                network,
                rpc_url: "https://soroban-testnet.stellar.org".to_string(),
                contract_address: "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4".to_string(),
                chain_id: 0,
            },
            BlockchainNetwork::Ethereum => ChainConfig {
                network,
                rpc_url: "https://eth-mainnet.g.alchemy.com/v2/demo".to_string(),
                contract_address: "0x0000000000000000000000000000000000000000".to_string(),
                chain_id: 1,
            },
            BlockchainNetwork::Polygon => ChainConfig {
                network,
                rpc_url: "https://polygon-rpc.com".to_string(),
                contract_address: "0x0000000000000000000000000000000000000000".to_string(),
                chain_id: 137,
            },
            BlockchainNetwork::Arbitrum => ChainConfig {
                network,
                rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
                contract_address: "0x0000000000000000000000000000000000000000".to_string(),
                chain_id: 42161,
            },
        }
    }

    pub fn create_cross_chain_transaction(
        source: BlockchainNetwork,
        target: BlockchainNetwork,
    ) -> CrossChainTransaction {
        CrossChainTransaction {
            source_chain: source,
            target_chain: target,
            transaction_id: Self::generate_tx_id(),
            status: TransactionStatus::Pending,
        }
    }

    fn generate_tx_id() -> String {
        format!("tx_{}", uuid::Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stellar_config() {
        let config = ChainAbstraction::get_chain_config(BlockchainNetwork::Stellar);
        assert_eq!(config.network, BlockchainNetwork::Stellar);
        assert!(!config.rpc_url.is_empty());
    }

    #[test]
    fn test_get_ethereum_config() {
        let config = ChainAbstraction::get_chain_config(BlockchainNetwork::Ethereum);
        assert_eq!(config.network, BlockchainNetwork::Ethereum);
        assert_eq!(config.chain_id, 1);
    }

    #[test]
    fn test_get_polygon_config() {
        let config = ChainAbstraction::get_chain_config(BlockchainNetwork::Polygon);
        assert_eq!(config.network, BlockchainNetwork::Polygon);
        assert_eq!(config.chain_id, 137);
    }

    #[test]
    fn test_create_cross_chain_transaction() {
        let tx = ChainAbstraction::create_cross_chain_transaction(
            BlockchainNetwork::Stellar,
            BlockchainNetwork::Ethereum,
        );
        assert_eq!(tx.source_chain, BlockchainNetwork::Stellar);
        assert_eq!(tx.target_chain, BlockchainNetwork::Ethereum);
        assert_eq!(tx.status, TransactionStatus::Pending);
    }

    #[test]
    fn test_transaction_id_generation() {
        let tx1 = ChainAbstraction::create_cross_chain_transaction(
            BlockchainNetwork::Stellar,
            BlockchainNetwork::Polygon,
        );
        let tx2 = ChainAbstraction::create_cross_chain_transaction(
            BlockchainNetwork::Stellar,
            BlockchainNetwork::Polygon,
        );
        assert_ne!(tx1.transaction_id, tx2.transaction_id);
    }
}
