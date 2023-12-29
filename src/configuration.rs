use crate::security::{Keyring, KeyringError};
use dirs::config_dir;
use log;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub const DEFAULT_APPLICATION_ID: &'static str = "uamcli";
pub const DEFAULT_PROJECT_ID: &'static str = "";
pub const DEFAULT_ENVIRONMENT_ID: &'static str = "";
pub const DEFAULT_CONFIGURATION_FILE_NAME: &'static str = "config.yml";
pub const DEFAULT_CLIENT_SECRET_KEY: &'static str = "client_secret";

#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    #[error("failed to resolve the configuration directory")]
    FailedToFindConfigurationDirectory,
    #[error("failed to load configuration data, because of: {cause:?}")]
    FailedToLoadData { cause: Box<dyn std::error::Error> },
    #[error("failed to write configuration data to file, because of: {cause:?}")]
    FailedToWriteData { cause: Box<dyn std::error::Error> },
    #[error("missing value for property \"{name:?}\"")]
    MissingRequiredPropertyValue { name: String },
    #[error("credentials not provided")]
    CredentialsNotProvided,
    #[error("security error {0}")]
    KeyringError(#[from] KeyringError),
    #[error("input/output error")]
    InputOutput(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    project_id: String,
    environment_id: String,
    client_id: Option<String>,
    #[serde(skip_serializing)]
    client_secret: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new(
            DEFAULT_ENVIRONMENT_ID.to_string(),
            DEFAULT_PROJECT_ID.to_string(),
            None,
            None,
        )
    }
}

impl Configuration {
    pub fn new(
        environment_id: String,
        project_id: String,
        client_id: Option<String>,
        client_secret: Option<String>,
    ) -> Configuration {
        Self {
            environment_id,
            project_id,
            client_id,
            client_secret,
        }
    }

    pub fn environment_id(&self) -> String {
        self.environment_id.to_owned()
    }

    pub fn set_environment_id(&mut self, environment_id: String) {
        self.environment_id = environment_id.to_owned();
    }

    pub fn project_id(&self) -> String {
        self.project_id.to_owned()
    }

    pub fn set_project_id(&mut self, project_id: String) {
        self.project_id = project_id;
    }

    pub fn client_id(&self) -> Option<String> {
        self.client_id.to_owned()
    }

    pub fn set_client_id(&mut self, client_id: Option<String>) {
        self.client_id = client_id;
    }

    pub fn set_client_secret(
        &mut self,
        client_secret: Option<String>,
    ) -> Result<(), ConfigurationError> {
        self.client_secret = client_secret.to_owned();
        Ok(())
    }

    pub fn client_secret(&self) -> Option<String> {
        self.client_secret.to_owned()
    }

    pub fn get_default_configuration_file_path() -> Result<PathBuf, ConfigurationError> {
        let configuration_directory = config_dir();
        match configuration_directory {
            Some(configuration_directory) => {
                let mut default_config_file_path = configuration_directory;
                default_config_file_path.push(DEFAULT_APPLICATION_ID);
                default_config_file_path.push(DEFAULT_CONFIGURATION_FILE_NAME);

                Ok(default_config_file_path)
            }
            None => Err(ConfigurationError::FailedToFindConfigurationDirectory),
        }
    }

    pub fn load_default() -> Result<Configuration, ConfigurationError> {
        let default_file_path = Configuration::get_default_configuration_file_path()?;
        log::debug!(
            "Loading configuration from {}...",
            default_file_path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap()
        );
        Configuration::load_from_file(default_file_path)
    }

    pub fn load_from_file(path: PathBuf) -> Result<Configuration, ConfigurationError> {
        // read the configuration from a file
        let result = match fs::read_to_string(path.clone()) {
            Ok(configuration) => match serde_yaml::from_str::<Configuration>(&configuration) {
                Ok(configuration) => Ok(configuration),
                Err(cause) => Err(ConfigurationError::FailedToLoadData {
                    cause: Box::new(cause),
                }),
            },
            Err(cause) => Err(ConfigurationError::FailedToLoadData {
                cause: Box::new(cause),
            }),
        };

        // read the client secret from the keystore
        match result {
            Ok(configuration) => {
                let mut configuration = configuration.clone();
                let keyring =
                    Keyring::new(DEFAULT_APPLICATION_ID, configuration.project_id.as_str());
                match keyring.get(DEFAULT_CLIENT_SECRET_KEY)? {
                    Some(secret) => {
                        configuration.client_secret = Some(secret);
                        Ok(configuration)
                    }
                    None => Err(ConfigurationError::CredentialsNotProvided),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn write(&self, writer: Box<dyn Write>) -> Result<(), ConfigurationError> {
        match serde_yaml::to_writer(writer, &self.clone()) {
            Ok(()) => Ok(()),
            Err(e) => Err(ConfigurationError::FailedToWriteData { cause: Box::new(e) }),
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), ConfigurationError> {
        // first check if the parent directory exists and try to create it if not
        let configuration_directory = path.parent();
        match configuration_directory {
            Some(path) => {
                // this operation only executes if the directory does not exit
                match fs::create_dir_all(path) {
                    Ok(()) => (),
                    Err(_) => return Err(ConfigurationError::FailedToFindConfigurationDirectory),
                }
            }
            None => return Err(ConfigurationError::FailedToFindConfigurationDirectory),
        }

        // write to file
        let file = File::create(&path);
        match file {
            Ok(file) => {
                let writer: Box<dyn Write> = Box::new(file);
                self.write(writer)?;
            }
            Err(e) => return Err(ConfigurationError::FailedToWriteData { cause: Box::new(e) }),
        };

        // write the secret to the keystore
        if let Some(secret) = &self.client_secret {
            let keyring = Keyring::new(DEFAULT_APPLICATION_ID, &self.project_id);
            keyring.put(DEFAULT_CLIENT_SECRET_KEY, secret.as_str())?;
        }

        Ok(())
    }

    pub fn save_to_default(&self) -> Result<(), ConfigurationError> {
        self.save(&Self::get_default_configuration_file_path()?)
    }

    pub fn delete(&self) -> Result<(), ConfigurationError> {
        fs::remove_file(&Self::get_default_configuration_file_path()?)?;

        let keyring = Keyring::new(DEFAULT_APPLICATION_ID, &self.project_id);
        keyring.delete(DEFAULT_CLIENT_SECRET_KEY)?;
        Ok(())
    }
}
