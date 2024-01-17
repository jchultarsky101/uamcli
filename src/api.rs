use crate::{
    client::Client,
    configuration::Configuration,
    model::{Asset, AssetIdentity, AssetStatus, MetadataEntry},
};
use std::{cell::RefCell, collections::HashMap, fs::File, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("client ID is not provided or it is invalid")]
    InvalidClientId,
    #[error("client secret is not provided or it is invalid")]
    InvalidClientSecret,
    #[error("client error")]
    HttpClientError(#[from] crate::client::ClientError),
    #[error("HTTP client not initialized")]
    ClientNotInitialized,
    #[error("input/output error")]
    InputOutput(#[from] std::io::Error),
    #[error("CSV format parsing error")]
    CsvParse(#[from] csv::Error),
    #[error("asset not found")]
    AssetNotFound,
}

pub struct Api {
    configuration: RefCell<Configuration>,
    client: Option<Client>,
}

impl Api {
    pub fn new(configuration: &RefCell<Configuration>) -> Api {
        Api {
            configuration: configuration.clone(),
            client: None,
        }
    }

    pub fn configuration(&self) -> RefCell<Configuration> {
        self.configuration.clone()
    }

    pub async fn init(&mut self) -> Result<(), ApiError> {
        let organization_id = self.configuration.borrow().organization_id();
        let project_id = self.configuration.borrow().project_id();
        let environment_id = self.configuration.borrow().environment_id();
        //let account = self.configuration.borrow().account();
        let client_id = self.configuration.borrow().client_id();
        let client_secret = self.configuration.borrow().client_secret();

        if client_id.is_none() || client_id.clone().unwrap().is_empty() {
            return Err(ApiError::InvalidClientId);
        }
        let client_id = client_id.unwrap();

        if client_secret.is_none() || client_secret.clone().unwrap().is_empty() {
            return Err(ApiError::InvalidClientSecret);
        }
        let client_secret = client_secret.unwrap();

        let client = Client::new(
            organization_id,
            project_id,
            environment_id,
            client_id,
            client_secret,
        )?;

        self.client = Some(client);

        Ok(())
    }

    pub fn logoff(&mut self) -> Result<(), ApiError> {
        self.client = None;

        todo!("Implement logoff");
    }

    pub async fn search_asset(&mut self) -> Result<Vec<Asset>, ApiError> {
        self.init().await?;
        log::trace!("Searching for assets...");
        match &self.client {
            Some(client) => Ok(client.search_asset().await?),
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    pub async fn create_asset(
        &mut self,
        name: String,
        description: Option<String>,
        data_files: Vec<&PathBuf>,
    ) -> Result<AssetIdentity, ApiError> {
        self.init().await?;
        log::trace!("Creating asset {}...", name.to_owned());
        match &self.client {
            Some(client) => Ok(client.create_asset(name, description, data_files).await?),
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    pub async fn get_asset(&mut self, identity: &AssetIdentity) -> Result<Option<Asset>, ApiError> {
        self.init().await?;
        log::trace!("Retrieving asset {}...", identity.id());
        match &self.client {
            Some(client) => Ok(client.get_asset(identity).await?),
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    pub async fn set_asset_status(
        &mut self,
        identity: &AssetIdentity,
        status: &AssetStatus,
    ) -> Result<(), ApiError> {
        self.init().await?;
        log::trace!("Publishing asset {}...", identity.id());
        match &self.client {
            Some(client) => Ok(client.set_asset_status(identity, status).await?),
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    pub async fn download_asset(
        &mut self,
        identity: &AssetIdentity,
        output_directory: Option<&PathBuf>,
    ) -> Result<(), ApiError> {
        self.init().await?;
        log::trace!("Downloading all files for asset {}...", identity.id());
        match &self.client {
            Some(client) => Ok(client
                .download_all_asset_files(identity, output_directory)
                .await?),
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    pub async fn upload_asset_metadata(
        &mut self,
        identity: &AssetIdentity,
        data_file_path: &PathBuf,
    ) -> Result<(), ApiError> {
        self.init().await?;
        log::trace!("Uploading asset metadata for asset {}...", identity.id());

        log::trace!("Reading data from file...");
        let file = File::open(data_file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut records: HashMap<String, Option<String>> = HashMap::new();
        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: MetadataEntry = result?;
            records.insert(record.name, record.value);
        }

        match &self.client {
            Some(client) => {
                let asset = client.get_asset(identity).await?;

                match asset {
                    Some(mut asset) => {
                        // filter out any properties that may have no value
                        let md: HashMap<String, String> = records
                            .into_iter()
                            .filter_map(|(k, v)| {
                                if v.is_some() {
                                    Some((k, v.unwrap()))
                                } else {
                                    None
                                }
                            })
                            .collect();

                        match &self.client {
                            Some(client) => {
                                // read the list of registered properties
                                for (n, _) in md.iter() {
                                    let definition = client.get_metadata_definition(&n).await;
                                    match definition {
                                        Ok(definition) => match definition {
                                            Some(_) => (),
                                            None => {
                                                client.register_metadata_definition(&n).await?;
                                            }
                                        },
                                        Err(e) => {
                                            println!(
                                                "Error while obtaining property definition for {}. Error: {}",
                                                n.to_owned(), e
                                            );
                                        }
                                    }
                                }

                                let metadata: Option<Option<HashMap<String, String>>> =
                                    Some(Some(md));
                                asset.set_metadata(metadata);
                                Ok(client.update_asset(&asset).await?)
                            }
                            None => Err(ApiError::ClientNotInitialized),
                        }
                    }
                    None => Err(ApiError::AssetNotFound),
                }
            }
            None => Err(ApiError::ClientNotInitialized),
        }
    }
}
