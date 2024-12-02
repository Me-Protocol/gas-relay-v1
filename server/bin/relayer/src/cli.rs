use tracing_subscriber::{filter::LevelFilter, util::SubscriberInitExt};

/// Main entry point for the CLI
///
/// Parses the CLI arguments and runs the appropriate subcommand.
/// Listens for a ctrl-c signal and shuts down all components when received.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    logger_setup()?;
    // let config: Config = return_me_indexer_config();

    // // create system db is one has not been created
    // let mut db_client = create_db_instance(&config.db_url)
    //     .await
    //     .expect("Could not create db instance");
    // create_system_table(&mut db_client).await?;

    // // server config
    // let server_config = config.clone().server;
    // //indexer config
    // let indexer_configs = config.clone().indexer;

    // tracing::info!("Starting Chronicle with config: {:?}", config.clone());

    // let mut tasks = vec![ServerTask::new(server_config, config.db_url.clone()).boxed()];

    // for indexer_config in indexer_configs {
    //     tasks.push(IndexerTask::new(indexer_config, config.db_url.clone()).boxed());
    // }

    // spawn_tasks(tasks, tokio::signal::ctrl_c()).await;

    Ok(())
}

// this function is for setting up the logging process
pub fn logger_setup() -> Result<(), anyhow::Error> {
    let filter =
        tracing_subscriber::EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .finish()
        .try_init()?;

    Ok(())
}
