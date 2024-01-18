use pretty_env_logger;
use std::cell::RefCell;
use thiserror::Error;
use uamcli::{
    api::Api,
    cli::{Cli, CliError},
    configuration::Configuration,
};

/// A wrapper error encompassing all specific errors from the library functions.
///
/// This error type is designed to convert various error messages into a human-friendly format.
#[derive(Debug, Error)]
enum UamCliError {
    #[error("CLI error")]
    CliError(#[from] CliError), // wrap all errors from the CLI module
}

/// Main entry point for the program
#[tokio::main]
async fn main() -> Result<(), UamCliError> {
    // initialize the log
    let _log_init_result = pretty_env_logger::try_init_timed();

    // create new configuration object.
    // Initialize from the default configuration file location, or create a new configuration file if none exist.
    let configuration = RefCell::new(Configuration::load_default().unwrap_or_default());

    // create a new API object and initialize it with the configuration
    let api = Api::new(&configuration);

    // parse command line arguments and execute required command
    Cli::default().execute_command(api).await?;

    // to-do: catch the error, print human-friendly error message, return appropriate exit code

    Ok(())
}
