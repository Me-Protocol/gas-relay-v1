//! Monitor task

use crate::Task;
use anyhow::bail;
use async_trait::async_trait;
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug)]
pub struct MonitorTask {
    pub config: String,
}

impl MonitorTask {
    pub fn new(config: String) -> Self {
        Self { config }
    }

    /// Converts the task into a boxed trait object.
    pub fn boxed(self) -> Box<dyn Task> {
        Box::new(self)
    }
}

#[async_trait]
impl Task for MonitorTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        // let monitor_handle = tokio::spawn(async move {
        //     select! {
        //         _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
        //             info!("Monitor task completed");
        //         }
        //         _ = shutdown_token.cancelled() => {
        //             info!("Shutting down monitor");
        //         }
        //     }
        // });

        // match try_join!(monitor_handle) {
        //     Ok(_) => {
        //         info!("Monitor task completed");
        //     }
        //     Err(e) => bail!("Error running monitor: {:?}", e),
        // }
        Ok(())
    }
}
