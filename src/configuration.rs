use dirs::config_dir;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use thiserror::Error;

pub const DEFAULT_APPLICATION_ID: &'static str = "uamcli";
pub const DEFAULT_CONFIGURATION_FILE_NAME: &'static str = "config.yml";

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
    #[error("unknown tenant \"{tenant_id:?}\"")]
    UnknownTenant { tenant_id: String },
    #[error("credentials not provided")]
    CredentialsNotProvided,
    #[error("security error {0}")]
    KeyringError(#[from] KeyringError),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    client_id: String,
    client_secret: String,
    project_id: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            project_id: String::new(),
        }
    }
}

impl Configuration {
    pub fn project_id(&self) -> String {
        self.project_id.to_owned()
    }

    pub fn set_project_id(&mut self, project_id: String) {
        self.project_id = project_id;
    }

    pub fn client_id(&self) -> String {
        self.client_id.to_owned()
    }

    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    pub fn client_secret(&self) -> String {
        self.client_secret.to_owned()
    }

    pub fn set_client_secret(&mut self, client_secret: String) {
        self.client_secret = client_secret;
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
        match fs::read_to_string(path.clone()) {
            Ok(configuration) => {
                let configuration = serde_yaml::from_str(&configuration);
                match configuration {
                    Ok(configuration) => Ok(configuration),
                    Err(cause) => Err(ConfigurationError::FailedToLoadData {
                        cause: Box::new(cause),
                    }),
                }
            }
            Err(cause) => Err(ConfigurationError::FailedToLoadData {
                cause: Box::new(cause),
            }),
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

        let file = File::create(&path);
        match file {
            Ok(file) => {
                let writer: Box<dyn Write> = Box::new(file);
                Ok(self.write(writer)?)
            }
            Err(e) => Err(ConfigurationError::FailedToWriteData { cause: Box::new(e) }),
        }
    }

    pub fn save_to_default(&self) -> Result<(), ConfigurationError> {
        self.save(&Self::get_default_configuration_file_path()?)
    }
}

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("keyring error")]
    CannotAccessKeyringEntity(#[from] keyring::Error),
}

pub struct Keyring {}

impl Default for Keyring {
    fn default() -> Keyring {
        Keyring {}
    }
}

impl Keyring {
    fn get_entry(&self, key: &String) -> Result<Entry, KeyringError> {
        Ok(Entry::new(DEFAULT_APPLICATION_ID, key.as_str())?)
    }

    pub fn get(&self, key: &String) -> Result<Option<String>, KeyringError> {
        match self.get_entry(key)?.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(e) => match e {
                keyring::Error::NoEntry => Ok(None),
                _ => Err(KeyringError::from(e)),
            },
        }
    }

    pub fn put(&self, key: &String, value: String) -> Result<(), KeyringError> {
        self.get_entry(key)?.set_password(value.as_str())?;
        Ok(())
    }

    pub fn delete(&self, key: &String) -> Result<(), KeyringError> {
        self.get_entry(key)?.delete_password()?;
        Ok(())
    }
}
