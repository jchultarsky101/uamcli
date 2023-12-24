use crate::{
    api::Api,
    configuration::{Configuration, ConfigurationError},
};
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;
use thiserror::Error;

pub struct Cli {}

pub const COMMAND_CONFIG: &str = "config";
pub const COMMAND_EXPORT: &str = "export";
pub const COMMAND_GET: &str = "get";
pub const COMMAND_PATH: &str = "path";
pub const COMMAND_SET: &str = "set";
pub const COMMAND_DELETE: &str = "delete";
pub const COMMAND_CLIENT: &str = "client";
pub const COMMAND_FOLDERS: &str = "folders";
pub const COMMAND_LOGIN: &str = "login";
pub const COMMAND_LOGOFF: &str = "logoff";
pub const COMMAND_FOLDER: &str = "folder";

pub const PARAMETER_OUTPUT: &str = "output";
pub const PARAMETER_API_URL: &str = "api_url";
pub const PARAMETER_OIDC_URL: &str = "oidc_url";
pub const PARAMETER_CLIENT_ID: &str = "client_id";
pub const PARAMETER_CLIENT_SECRET: &str = "client_secret";
pub const PARAMETER_PROJECT_ID: &str = "project";

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Configuration Error")]
    ConfigurationError(#[from] ConfigurationError),
    #[error("Generic")]
    Generic,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli {}
    }
}

impl Cli {
    fn prepare_commands(&self) -> ArgMatches {
        let output_file_parameter = Arg::new(PARAMETER_OUTPUT)
            .short('o')
            .long(PARAMETER_OUTPUT)
            .num_args(1)
            .required(true)
            .help("output file path")
            .value_parser(clap::value_parser!(PathBuf));

        let project_id_parameter = Arg::new(PARAMETER_PROJECT_ID)
            .short('p')
            .long(PARAMETER_PROJECT_ID)
            .num_args(1)
            .required(true)
            .help("tenant ID");

        let client_id_parameter = Arg::new(PARAMETER_CLIENT_ID)
            .long(PARAMETER_CLIENT_ID)
            .required(true)
            .help("Client ID for authentication");

        let client_secret_parameter = Arg::new(PARAMETER_CLIENT_SECRET)
            .long(PARAMETER_CLIENT_SECRET)
            .required(true)
            .help("Client secret for authentication");

        Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                // Configuration
                Command::new(COMMAND_CONFIG)
                    .about("working with configuration")
                    .subcommand_required(true)
                    .subcommand(
                        Command::new(COMMAND_GET)
                            .about("displays configuration")
                            .subcommand(
                                Command::new(COMMAND_PATH).about("show the configuration path"),
                            )
                            .subcommand(
                                Command::new(COMMAND_CLIENT).about("sets the client properties"),
                            ),
                    )
                    .subcommand(
                        Command::new(COMMAND_SET)
                            .about("sets configuration property")
                            .subcommand_required(true)
                            .subcommand(
                                Command::new(COMMAND_CLIENT)
                                    .about("Sets the clinet properties")
                                    .arg(project_id_parameter.clone())
                                    .arg(client_id_parameter)
                                    .arg(client_secret_parameter),
                            ),
                    )
                    .subcommand(
                        Command::new(COMMAND_EXPORT)
                            .about("export the current configuration in a file")
                            .arg(output_file_parameter),
                    )
                    .subcommand(
                        Command::new(COMMAND_DELETE).about("deletes the configuration file"),
                    ),
            )
            .get_matches()
    }

    /// Parses the command line arguments and executes matching commands.
    /// Returns Ok(()) if successfule or CliError otherwise.
    ///
    /// # Arguments
    ///
    /// * api - configured API object
    ///
    pub fn execute_command(&self, api: Api) -> Result<(), CliError> {
        match self.prepare_commands().subcommand() {
            // configuration commands and their parameters
            Some((COMMAND_CONFIG, sub_matches)) => match sub_matches.subcommand() {
                Some((COMMAND_GET, sub_matches)) => match sub_matches.subcommand() {
                    Some((COMMAND_PATH, _)) => {
                        let path = Configuration::get_default_configuration_file_path()?;
                        let path = path.into_os_string().into_string().unwrap();
                        println!("{}", path);
                    }
                    Some((COMMAND_CLIENT, _sub_matches)) => {
                        let configuration = api.configuration();
                        let configuration = configuration.clone();
                        let configuration = configuration.borrow();
                        let json = serde_json::to_string(&configuration.clone()).unwrap();
                        println!("{}", json);
                    }
                    _ => unreachable!("Invalid command"),
                },
                Some((COMMAND_SET, sub_matches)) => match sub_matches.subcommand() {
                    Some((COMMAND_CLIENT, sub_matches)) => {
                        let project_id =
                            sub_matches.get_one::<String>(PARAMETER_PROJECT_ID).unwrap(); // unwraps here are safe, because the arguments is mandatory and it will caught by Clap before this point
                        let client_id = sub_matches.get_one::<String>(PARAMETER_CLIENT_ID).unwrap();
                        let client_secret = sub_matches
                            .get_one::<String>(PARAMETER_CLIENT_SECRET)
                            .unwrap();

                        let configuration = api.configuration();
                        let mut configuration = configuration.borrow_mut();

                        configuration.set_project_id(project_id.to_owned());
                        configuration.set_client_id(Some(client_id.to_owned()));
                        configuration.set_client_secret(Some(client_secret.to_owned()))?;

                        configuration.save_to_default()?;
                    }
                    _ => unreachable!("Invalid command"),
                },
                Some((COMMAND_EXPORT, sub_matches)) => {
                    let path = sub_matches.get_one::<PathBuf>(PARAMETER_OUTPUT).unwrap(); // it is save vefause the argument is mandatory
                    api.configuration().borrow().save(path)?;
                }
                Some((COMMAND_DELETE, _)) => {
                    api.configuration().borrow().delete()?;
                }
                _ => unreachable!("Invalid subcommand for 'config set"),
            },

            // Login operations

            // Project operations
            _ => unreachable!("Invalid command"),
        }

        Ok(())
    }
}
