//! This module holds the handler for the raley server routes
use crate::{error::RelayServerError, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use primitives::{
    db::{inital_insert_request_status, query_request_status_by_request_id},
    relay::{generate_request_id, RelayRequest, RequestState, RequestStatus},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchRelayRequestParams {
    pub refund_receiver: String,
    pub requests: Vec<RelayRequest>,
}

pub async fn get_request_status(
    State(state): State<Arc<AppState>>,
    Path(request_id): Path<String>,
) -> Result<Json<Option<RequestStatus>>, RelayServerError> {
    let request_status = query_request_status_by_request_id(&state.db_client, request_id)
        .await
        .unwrap();

    Ok(Json(request_status))
}

pub async fn relay_request(
    State(state): State<Arc<AppState>>,
    Json(relay_request): Json<RelayRequest>,
) -> Result<Json<RequestStatus>, RelayServerError> {
    // Validate access key
    if relay_request.access_key != state.access_key {
        return Err(RelayServerError::BadRequest("Invalid access key".to_string()));
    }

    // Generate request ID
    let request_id = generate_request_id();

    // Attempt to insert initial request status
    let request_status = inital_insert_request_status(
        &state.db_client,
        relay_request.chain_id,
        request_id.clone(),
        RequestState::Pending,
        false,
    )
    .await
    .map_err(|e| RelayServerError::DatabaseError(format!("Failed to insert initial request status: {:?}", e)))?;

    // Attempt to process the request
    let pending_tx = state
        .processor
        .lock()
        .await
        .process_request(relay_request.into_data(), request_id.clone(), 0)
        .await;

    // Attempt to send the pending transaction over the channel
    state
        .mpsc_sender
        .send(pending_tx)
        .await
        .map_err(|e| RelayServerError::ChannelError(format!("Failed to send transaction: {:?}", e)))?;

    Ok(Json(request_status))
}

pub async fn batch_relay_request(
    State(state): State<Arc<AppState>>,
    Json(relay_requests): Json<BatchRelayRequestParams>,
) -> Result<Json<RequestStatus>, RelayServerError> {
    // Validate that the batch request is not empty
    if relay_requests.requests.is_empty() {
        return Err(RelayServerError::BadRequest(
            "Empty batch request".to_string(),
        ));
    }

    // Generate request ID
    let request_id = generate_request_id();

    // Attempt to insert initial batch request status
    let request_status = inital_insert_request_status(
        &state.db_client,
        relay_requests.requests[0].chain_id,
        request_id.clone(),
        RequestState::Pending,
        true,
    )
    .await
    .map_err(|e| RelayServerError::DatabaseError(format!("Failed to insert initial batch request status: {:?}", e)))?;

    // Collect requests into appropriate format
    let requests: Vec<_> = relay_requests
        .requests
        .iter()
        .map(|r| r.into_data())
        .collect();

    // Attempt to process the batch request
    let pending_tx = state
        .processor
        .lock()
        .await
        .process_batch_request(
            requests,
            relay_requests.refund_receiver.clone(),
            0,
            request_id.clone(),
        )
        .await;

    // Attempt to send the pending transaction over the channel
    state
        .mpsc_sender
        .send(pending_tx)
        .await
        .map_err(|e| RelayServerError::ChannelError(format!("Failed to send batch transaction: {:?}", e)))?;

    Ok(Json(request_status))
}
