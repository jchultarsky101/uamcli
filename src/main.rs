use std::cell::RefCell;

use pretty_env_logger;
use thiserror::Error;
use uamcli::{
    api::Api,
    cli::{Cli, CliError},
    configuration::Configuration,
};

#[derive(Debug, Error)]
enum UamCliError {
    #[error("CLI error")]
    CliError(#[from] CliError),
}

/// Main entry point for the program
#[tokio::main]
async fn main() -> Result<(), UamCliError> {
    // initialize the log
    let _log_init_result = pretty_env_logger::try_init_timed();
    let configuration = RefCell::new(Configuration::load_default().unwrap_or_default());

    Cli::default().execute_command(Api::new(&configuration))?;

    Ok(())
}
