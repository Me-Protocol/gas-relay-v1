//! This file would be responsible for all the blockchain related operations in the gasless-relayer server.
use crate::configs::ChainsConfig;
use alloy::primitives::Address;

/// This struct would be responsible for processing all the requests to be sent to the blockchain.
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
    ///
    /// # Arguments
    /// - `self`
    /// - `request` - The request to be processed
    /// - `chain` - The chain to which the request would be sent
    pub fn process_request(&self) {
        todo!()
    }

    /// This is function would be responsible for processing a batch of requests
    ///
    /// # Arguments
    /// - `self`
    /// - `requests` - The requests to be processed
    /// - `chain` - The chain to which the requests would be sent
    pub fn process_batch_request(&self) {
        todo!()
    }
}
