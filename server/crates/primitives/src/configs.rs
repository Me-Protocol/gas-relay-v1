use alloy::{
    network::{Ethereum, EthereumWallet},
    providers::{
        fillers::{FillProvider, JoinFill, WalletFiller},
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
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

    pub fn chain_provider_with_private_key(&self) -> RelayerSignerWithPrivatekey {
        let rand_private_key: PrivateKeySigner =
            self.accounts_private_keys[0].clone().parse().unwrap();
        let wallet = EthereumWallet::from(rand_private_key);
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .on_http(self.rpc_url.parse().unwrap());

        provider
    }
}
