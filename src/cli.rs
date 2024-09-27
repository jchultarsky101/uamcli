/// This module defines the available structure of command line commands and their arguments as well
/// as the method to parse and execute the command.
use crate::{
    api::Api,
    configuration::{Configuration, ConfigurationError},
    model::{AssetIdentity, AssetStatus},
};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use thiserror::Error;

pub struct Cli {}

const COMMAND_CONFIG: &str = "config";
const COMMAND_EXPORT: &str = "export";
const COMMAND_GET: &str = "get";
const COMMAND_PATH: &str = "path";
const COMMAND_SET: &str = "set";
const COMMAND_CREATE: &str = "create";
const COMMAND_DELETE: &str = "delete";
const COMMAND_CLIENT: &str = "client";
//const COMMAND_LOGIN: &str = "login";
//const COMMAND_LOGOFF: &str = "logoff";
const COMMAND_ASSET: &str = "asset";
const COMMAND_SEARCH: &str = "search";
const COMMAND_DOWNLOAD: &str = "download";
const COMMAND_UPLOAD: &str = "upload";
const COMMAND_STATUS: &str = "status";
const COMMAND_METADATA: &str = "metadata";
const COMMAND_GENERATE_THUMBNAIL: &str = "generate-thumbnail";

const PARAMETER_OUTPUT: &str = "output";
const PARAMETER_DOWNLOAD_DIR: &str = "download-dir";
const PARAMETER_CLIENT_ID: &str = "client-id";
const PARAMETER_CLIENT_SECRET: &str = "client-secret";
const PARAMETER_ORGANIZATION: &str = "organization";
const PARAMETER_PROJECT_ID: &str = "project";
const PARAMETER_ENVIRONMENT_ID: &str = "environment";
const PARAMETER_NAME: &str = "name";
const PARAMETER_DESCRIPTION: &str = "description";
const PARAMETER_ASSET_ID: &str = "asset-id";
const PARAMETER_ASSET_VERSION: &str = "asset-version";
const PARAMETER_DATA_FILE: &str = "data";
const PARAMETER_STATUS: &str = "status";
const PARAMETER_PUBLISH: &str = "publish";
const PARAMETER_ASSET_NAME: &str = "asset-name";
const PARAMETER_METADATA_KEY: &str = "meta";

const BANNER: &'static str = r#"
╦ ╦╔═╗╔╦╗  ╔═╗╦  ╦
║ ║╠═╣║║║  ║  ║  ║
╚═╝╩ ╩╩ ╩  ╚═╝╩═╝╩

"#;

/// Wrapper error for all errors that may be produced by
/// modules at lower level
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Configuration Error")]
    ConfigurationError(#[from] ConfigurationError),
    #[error("API error")]
    ApiError(#[from] crate::api::ApiError),
    #[error("Asset staus parse error")]
    StatusParseError(#[from] crate::model::AssetStatusParseError),
}

impl Default for Cli {
    fn default() -> Cli {
        Cli {}
    }
}

/// Command Line Interface abstraction.
///
/// Provides method to declare and execute CLI commands.
impl Cli {
    /// Declares the structure of all available CLI commands.
    ///
    /// Returns clap::ArgMatches object to be used for command execution.
    fn prepare_commands(&self) -> ArgMatches {
        let output_file_parameter = Arg::new(PARAMETER_OUTPUT)
            .short('o')
            .long(PARAMETER_OUTPUT)
            .num_args(1)
            .required(true)
            .help("Output file path")
            .value_parser(clap::value_parser!(PathBuf));
        let organization_id_parameter = Arg::new(PARAMETER_ORGANIZATION)
            .short('o')
            .long(PARAMETER_ORGANIZATION)
            .num_args(1)
            .required(true)
            .help("Organization ID");
        let project_id_parameter = Arg::new(PARAMETER_PROJECT_ID)
            .short('p')
            .long(PARAMETER_PROJECT_ID)
            .num_args(1)
            .required(true)
            .help("Tenant ID");
        let client_id_parameter = Arg::new(PARAMETER_CLIENT_ID)
            .long(PARAMETER_CLIENT_ID)
            .required(true)
            .help("Client ID for authentication");
        let client_secret_parameter = Arg::new(PARAMETER_CLIENT_SECRET)
            .long(PARAMETER_CLIENT_SECRET)
            .required(true)
            .help("Client secret for authentication");
        let environment_id_parameter = Arg::new(PARAMETER_ENVIRONMENT_ID)
            .long(PARAMETER_ENVIRONMENT_ID)
            .required(true)
            .help("Unity environment ID");
        let asset_id_parameter = Arg::new(PARAMETER_ASSET_ID)
            .long(PARAMETER_ASSET_ID)
            .required(true)
            .action(ArgAction::Set)
            .help("Asset ID");
        let asset_version_parameter = Arg::new(PARAMETER_ASSET_VERSION)
            .long(PARAMETER_ASSET_VERSION)
            .required(false)
            .default_value("1")
            .help("Asset version");
        let metadata_key_parameter = Arg::new(PARAMETER_METADATA_KEY)
            .long(PARAMETER_METADATA_KEY)
            .required(true)
            .num_args(1..)
            .value_delimiter(',')
            .action(clap::ArgAction::Append)
            .help("Metadata property name");

        Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .before_long_help(BANNER)
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                // Configuration
                Command::new(COMMAND_CONFIG)
                    .about("Working with configuration")
                    .subcommand_required(true)
                    .subcommand(
                        Command::new(COMMAND_CLIENT)
                            .about("Client configuration")
                            .subcommand_required(true)
                            .subcommand(
                                Command::new(COMMAND_SET)
                                    .about("Sets new client configuration")
                                    .arg(organization_id_parameter)
                                    .arg(project_id_parameter)
                                    .arg(environment_id_parameter)
                                    .arg(client_id_parameter)
                                    .arg(client_secret_parameter),
                            )
                            .subcommand(
                                Command::new(COMMAND_GET)
                                    .about("Prints the current client configuration")        
                            )
                    )
                    .subcommand(
                        Command::new(COMMAND_PATH)
                            .about("Configuration path")
                            .subcommand_required(true)
                            .subcommand(
                                Command::new(COMMAND_GET)
                                    .about("Prints the default configuration file path")
                            )
                    )
                    .subcommand(
                        Command::new(COMMAND_EXPORT)
                            .about("Exports the current configuration in a file")
                            .arg(output_file_parameter),
                    )
                    .subcommand(
                        Command::new(COMMAND_DELETE).about("Deletes the configuration file"),
                    ),
            )
            .subcommand(
                Command::new(COMMAND_ASSET)
                    .about("Digital asset operations")
                    .subcommand_required(true)
                    .subcommand(
                        Command::new(COMMAND_SEARCH)
                            .about("Searches for assets in the project")
                            .arg(Arg::new(PARAMETER_ASSET_ID)
                                .help("Optional: The asset ID")
                                .long(PARAMETER_ASSET_ID)
                                .required(false)
                                .action(ArgAction::Set)
                            )
                            .arg(Arg::new(PARAMETER_ASSET_VERSION)
                                .help("Asset version. Required if an asset ID is provided")
                                .long(PARAMETER_ASSET_VERSION)
                                .requires(PARAMETER_ASSET_ID)
                                .required(false)
                                .default_value("1")
                                .action(ArgAction::Set)
                                
                            )
                            .arg(Arg::new(PARAMETER_ASSET_NAME)
                                .help("Optional: The name of the asset")
                                .long(PARAMETER_ASSET_NAME)
                                .required(false)
                                .action(ArgAction::Set)
                            ),
                    )
                    .subcommand(
                        Command::new(COMMAND_GET)
                            .about("Retrieves an asset")
                            .arg(asset_id_parameter.clone())
                            .arg(asset_version_parameter.clone()),
                    )
                    .subcommand(
                        Command::new(COMMAND_CREATE)
                            .about("Creates new asset in the project")
                            .arg(
                                Arg::new(PARAMETER_NAME)
                                    .long(PARAMETER_NAME)
                                    .required(true)
                                    .help("asset name"),
                            )
                            .arg(
                                Arg::new(PARAMETER_DESCRIPTION)
                                    .long(PARAMETER_DESCRIPTION)
                                    .required(false)
                                    .help("Asset description"),
                            )
                            .arg(
                                Arg::new(PARAMETER_DATA_FILE)
                                    .long(PARAMETER_DATA_FILE)
                                    .required(true)
                                    .action(clap::ArgAction::Append)
                                    .help("File containing the 3D model data")
                                    .value_parser(clap::value_parser!(PathBuf)),
                            )
                            .arg(
                                Arg::new(PARAMETER_PROJECT_ID)
                                    .long(PARAMETER_PUBLISH)
                                    .required(false)
                                    .action(ArgAction::SetTrue)
                                    .help("If present, the asset will be automatically published after creation")
                            ),
                    )
                    .subcommand(
                        Command::new(COMMAND_DOWNLOAD)
                            .about("Download all asset files")
                            .arg(asset_id_parameter.clone())
                            .arg(asset_version_parameter.clone())
                            .arg(
                                Arg::new(PARAMETER_DOWNLOAD_DIR)
                                    .long(PARAMETER_DOWNLOAD_DIR)
                                    .required(false)
                                    .help("Download directory path")
                                    .value_parser(clap::value_parser!(PathBuf)),
                            ),
                    )
                    .subcommand(
                        Command::new(COMMAND_DELETE)
                            .about("Delete asset(s) from the project")
                            .arg(
                                Arg::new(PARAMETER_ASSET_ID)
                                    .long(PARAMETER_ASSET_ID)
                                    .num_args(1..)
                                    .value_delimiter(',')
                                    .action(clap::ArgAction::Append)
                                    .help("Asset UUID")
                            )
                    )
                    .subcommand(
                        Command::new(COMMAND_STATUS)
                            .about("Status operations on an asset")
                            .subcommand(
                                Command::new(COMMAND_SET)
                                    .arg(asset_id_parameter.clone())
                                    .arg(asset_version_parameter.clone())
                                    .arg(
                                        Arg::new(PARAMETER_STATUS)
                                            .long(PARAMETER_STATUS)
                                            .required(true)
                                            .help("Asset status value (e.g. draft, inreview, approved, published, rejected, withdrawn)")
                                    ),
                            ),
                    )
                    .subcommand(
                        Command::new(COMMAND_METADATA)
                            .about("Metadata operations")
                            .subcommand(
                                Command::new(COMMAND_UPLOAD)
                                    .about("Uploads metadata for an asset")
                                    .arg(asset_id_parameter.clone())
                                    .arg(asset_version_parameter.clone())
                                    .arg(
                                        Arg::new(PARAMETER_DATA_FILE)
                                            .long(PARAMETER_DATA_FILE)
                                            .required(true)
                                            .action(clap::ArgAction::Append)
                                            .help("File containing the metadata in CSV format with two columns: NAME, VALUE")
                                            .value_parser(clap::value_parser!(PathBuf)),
                                    ),
                            )
                            .subcommand(
                                Command::new(COMMAND_DELETE)
                                    .about("Deletes metadata associated with an asset")
                                    .arg(asset_id_parameter.clone())
                                    .arg(asset_version_parameter.clone())
                                    .arg(metadata_key_parameter.clone()),
                            )
                    )
                    .subcommand(
                        Command::new(COMMAND_GENERATE_THUMBNAIL)
                            .about("Generates thumbnails for asset(s)")
                            .arg(Arg::new(PARAMETER_ASSET_ID)
                                .help("Optional: The asset ID. If not provided, it will attempt to generate thumbnails for assets in the project that do not have one already")
                                .long(PARAMETER_ASSET_ID)
                                .required(false)
                                .action(ArgAction::Set)
                            )
                            .arg(Arg::new(PARAMETER_ASSET_VERSION)
                                .help("Asset version. Required if an asset ID is provided")
                                .long(PARAMETER_ASSET_VERSION)
                                .requires(PARAMETER_ASSET_VERSION)
                                .required(false)
                                .default_value("1")
                                .action(ArgAction::Set)
                                
                            ),
                    )
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
    pub async fn execute_command(&self, mut api: Api) -> Result<(), CliError> {
        match self.prepare_commands().subcommand() {
            // configuration commands and their parameters
            Some((COMMAND_CONFIG, sub_matches)) => match sub_matches.subcommand() {
                Some((COMMAND_PATH, sub_matches)) => match sub_matches.subcommand() {
                    Some((COMMAND_GET, _)) => {
                        let path = Configuration::get_default_configuration_file_path()?;
                        let path = path.into_os_string().into_string().unwrap();
                        println!("{}", path);
                    }
                    _ => unreachable!("Invalid command"),
                },
                Some((COMMAND_CLIENT, sub_matches)) => match sub_matches.subcommand() {
                    Some((COMMAND_SET, sub_matches)) => {
                        let organization_id = sub_matches
                            .get_one::<String>(PARAMETER_ORGANIZATION)
                            .unwrap();
                        let project_id =
                            sub_matches.get_one::<String>(PARAMETER_PROJECT_ID).unwrap(); // unwraps here are safe, because the arguments is mandatory and it will caught by Clap before this point
                        let environment_id = sub_matches
                            .get_one::<String>(PARAMETER_ENVIRONMENT_ID)
                            .unwrap();
                        let client_id = sub_matches.get_one::<String>(PARAMETER_CLIENT_ID).unwrap();
                        let client_secret = sub_matches
                            .get_one::<String>(PARAMETER_CLIENT_SECRET)
                            .unwrap();

                        let configuration = api.configuration();
                        let mut configuration = configuration.borrow_mut();

                        configuration.set_organization_id(organization_id.to_owned());
                        configuration.set_project_id(project_id.to_owned());
                        configuration.set_environment_id(environment_id.to_owned());
                        configuration.set_client_id(Some(client_id.to_owned()));
                        configuration.set_client_secret(Some(client_secret.to_owned()))?;

                        configuration.save_to_default()?;
                    }
                    Some((COMMAND_GET, _)) => {
                        let configuration = api.configuration();
                        let configuration = configuration.clone();
                        let configuration = configuration.borrow();
                        let json = serde_json::to_string(&configuration.clone()).unwrap();
                        println!("{}", json);
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
            Some((COMMAND_ASSET, sub_matches)) => match sub_matches.subcommand() {
                // Asset commands
                Some((COMMAND_SEARCH, sub_matches)) => {
                    let asset_name: Option<&String> = sub_matches.get_one::<String>(PARAMETER_ASSET_NAME);
                    let asset_id: Option<&String> = sub_matches.get_one::<String>(PARAMETER_ASSET_ID);
                    let asset_version: Option<&String> = sub_matches.get_one::<String>(PARAMETER_ASSET_VERSION);

                    let asset_id = match (asset_id, asset_version) {
                        (Some(asset_id), Some(asset_version)) => {
                            Some(AssetIdentity::new(asset_id.to_owned(), asset_version.to_owned()))
                        }
                        _ => None,
                    };

                    let assets = api.search_asset(asset_id, asset_name.map(|n| n.clone())).await?;
                    let json = serde_json::to_string(&assets).unwrap();
                    println!("{}", json);
                }
                Some((COMMAND_CREATE, sub_matches)) => {
                    let name = sub_matches.get_one::<String>(PARAMETER_NAME).unwrap();
                    let description = sub_matches.get_one::<String>(PARAMETER_DESCRIPTION);
                    let data_file_paths = sub_matches
                        .get_many::<PathBuf>(PARAMETER_DATA_FILE)
                        .unwrap();
                    let data_file_paths: Vec<&PathBuf> =
                        data_file_paths.into_iter().map(|p| p.into()).collect();
                    let publish = sub_matches.get_flag(PARAMETER_PROJECT_ID);

                    let result = api
                        .create_asset(
                            name.to_owned(),
                            description.to_owned().map(|s| s.to_owned()),
                            data_file_paths,
                            publish,
                        )
                        .await?;
                    let json = serde_json::to_string(&result).unwrap();
                    println!("{}", json);
                }
                Some((COMMAND_DELETE, sub_matches)) => {
                    let asset_ids: Vec<String> = sub_matches.get_many::<String>(PARAMETER_ASSET_ID).unwrap().cloned().collect();
                    
                    api.delete_asset(asset_ids).await?;
                }
                Some((COMMAND_GET, sub_matches)) => {
                    let id = sub_matches.get_one::<String>(PARAMETER_ASSET_ID).unwrap();
                    let version = sub_matches
                        .get_one::<String>(PARAMETER_ASSET_VERSION)
                        .unwrap();
                    let identity = AssetIdentity::new(id.to_owned(), version.to_owned());

                    let result = api.get_asset(&identity).await?;
                    let json = serde_json::to_string(&result).unwrap();
                    println!("{}", json);
                }
                Some((COMMAND_DOWNLOAD, sub_matches)) => {
                    let id = sub_matches.get_one::<String>(PARAMETER_ASSET_ID).unwrap();
                    let version = sub_matches
                        .get_one::<String>(PARAMETER_ASSET_VERSION)
                        .unwrap();
                    let identity = AssetIdentity::new(id.to_owned(), version.to_owned());
                    let output_directory = sub_matches.get_one::<PathBuf>(PARAMETER_DOWNLOAD_DIR);

                    let _ = api.download_asset(&identity, output_directory).await?;
                }
                Some((COMMAND_STATUS, sub_matches)) => match sub_matches.subcommand() {
                    Some((COMMAND_SET, sub_matches)) => {
                        let id = sub_matches.get_one::<String>(PARAMETER_ASSET_ID).unwrap();
                        let version = sub_matches
                            .get_one::<String>(PARAMETER_ASSET_VERSION)
                            .unwrap();
                        let identity = AssetIdentity::new(id.to_owned(), version.to_owned());
                        let status = sub_matches.get_one::<String>(PARAMETER_STATUS).unwrap();
                        let status: AssetStatus = status.as_str().parse()?;

                        let _ = api.set_asset_status(&identity, &status).await?;
                    }
                    _ => unreachable!("Invalid subcommand for 'asset status"),
                },
                Some((COMMAND_METADATA, sub_matches)) => match sub_matches.subcommand() {
                    Some((COMMAND_UPLOAD, sub_matches)) => {
                        let id = sub_matches.get_one::<String>(PARAMETER_ASSET_ID).unwrap();
                        let version = sub_matches
                            .get_one::<String>(PARAMETER_ASSET_VERSION)
                            .unwrap();
                        let identity = AssetIdentity::new(id.to_owned(), version.to_owned());

                        let data_file_path =
                            sub_matches.get_one::<PathBuf>(PARAMETER_DATA_FILE).unwrap();

                        let _ = api.upload_asset_metadata(&identity, data_file_path).await?;
                    }
                    Some((COMMAND_DELETE, sub_matches)) => {
                        let id = sub_matches.get_one::<String>(PARAMETER_ASSET_ID).unwrap();
                        let version = sub_matches
                            .get_one::<String>(PARAMETER_ASSET_VERSION)
                            .unwrap();
                        let identity = AssetIdentity::new(id.to_owned(), version.to_owned());
                        let keys = sub_matches.get_many::<String>(PARAMETER_METADATA_KEY).map(|iter| iter.cloned().collect::<Vec<String>>()).unwrap();


                        let _ = api.delete_asset_metadata(&identity, &keys).await?;
                    }
                    _ => unreachable!("Invalid subsommand for 'asset metadata'"), // this will never be reached because the command is validated first
                },
                Some((COMMAND_GENERATE_THUMBNAIL, sub_matches)) => {
                    let asset_id: Option<&String> = sub_matches.get_one::<String>(PARAMETER_ASSET_ID);
                    let asset_version: Option<&String> = sub_matches.get_one::<String>(PARAMETER_ASSET_VERSION);

                    let asset_id = match (asset_id, asset_version) {
                        (Some(asset_id), Some(asset_version)) => {
                            Some(AssetIdentity::new(asset_id.to_owned(), asset_version.to_owned()))
                        }
                        _ => None,
                    };

                    let assets = api.generate_asset_thumbnail(asset_id).await?;
                    let json = serde_json::to_string(&assets).unwrap();
                    println!("{}", json);
                }
                _ => unreachable!("Invalid subsommand for 'asset'"),
            },
            _ => unreachable!("Invalid command"),
        }

        Ok(())
    }
}
