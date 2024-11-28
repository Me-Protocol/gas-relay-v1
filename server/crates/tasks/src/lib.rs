//! This file discribes how multi-threadable tasks would be structured in the gasless-relayer server.
use futures::{future::try_join_all, Future};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use async_trait::async_trait;



pub mod monitor;
pub mod relay;



/// Core task trait implemenated by top level gasless-relayer tasks.
#[async_trait] // to fix: the trait `Task` cannot be made into an object consider moving `run` to another trait
pub trait Task: Sync + Send + 'static {
    async fn run(self: Box<Self>, shutdown_token: CancellationToken) -> anyhow::Result<()>;
}

/// This takes in a list of tasks and runs them concurrently and waiting for a shutdown signal.
pub async fn spawn_tasks<T, R, E>(tasks: impl IntoIterator<Item = Box<dyn Task>>, signal: T)
where
    T: Future<Output = Result<R, E>> + Send + 'static,
    E: std::fmt::Debug,
{
    let (shutdown_scope, mut shutdown_wait) = mpsc::channel::<()>(1);
    let shutdown_token = CancellationToken::new();
    let mut shutdown_scope = Some(shutdown_scope);

    let handles = tasks.into_iter().map(|task| {
        let st = shutdown_token.clone();
        let ss = shutdown_scope.clone();
        async move {
            let ret = task.run(st).await;
            drop(ss);
            ret
        }
    });

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