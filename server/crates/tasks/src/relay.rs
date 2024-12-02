use crate::Task;
use anyhow::bail;
use async_trait::async_trait;
use primitives::configs::ServerConfig;
use relay::run_relayer_server;
use tokio::{select, try_join};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug)]
pub struct ServerTask {
    pub config: ServerConfig,
}

impl ServerTask {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    /// Converts the task into a boxed trait object.
    pub fn boxed(self) -> Box<dyn Task> {
        Box::new(self)
    }
}

#[async_trait]
impl Task for ServerTask {
    async fn run(mut self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()> {
        let server_handle = tokio::spawn(async move {
            select! {
                server = run_relayer_server(self.config) => {
                    if server.is_err() {
                        info!("Relay server failed to start");
                    }
                }
                _ = shutdown_token.cancelled() => {
                    info!("Shutting down relay server");
                }
            }
        });

        match try_join!(server_handle) {
            Ok(_) => {
                info!("Server task completed");
            }
            Err(e) => bail!("Error running server: {:?}", e),
        }

        Ok(())
    }
}
