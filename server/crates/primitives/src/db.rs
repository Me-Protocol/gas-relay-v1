use crate::relay::{RequestState, RequestStatus};
use chrono::NaiveDateTime;
use postgres::NoTls;

const DB_VERSION: &str = "2";

pub async fn create_db_instance(url: &String) -> Result<tokio_postgres::Client, anyhow::Error> {
    let (client, connection) = tokio_postgres::connect(url.as_str(), NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn create_request_status_table(
    client: &tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            CREATE TABLE IF NOT EXISTS request_status_{DB_VERSION} (
                id              SERIAL PRIMARY KEY,
                chain_id        BIGINT NOT NULL,
                request_id      VARCHAR NOT NULL UNIQUE,
                request_state   VARCHAR NOT NULL,
                created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                transaction_hash VARCHAR NOT NULL,
                block_number    BIGINT NOT NULL,
                mined_at        TIMESTAMP NOT NULL,
                gas_used        BIGINT NOT NULL,
                batch           BOOL DEFAULT FALSE NOT NULL
            )
        "
    );
    client.batch_execute(&executable).await?;

    Ok(())
}

pub async fn inital_insert_request_status(
    client: &tokio_postgres::Client,
    chain_id: u64,
    request_id: String,
    request_state: RequestState,
    is_batch: bool,
) -> Result<RequestStatus, anyhow::Error> {
    let query = format!("
        INSERT INTO request_status_{DB_VERSION} (
            chain_id, request_id, request_state, transaction_hash, block_number, mined_at, gas_used, batch
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (request_id) DO NOTHING
    ");

    // Assuming block_number, mined_at, and gas_used are default values for initial insert
    let block_number = 0_i64;
    let mined_at = chrono::Utc::now().naive_utc(); // Timestamp for mined_at
    let gas_used = 0_i64;
    let request_state: String = request_state.into();
    let transaction_hash = "".to_string();

    client
        .execute(
            &query,
            &[
                &(chain_id as i64),
                &request_id,
                &request_state,
                &transaction_hash,
                &block_number,
                &mined_at,
                &gas_used,
                &is_batch,
            ],
        )
        .await?;

    let request_status = RequestStatus {
        chain_id,
        request_id,
        request_state: request_state.into(),
        created_at: mined_at,
        transaction_hash,
        block_number: block_number as u64,
        mined_at,
        gas_used: gas_used as u64,
        is_batch,
    };

    Ok(request_status)
}

pub async fn final_update_request_status(
    client: &tokio_postgres::Client,
    request_id: String,
    request_state: RequestState,
    block_number: u64,
    mined_at: NaiveDateTime,
    gas_used: u64,
    transaction_hash: String,
) -> Result<(), anyhow::Error> {
    let query = format!(
        "
        UPDATE request_status_{DB_VERSION}
        SET request_state = $1, 
            block_number = $2, 
            mined_at = $3, 
            gas_used = $4, 
            transaction_hash = $5
        WHERE request_id = $6;
    "
    );
    let request_state: String = request_state.into();

    client
        .execute(
            &query,
            &[
                &request_state,
                &(block_number as i64),
                &mined_at,
                &(gas_used as i64),
                &transaction_hash,
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
    let query = format!(
        "
        SELECT 
            chain_id, 
            request_id, 
            request_state, 
            created_at, 
            transaction_hash, 
            block_number, 
            mined_at, 
            gas_used,
            batch
        FROM request_status_{DB_VERSION}
        WHERE request_id = $1
    "
    );

    let row = client.query_opt(&query, &[&request_id]).await?;

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
            is_batch: row.get(8),
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
    let query = format!(
        "
        SELECT 
            chain_id, 
            request_id, 
            request_state, 
            created_at, 
            transaction_hash, 
            block_number, 
            mined_at, 
            gas_used,
            batch
        FROM request_status_{DB_VERSION}
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    "
    );

    let offset = (page_number - 1) * page_size;

    let rows = client
        .query(&query, &[&(page_size), &(offset as i64)])
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
            is_batch: row.get(8),
        })
        .collect();

    Ok(request_statuses)
}
