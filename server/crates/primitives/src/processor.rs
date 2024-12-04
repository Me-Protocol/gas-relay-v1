//! This file would be responsible for all the blockchain related operations in the gasless-relayer server.
use crate::configs::ChainsConfig;
use alloy::{
    network::Ethereum,
    primitives::{aliases::U48, Address, Bytes, FixedBytes, U256},
    providers::{PendingTransactionBuilder, RootProvider},
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

pub struct ForwardRequestData {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub gas: U256,
    pub deadline: U48,
    pub data: Bytes,
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
    pub async fn process_request(&self, request: ForwardRequestData) {
        let trusted_forwarder_contract = self.get_trusted_forwarder();
        let req = trusted_forwarder_contract.execute(request.into());
        let req_stage_two = req.send().await.unwrap();
        todo!()
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
    pub fn process_batch_request(&self) {
        todo!()
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
    ) -> TrustedForwarderContractInstance<Http<Client>, RootProvider<Http<Client>>> {
        TrustedForwarderContract::new(self.trusted_forwarder, self.chains_config.chain_provider())
    }
}

// impl from `ForwardRequestData` to `ERC2771Forwarder::ForwardRequestData`
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
