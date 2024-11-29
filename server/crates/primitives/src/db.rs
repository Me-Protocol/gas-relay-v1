use crate::relay::{RequestState, RequestStatus};
use chrono::NaiveDateTime;

pub async fn create_request_status_table(
    client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            CREATE TABLE IF NOT EXISTS request_status (
                id              SERIAL PRIMARY KEY,
                chain_id        BIGINT NOT NULL,
                request_id      VARCHAR NOT NULL UNIQUE,
                request_state   VARCHAR NOT NULL,
                created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                transaction_hash VARCHAR NOT NULL,
                block_number    BIGINT NOT NULL,
                mined_at        TIMESTAMP NOT NULL,
                gas_used        BIGINT NOT NULL
            )
        "
    );
    client.batch_execute(&executable).await?;

    Ok(())
}

pub async fn inital_insert_request_status(
    client: &mut tokio_postgres::Client,
    chain_id: u64,
    request_id: String,
    request_state: RequestState,
    transaction_hash: String,
) -> Result<(), anyhow::Error> {
    let query = r#"
        INSERT INTO request_status (
            chain_id, request_id, request_state, transaction_hash, block_number, mined_at, gas_used
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (request_id) DO NOTHING
    "#;

    // Assuming block_number, mined_at, and gas_used are default values for initial insert
    let block_number = 0_i64;
    let mined_at = chrono::Utc::now(); // Timestamp for mined_at
    let gas_used = 0_i64;
    let request_state: String = request_state.into();

    client
        .execute(
            query,
            &[
                &(chain_id as i64),
                &request_id,
                &request_state,
                &transaction_hash,
                &block_number,
                &mined_at,
                &gas_used,
            ],
        )
        .await?;

    Ok(())
}

pub async fn final_update_request_status(
    client: &mut tokio_postgres::Client,
    request_id: String,
    request_state: RequestState,
    block_number: u64,
    mined_at: NaiveDateTime,
    gas_used: u64,
) -> Result<(), anyhow::Error> {
    let query = r#"
        UPDATE request_status
        SET request_state = $1, 
            block_number = $2, 
            mined_at = $3, 
            gas_used = $4
        WHERE request_id = $5
    "#;
    let request_state: String = request_state.into();

    client
        .execute(
            query,
            &[
                &request_state,
                &(block_number as i64),
                &mined_at,
                &(gas_used as i64),
                &request_id,
            ],
        )
        .await?;

    Ok(())
}

pub async fn query_request_status_by_request_id(
    client: &tokio_postgres::Client,
    request_id: String,
) -> Result<Option<RequestStatus>, anyhow::Error> {
    let query = r#"
        SELECT 
            chain_id, 
            request_id, 
            request_state, 
            created_at, 
            transaction_hash, 
            block_number, 
            mined_at, 
            gas_used
        FROM request_status
        WHERE request_id = $1
    "#;

    let row = client.query_opt(query, &[&request_id]).await?;

    if let Some(row) = row {
        let request_status = RequestStatus {
            chain_id: row.get::<_, i64>(0) as u64,
            request_id: row.get(1),
            request_state: row.get::<_, String>(2).into(), // Assuming RequestState implements FromStr
            created_at: row.get(3),
            transaction_hash: row.get(4),
            block_number: row.get::<_, i64>(5) as u64,
            mined_at: row.get(6),
            gas_used: row.get::<_, i64>(7) as u64,
        };
        Ok(Some(request_status))
    } else {
        Ok(None)
    }
}

pub async fn query_all_request_status_paginated(
    client: &tokio_postgres::Client,
    page_number: i64,
    page_size: i64,
) -> Result<Vec<RequestStatus>, anyhow::Error> {
    let query = r#"
        SELECT 
            chain_id, 
            request_id, 
            request_state, 
            created_at, 
            transaction_hash, 
            block_number, 
            mined_at, 
            gas_used
        FROM request_status
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    "#;

    let offset = (page_number - 1) * page_size;

    let rows = client
        .query(query, &[&(page_size), &(offset as i64)])
        .await?;

    let request_statuses = rows
        .into_iter()
        .map(|row| RequestStatus {
            chain_id: row.get::<_, i64>(0) as u64,
            request_id: row.get(1),
            request_state: row.get::<_, String>(2).into(), // Assuming RequestState implements FromStr
            created_at: row.get(3),
            transaction_hash: row.get(4),
            block_number: row.get::<_, i64>(5) as u64,
            mined_at: row.get(6),
            gas_used: row.get::<_, i64>(7) as u64,
        })
        .collect();

    Ok(request_statuses)
}
