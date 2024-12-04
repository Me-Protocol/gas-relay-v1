//! This file would be responsible for all the blockchain related operations in the gasless-relayer server.
use crate::configs::ChainsConfig;
use alloy::primitives::Address;

/// This struct would be responsible for processing all the requests to be sent to the blockchain.
/// struct would also be resonsible for waiting for the transaction to be mined
pub struct Processor {
    /// The chain to which the requests would be sent
    pub chains_config: ChainsConfig,
    /// This is the address of the trusted forwarder contract
    pub trusted_forwarder: Address,
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
    pub fn process_request(&self) {
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
    /// - `tx_hash` - The transaction hash to be monitored
    /// - `chain` - The chain to which the transaction was sent
    pub fn wait_for_transaction(&self) {
        todo!()
    }
}
