use alloy::{
    network::{Ethereum, EthereumWallet},
    providers::{
        fillers::{FillProvider, JoinFill, WalletFiller},
        Identity, PendingTransactionBuilder, ProviderBuilder, RootProvider,
    },
    transports::http::{Client, Http},
};
use serde::{Deserialize, Serialize};

pub type RelayerSignerWithPrivatekey = FillProvider<
    JoinFill<Identity, WalletFiller<EthereumWallet>>,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    /// This is the URL of the server
    pub server_url: String,
    /// This is the URL of the database
    pub db_url: String,
    /// This is a key that would be used to post to the relay service
    pub access_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainsConfigParseable {
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
pub struct ChainsConfig {
    /// This is the name of the relayer server
    pub name: Option<String>,
    /// This is the rpc_url of the chain
    pub rpc_url: String,
    /// This is the chain_id of the chain
    pub chain_id: u64,
    /// This are the private keys controlling this relayers
    pub accounts_private_keys: RelayerAccounts,
    /// This is the trusted forwarder address for the chain
    pub trusted_forwarder: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayerConfig {
    /// This is the name of the relayer server
    pub name: Option<String>,
    /// This is the config for the chains
    pub chains: Vec<ChainsConfigParseable>,
    /// Server config
    pub server: ServerConfig,
    /// This is the MPSC channel size
    pub mpsc_channel_size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayerAccounts {
    keys: Vec<String>,
}

#[derive(Debug)]
pub struct PendingRequest {
    pub request_id: String,
    pub tx_pending: PendingTransactionBuilder<Http<Client>, Ethereum>,
}

impl ChainsConfig {
    pub fn new(
        name: Option<String>,
        rpc_url: String,
        chain_id: u64,
        accounts_private_keys: RelayerAccounts,
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

impl RelayerAccounts {
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }

    pub fn get_current_key(&mut self) -> String {
        let key = self.keys.pop().unwrap();
        self.keys.insert(0, key.clone());
        key
    }
}

impl ChainsConfigParseable {
    pub fn to_config(&self) -> ChainsConfig {
        ChainsConfig::new(
            self.name.clone(),
            self.rpc_url.clone(),
            self.chain_id,
            RelayerAccounts::new(self.accounts_private_keys.clone()),
            self.trusted_forwarder.clone(),
        )
    }
}
