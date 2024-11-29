//! This module holds the handler for the raley server routes

use axum::{
    extract::{Path, Query, State},
    Json,
};
use primitives::relay::RequestStatus;
use std::sync::Arc;

use crate::{error::RelayServerError, AppState};

pub async fn get_request_status(
    State(state): State<Arc<AppState>>,
    Path(request_id): Path<String>,
) -> Result<Json<RequestStatus>, RelayServerError> {
    todo!()
    // Ok(Json(request_status))
}
