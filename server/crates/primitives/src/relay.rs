use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestState {
    Success,
    Failed,
    Timeout,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayRequest {
    from: String,
    to: String,
    value: u64,
    gas: u64,
    deadline: u64,
    data: String,
    nonce: u64,
    signature: String,
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
}
