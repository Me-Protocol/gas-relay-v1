use crate::relay::RequestState;
use axum::http::StatusCode;
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Serialize)]
struct CreateRequestRow {
    request_id: String,
}

pub async fn inital_insert_request_status(
    pool: &PgPool,
    chain_id: u64,
    request_id: String,
    request_state: RequestState,
    transaction_hash: String,
) {
    let row = sqlx::query_as!(
        CreateTaskRow,
        "INSERT INTO tasks (chain_id, request_id, request_state, transaction_hash) VALUES ($1, $2, $3, $4) RETURNING request_id",
        chain_id,
        request_id,
        request_state.to_string(),
        transaction_hash,
      )
      .fetch_one(&pool)
      .await
      .map_err(|e| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          json!({"success": false, "message": e.to_string()}).to_string(),
        )
      })?;
}

pub async fn final_update_request_status(
    pool: &PgPool,
    request_id: String,
    request_state: RequestState,
    block_number: u64,
    mined_at: NaiveDateTime,
    gas_used: u64,
) {
}
