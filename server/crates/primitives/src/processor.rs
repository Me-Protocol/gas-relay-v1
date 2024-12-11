//! This file would be responsible for all the blockchain related operations in the gasless-relayer server.
use crate::configs::{ChainsConfig, PendingRequest};
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
use std::str::FromStr;
use TrustedForwarderContract::TrustedForwarderContractInstance;

/// This struct would be responsible for processing all the requests to be sent to the blockchain.
/// struct would also be resonsible for waiting for the transaction to be mined
#[derive(Debug, Clone)]
pub struct Processor {
    /// This are the cinfig for the chains this messages would be sent to
    pub chains_config: Vec<ChainsConfig>,
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
    pub fn new(chains_config: Vec<ChainsConfig>) -> Self {
        Self { chains_config }
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
        &mut self,
        request: ForwardRequestData,
        request_id: String,
        chain_index: usize,
    ) -> Result<PendingRequest, String> {
        let trusted_forwarder_contract = self.get_trusted_forwarder(chain_index);
        let req = trusted_forwarder_contract.execute(request.into());
        let pending_tx = req
            .send()
            .await
            .map_err(|e| format!("Processor Error: {:?}", e))?;

        Ok(PendingRequest {
            request_id,
            tx_pending: pending_tx,
        })
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
    /// - `request_id` - The id of the request
    pub async fn process_batch_request(
        &mut self,
        request: Vec<ForwardRequestData>,
        refund_receiver: String,
        chain_index: usize,
        request_id: String,
    ) -> Result<PendingRequest, String> {
        let trusted_forwarder_contract = self.get_trusted_forwarder(chain_index);
        let request = request
            .iter()
            .map(|r| <&ForwardRequestData as Into<ERC2771Forwarder::ForwardRequestData>>::into(r))
            .collect();
        let req = trusted_forwarder_contract
            .executeBatch(request, Address::from_str(&refund_receiver).unwrap());
        let pending_tx = req
            .send()
            .await
            .map_err(|e| format!("Processor Error: {:?}", e))?;

        Ok(PendingRequest {
            request_id,
            tx_pending: pending_tx,
        })
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
        let tx_hash = pending_tx.watch().await.unwrap();
        tx_hash
    }

    pub fn get_trusted_forwarder(
        &mut self,
        chain_index: usize,
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
        let rand_private_key: PrivateKeySigner = self.chains_config[chain_index]
            .accounts_private_keys
            .get_current_key()
            .parse()
            .unwrap();
        let wallet = EthereumWallet::from(rand_private_key.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(self.chains_config[chain_index].rpc_url.parse().unwrap());

        TrustedForwarderContract::new(
            Address::from_str(&self.chains_config[chain_index].trusted_forwarder)
                .expect("Invalid Address from config"),
            provider,
        )
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
    use crate::configs::{ChainsConfig, RelayerAccounts};
    use alloy::{
        network::EthereumWallet,
        node_bindings::Anvil,
        primitives::{b256, bytes},
        providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
    };

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockContract,
        "src/contract-artifacts/MockContract.json"
    );

    #[tokio::test]
    async fn test_process_request() {
        let anvil = Anvil::new().block_time(1).try_spawn().unwrap();
        let http_rpc_url = anvil.endpoint();
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let alice = b256!("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");
        let signer_alice = PrivateKeySigner::from_bytes(&alice).unwrap();
        let wallet = EthereumWallet::from(signer.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(http_rpc_url.parse().unwrap());

        let tf = TrustedForwarderContract::deploy(provider.clone(), "ERC2771Forwarder".to_string())
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
            accounts_private_keys: RelayerAccounts::new(vec![alice.to_string()]),
            trusted_forwarder: tf_instance.address().to_string(),
        };

        let mut processor = Processor::new(vec![chains_config]);

        let forward_request = ForwardRequestData {
            from: signer_alice.address(),
            to: *mock.address(),
            value: U256::from(0),
            gas: U256::from(100000),
            nonce: U256::from(0),
            deadline: U48::from(1733533191),
            data: mock.mockFunction().calldata().clone(),
            signature: bytes!("cf8b1890beb9783665ee231e7046e7fa64e2e87d450bc04bf45f4f2cbf87734b235af0c44d829aca7faf663476ff9e5c81f160d866cf72a1ffb14db796cd10b31b"),
        };

        let _pending_tx = processor
            .process_request(forward_request, "1".to_string(), 0)
            .await;
    }
}
