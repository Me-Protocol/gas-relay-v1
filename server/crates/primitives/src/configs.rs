use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    /// This is the URL of the server
    pub server_url: String,
    /// This is the URL of the database
    pub db_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayingAccount {
    /// This is the URL of the server
    pub name: String,
    /// This is the URL of the database
    pub private_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayerConfig {
    /// This is the name of the chronicle server
    pub name: Option<String>,
    /// This is a list of all the indexer Config
    pub relaying_accounts: Vec<RelayingAccount>,
    /// Server config
    pub server: ServerConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chain {
    pub chain_id: u64,
    pub chain_name: String,
    pub chain_url: String,
}

impl Chain {
    pub fn new(chain_id: u64, chain_name: String, chain_url: String) -> Self {
        Self {
            chain_id,
            chain_name,
            chain_url,
        }
    }

    pub fn provider(&self) {}
}
