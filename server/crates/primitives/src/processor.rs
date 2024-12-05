//! This file would be responsible for all the blockchain related operations in the gasless-relayer server.
use std::str::FromStr;

use crate::configs::ChainsConfig;
use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::{aliases::U48, Address, Bytes, FixedBytes, U256},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        PendingTransactionBuilder, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    sol,
    transports::http::{Client, Http},
};
use TrustedForwarderContract::TrustedForwarderContractInstance;

/// This struct would be responsible for processing all the requests to be sent to the blockchain.
/// struct would also be resonsible for waiting for the transaction to be mined
pub struct Processor {
    /// The chain to which the requests would be sent
    pub chains_config: ChainsConfig,
    /// This is the address of the trusted forwarder contract
    pub trusted_forwarder: Address,
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TrustedForwarderContract,
    "src/contract-artifacts/TrustedForwarder.json"
);

#[derive(Debug)]
pub struct ForwardRequestData {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub gas: U256,
    pub deadline: U48,
    pub data: Bytes,
    pub nonce: U256,
    pub signature: Bytes,
}

impl Processor {
    pub fn new(chains_config: ChainsConfig, trusted_forwarder: Address) -> Self {
        Self {
            chains_config,
            trusted_forwarder,
        }
    }

    /// This is function would be responsible for processing a single request
    /// this would just send the request to the chain and wait for it to be mined
    /// after this has been sent, this tx_hash would be used to wait for the transaction to be mined
    /// on the monitoring thread
    ///
    /// # Arguments
    /// - `self`
    /// - `request` - The request to be processed
    /// - `chain` - The chain to which the request would be sent
    pub async fn process_request(
        &self,
        request: ForwardRequestData,
    ) -> PendingTransactionBuilder<Http<Client>, Ethereum> {
        let trusted_forwarder_contract = self.get_trusted_forwarder();
        let req = trusted_forwarder_contract.execute(request.into());
        req.send().await.unwrap()
    }

    /// This is function would be responsible for processing a batch of requests
    /// this would just send the requests to the chain and wait for it to be mined
    /// after this has been sent, this tx_hash would be used to wait for the transaction to be mined
    /// on the monitoring thread
    ///
    /// # Arguments
    /// - `self`
    /// - `requests` - The requests to be processed
    /// - `chain` - The chain to which the requests would be sent
    pub async fn process_batch_request(
        &self,
        request: Vec<ForwardRequestData>,
        refund_receiver: String,
    ) -> PendingTransactionBuilder<Http<Client>, Ethereum> {
        let trusted_forwarder_contract = self.get_trusted_forwarder();
        let request = request
            .iter()
            .map(|r| <&ForwardRequestData as Into<ERC2771Forwarder::ForwardRequestData>>::into(r))
            .collect();
        let req = trusted_forwarder_contract
            .executeBatch(request, Address::from_str(&refund_receiver).unwrap());
        req.send().await.unwrap()
    }

    /// This function would be responsible for waiting for the transaction to be mined
    /// this would be called by the monitoring thread
    ///
    /// # Arguments
    /// - `self`
    /// - `pending_tx` - The transaction hash to be monitored
    pub async fn wait_for_transaction(
        &self,
        pending_tx: PendingTransactionBuilder<Http<Client>, Ethereum>,
    ) -> FixedBytes<32> {
        // TODO: add configuration for the number of blocks to wait for
        let tx_hash = pending_tx.watch().await.unwrap();
        tx_hash
    }

    pub fn get_trusted_forwarder(
        &self,
    ) -> TrustedForwarderContractInstance<
        Http<Client>,
        FillProvider<
            JoinFill<
                JoinFill<
                    alloy::providers::Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<Http<Client>>,
            Http<Client>,
            Ethereum,
        >,
    > {
        let rand_private_key: PrivateKeySigner = self.chains_config.accounts_private_keys[0]
            .clone()
            .parse()
            .unwrap();
        let wallet = EthereumWallet::from(rand_private_key.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(self.chains_config.rpc_url.parse().unwrap());

        TrustedForwarderContract::new(self.trusted_forwarder, provider)
    }
}

/// impl from `ForwardRequestData` to `ERC2771Forwarder::ForwardRequestData`
impl From<ForwardRequestData> for ERC2771Forwarder::ForwardRequestData {
    fn from(data: ForwardRequestData) -> Self {
        Self {
            from: data.from,
            to: data.to,
            value: data.value,
            gas: data.gas,
            deadline: data.deadline,
            data: data.data,
            signature: data.signature,
        }
    }
}

/// impl from `&ForwardRequestData` to `ERC2771Forwarder::ForwardRequestData`
impl From<&ForwardRequestData> for ERC2771Forwarder::ForwardRequestData {
    fn from(data: &ForwardRequestData) -> Self {
        Self {
            from: data.from,
            to: data.to,
            value: data.value,
            gas: data.gas,
            deadline: data.deadline,
            data: data.data.clone(),
            signature: data.signature.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configs::ChainsConfig;
    use alloy::{
        network::{EthereumWallet, NetworkWallet}, node_bindings::Anvil, primitives::{b256, Address, B256},
        providers::ProviderBuilder, signers::{local::PrivateKeySigner, Signer}, sol_types::{eip712_domain, SolStruct},
    };
    use serde::Serialize;
    use std::str::FromStr;
    
    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockContract,
        "src/contract-artifacts/MockContract.json"
    );
    
    sol! {
        #[allow(missing_docs)]
        #[derive(Serialize)]
        struct FowardRequestWithNonce {
            address from;
            address to;
            uint256 value;
            uint256 gas;
            uint256 nonce;
            uint48 deadline;
            bytes data;
        }
    }


    #[tokio::test]
    async fn test_process_request() {
        let anvil = Anvil::new().block_time(1).try_spawn().unwrap();
        let http_rpc_url = anvil.endpoint();
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(http_rpc_url.parse().unwrap());
        
        
        let tf = TrustedForwarderContract::deploy(provider.clone(), "TF".to_string())
            .await
            .unwrap();
        let tf_instance = TrustedForwarderContract::new(*tf.address(), provider.clone());
        let mock = MockContract::deploy(provider.clone(), tf_instance.address().clone())
            .await
            .unwrap();

        let chains_config = ChainsConfig {
            name: Some("Ethereum Dev Network".to_string()),
            rpc_url: http_rpc_url,
            chain_id: anvil.chain_id(),
            accounts_private_keys: vec![signer.to_bytes().to_string()],
            trusted_forwarder: tf_instance.address().to_string(),
        };
        
        let processor = Processor::new(chains_config, *tf_instance.address());
        
        let domain = eip712_domain! {
            name: "TF",
            version: "1",
            chain_id: anvil.chain_id(),
            verifying_contract: tf_instance.address().clone(),
            salt: B256::ZERO,
        };
        
        let request = FowardRequestWithNonce {
            from: signer.address(),
            to: *mock.address(),
            value: U256::from(0),
            gas: U256::from(100000),
            nonce: U256::from(0),
            deadline: U48::MAX,
            data: mock.mockFunction().calldata().clone(),
        };
        
        let hash = request.eip712_signing_hash(&domain);
        let signature = signer.sign_hash(&hash).await.unwrap();
        
        let forward_request = ForwardRequestData {
            from: request.from,
            to: request.to,
            value: request.value,
            gas: request.gas,
            deadline: request.deadline,
            data: request.data,
            nonce: request.nonce,
            signature: signature.as_bytes().into(),
        };
        
        println!("{:?}", forward_request);
        
        let pending_tx = processor.process_request(forward_request).await;
    }
}
