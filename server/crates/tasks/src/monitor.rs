//! Monitor task.
use crate::Task;
use anyhow::bail;
use async_trait::async_trait;
use monitor::run_monitor_task;
use primitives::{configs::PendingRequest, processor::Processor};
use tokio::{select, sync::mpsc::Receiver, try_join};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug)]
pub struct MonitorTask {
    pub db_url: String,
    pub mpsc_recv: Receiver<PendingRequest>,
}

impl MonitorTask {
    pub fn new(db_url: String, mpsc_recv: Receiver<PendingRequest>) -> Self {
        Self { db_url, mpsc_recv }
    }

    /// Converts the task into a boxed trait object.
    pub fn boxed(self) -> Box<dyn Task> {
        Box::new(self)
    }
}

#[async_trait]
impl Task for MonitorTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        let monitor_handle = tokio::spawn(async move {
            select! {
                monitor = run_monitor_task(
                    self.db_url,
                    self.mpsc_recv,
                ) => {
                    if monitor.is_err() {
                        info!("Monitor failed to start");
                    }
                    info!("Monitor task completed");
                }
                _ = shutdown_token.cancelled() => {
                    info!("Shutting down monitor");
                }
            }
        });

        match try_join!(monitor_handle) {
            Ok(_) => {
                info!("Monitor task completed");
            }
            Err(e) => bail!("Error running monitor: {:?}", e),
        }

        Ok(())
    }
}
