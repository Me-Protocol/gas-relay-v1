//! This file would be responsible for all the blockchain related operations in the gasless-relayer server.

use alloy::primitives::Address;

use crate::configs::Chain;
pub struct Processor {
    pub chain: Chain,
    pub trusted_forwarder: Address,
}

impl Processor {
    pub fn new(chain: Chain, trusted_forwarder: Address) -> Self {
        Self {
            chain,
            trusted_forwarder,
        }
    }

    pub fn process_request(&self) {
        todo!()
    }

    pub fn process_batch_request(&self) {
        todo!()
    }

    pub fn get_trusted_forwarder_contract(&self) {
        todo!()
    }
}
