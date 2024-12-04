use alloy::{
    providers::{ProviderBuilder, RootProvider},
    transports::http::{Client, Http},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    /// This is the URL of the server
    pub server_url: String,
    /// This is the URL of the database
    pub db_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainsConfig {
    /// This is the name of the relayer server
    pub name: Option<String>,
    /// This is the rpc_url of the chain
    pub rpc_url: String,
    /// This is the chain_id of the chain
    pub chain_id: u64,
    /// This are the private keys controlling this relayers
    pub accounts_private_keys: Vec<String>,
    /// This is the trusted forwarder address for the chain
    pub trusted_forwarder: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayerConfig {
    /// This is the name of the relayer server
    pub name: Option<String>,
    /// This is the config for the chains
    pub chains: Vec<ChainsConfig>,
    /// Server config
    pub server: ServerConfig,
}

impl ChainsConfig {
    pub fn new(
        name: Option<String>,
        rpc_url: String,
        chain_id: u64,
        accounts_private_keys: Vec<String>,
        trusted_forwarder: String,
    ) -> Self {
        Self {
            name,
            rpc_url,
            chain_id,
            accounts_private_keys,
            trusted_forwarder,
        }
    }

    pub fn chain_provider(&self) -> RootProvider<Http<Client>> {
        let provider = ProviderBuilder::new().on_http(self.rpc_url.parse().unwrap());
        provider
    }
}
