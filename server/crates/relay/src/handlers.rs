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
use std::sync::Arc;

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
    let request_id = generate_request_id();
    let request_status = inital_insert_request_status(
        &state.db_client,
        relay_request.chain_id,
        request_id.clone(),
        RequestState::Pending,
        false,
    )
    .await
    .unwrap();

    // shot a request to the process (this is where the chain interaction takes place)
    let pending_tx = state
        .processor
        .process_request(relay_request.into(), request_id, 0)
        .await;
    // broad cast a message via the chanel to monitor tx and update db... this would be async (would not be waiting for the response)
    state.mpsc_sender.send(pending_tx).await.unwrap();

    Ok(Json(request_status))
}

pub async fn batch_relay_request(
    State(state): State<Arc<AppState>>,
    Json(relay_requests): Json<Vec<RelayRequest>>,
) -> Result<Json<RequestStatus>, RelayServerError> {
    if relay_requests.is_empty() {
        return Err(RelayServerError::BadRequest(
            "Empty batch request".to_string(),
        ));
    }
    let request_id = generate_request_id();
    let request_status = inital_insert_request_status(
        &state.db_client,
        relay_requests[0].chain_id,
        request_id,
        RequestState::Pending,
        false,
    )
    .await
    .unwrap();

    // shot a request to the process (this is where the chain interaction takes place)
    // broad cast a message via the chanel to monitor tx and update db... this would be async (would not be waiting for the response)

    Ok(Json(request_status))
}
