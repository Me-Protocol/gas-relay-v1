//! This file discribes how multi-threadable tasks would be structured in the gasless-relayer server.
use async_trait::async_trait;
use futures::{future::try_join_all, Future};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub mod monitor;
pub mod relay;

/// Core task trait implemenated by top level gasless-relayer tasks.
#[async_trait] // to fix: the trait `Task` cannot be made into an object consider moving `run` to another trait
pub trait Task: Sync + Send + 'static {
    async fn run(self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()>;
}

/// This takes in a list of tasks and runs them concurrently and waiting for a shutdown signal.
pub async fn spawn_tasks<T, R, E>(
    relay_server: Box<dyn Task>,
    monitor_server: Box<dyn Task>,
    signal: T,
) where
    T: Future<Output = Result<R, E>> + Send + 'static,
    E: std::fmt::Debug,
{
    let (shutdown_scope, mut shutdown_wait) = mpsc::channel::<()>(1);
    let shutdown_token = CancellationToken::new();
    let mut shutdown_scope = Some(shutdown_scope);

    // Spawn the relay server
    let relay_handle: JoinHandle<anyhow::Result<()>> = {
        let st = shutdown_token.clone();
        let ss = shutdown_scope.clone();
        tokio::spawn(async move {
            let ret = relay_server.run(st).await;
            drop(ss); // Signal task completion
            ret
        })
    };

    // Spawn the monitor server
    let monitor_handle: JoinHandle<anyhow::Result<()>> = {
        let st = shutdown_token.clone();
        let ss = shutdown_scope.clone();
        tokio::spawn(async move {
            let ret = monitor_server.run(st).await;
            drop(ss); // Signal task completion
            ret
        })
    };

    // Combine the task handles for easier management
    let handles = vec![relay_handle, monitor_handle];

    // Running section on operational taskes and shutdown signal
    tokio::select! {
        res = try_join_all(handles) => {
            error!("Task exited unexpectedly: {res:?}");
        }
        res = signal => {
            match res {
                Ok(_) => {
                    info!("Received signal, shutting down");
                }
                Err(err) => {
                    error!("Error while waiting for signal: {err:?}");
                }
            }
        }
    }

    shutdown_token.cancel();
    shutdown_scope.take();
    shutdown_wait.recv().await;
}
