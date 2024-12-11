use std::str::FromStr;

use alloy::primitives::{aliases::U48, Address, Bytes, B256, U256};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::processor::ForwardRequestData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestState {
    Pending, // This means the request has been sent but not yet mined
    Success, // This means the request has been mined and successful
    Failed,  // This means the request has been mined and failed
    Timeout, // This request took too long to be mined
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayRequest {
    pub chain_id: u64,
    pub from: String,
    pub to: String,
    pub value: u64,
    pub gas: u64,
    pub deadline: u64,
    pub data: String,
    pub nonce: u64,
    pub signature: String,
    pub access_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestStatus {
    pub chain_id: u64,
    pub request_id: String,
    pub request_state: RequestState,
    pub created_at: NaiveDateTime,
    pub transaction_hash: String,
    pub block_number: u64,
    pub mined_at: NaiveDateTime,
    pub gas_used: u64,
    pub is_batch: bool,
}

// impl to convert a string to RequestState
impl From<String> for RequestState {
    fn from(s: String) -> Self {
        let s = s.as_str();
        match s {
            "Pending" => Self::Pending,
            "Success" => Self::Success,
            "Failed" => Self::Failed,
            "Timeout" => Self::Timeout,
            _ => panic!("Invalid request state"),
        }
    }
}

// impl to convert a RequestState to a string
impl From<RequestState> for String {
    fn from(s: RequestState) -> Self {
        match s {
            RequestState::Pending => "Pending".to_string(),
            RequestState::Success => "Success".to_string(),
            RequestState::Failed => "Failed".to_string(),
            RequestState::Timeout => "Timeout".to_string(),
        }
    }
}

pub fn generate_request_id() -> String {
    B256::random().to_string()
}

impl RelayRequest {
    pub fn into_data(&self) -> ForwardRequestData {
        ForwardRequestData {
            from: Address::from_str(&self.from).unwrap(),
            to: Address::from_str(&self.to).unwrap(),
            value: U256::from(self.value),
            gas: U256::from(self.gas),
            deadline: U48::from(self.deadline),
            data: Bytes::from_str(&self.data).unwrap(),
            nonce: U256::from(self.nonce),
            signature: Bytes::from_str(&self.signature).unwrap(),
        }
    }
}
