use primitives::{
    configs::PendingRequest,
    db::{create_db_instance, final_update_request_status},
    relay::RequestState,
};
use tokio::sync::mpsc::Receiver;

pub async fn run_monitor_task(
    db_url: String,
    mut mpsc_recv: Receiver<PendingRequest>,
) -> Result<(), anyhow::Error> {
    let client = create_db_instance(&db_url).await?;
    // on pending tx, watch for the tx to be mined, upate the db
    while let Some(pending_tx) = mpsc_recv.recv().await {
        process_monitoring(pending_tx, &client).await;
    }

    Ok(())
}

pub async fn process_monitoring(ptx: PendingRequest, client: &tokio_postgres::Client) {
    let tx_receipt_result = ptx.tx_pending.get_receipt().await;

    match tx_receipt_result {
        Ok(tx_receipt) => {
            final_update_request_status(
                client,
                ptx.request_id,
                RequestState::Success,
                tx_receipt.block_number.unwrap_or_default(),
                chrono::Utc::now().naive_utc(),
                tx_receipt.gas_used as u64,
                tx_receipt.transaction_hash.to_string(),
            )
            .await
            .unwrap();
        }
        Err(e) => {
            final_update_request_status(
                client,
                ptx.request_id,
                RequestState::Failed,
                0,
                chrono::Utc::now().naive_utc(),
                0 as u64,
                e.to_string(),
            )
            .await
            .unwrap();
        }
    }
}
