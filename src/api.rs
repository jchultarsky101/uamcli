/// This module provides high-level abstraction client over the Unity REST API.
///
/// Currently, only the minimum required  Asset Manager functionality is supported, but future releases may
/// cover more (e.g. Projects, Organizations, etc.). It is sufficient for most asset-related operations
/// such as file upload/download.
use crate::{
    client::Client,
    configuration::Configuration,
    model::{Asset, AssetIdentity, AssetStatus, MetadataEntry},
};
use std::{cell::RefCell, collections::HashMap, fs::File, path::PathBuf};
use thiserror::Error;

/// Wrapper error for all errors that may occur while
/// invoking API functions.
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

/// API client wrapper.
pub struct Api {
    configuration: RefCell<Configuration>, // configuration object
    client: Option<Client>,                // low-level HTTP client object
}

impl Api {
    /// Returns a new Api object.
    ///
    /// Parameters:
    ///
    /// * configuration - thread safe reference to a configuration object
    pub fn new(configuration: &RefCell<Configuration>) -> Api {
        Api {
            configuration: configuration.clone(),
            client: None,
        }
    }

    /// Returns a thread-safe reference to the configuration object used
    /// by this API
    pub fn configuration(&self) -> RefCell<Configuration> {
        self.configuration.clone()
    }

    /// Initializes the API wrapper based on the configuration values.
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

    // Terminates the HTTP session, invalidates the token.
    //
    // NOTE: This method is just a place holder and not imlemented for the moment. It is not used when we use service account authenication.
    // pub fn logoff(&mut self) -> Result<(), ApiError> {
    //     self.client = None;
    //     todo!("Implement logoff");
    // }

    /// Returns a list of Asset objects matching the search criteria in the current project.
    ///
    /// Parameters:
    ///
    /// asset_id: (optional) the asset identity (if, version). If none provided, it will return all assets in the project
    pub async fn search_asset(
        &mut self,
        asset_id: Option<AssetIdentity>,
        asset_name: Option<String>,
    ) -> Result<Vec<Asset>, ApiError> {
        self.init().await?;

        match &self.client {
            Some(client) => Ok(client.search_asset(asset_id, asset_name).await?),
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    /// Creates a new asset and uploads related files.
    ///
    /// Parameters:
    ///
    /// * name - unique asset name as it would apper in the Asset Manager UI
    /// * description - asset human-readable description
    /// * data_files - list of PathBuff references for files to be uploaded
    pub async fn create_asset(
        &mut self,
        name: String,
        description: Option<String>,
        data_files: Vec<&PathBuf>,
        publish: bool,
    ) -> Result<AssetIdentity, ApiError> {
        self.init().await?;
        log::trace!("Creating asset {}...", name.to_owned());
        match &self.client {
            Some(client) => {
                let id = client.create_asset(name, description, data_files).await?;

                if publish {
                    self.set_asset_status(&id, &AssetStatus::InReview).await?;
                    self.set_asset_status(&id, &AssetStatus::Approved).await?;
                    self.set_asset_status(&id, &AssetStatus::Published).await?;
                }

                Ok(id)
            }
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    /// Returns an Option<Asset>
    ///
    /// If an asset with the required identity exists, it will return the asset.
    /// If there are no matches, it will return None.
    ///
    /// Parameters:
    ///
    /// * identity: a reference to an AssetIdentity, whcih is comprised by asset ID and asset varsion.
    pub async fn get_asset(&mut self, identity: &AssetIdentity) -> Result<Option<Asset>, ApiError> {
        self.init().await?;
        log::trace!("Retrieving asset {}...", identity.id());
        match &self.client {
            Some(client) => Ok(client.get_asset(identity).await?),
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    /// Sets the status for an existing asset.
    ///
    /// In Unity Asset Manager, there is a concept of an asset workflow.
    /// The value of a status depends on the previous value.
    /// The normal chain is: draft -> inreview -> approved --> published.
    /// If the new status is incorrect, an error will be thrown.
    ///
    /// Parameters:
    /// * identity: reference to the asset's identity
    /// * status: the desired status
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

    /// Download the files associated with an existing asset.
    ///
    /// Parameters:
    /// * identity: a reference to the asset's identity
    /// * output_directory: and optional path to the target download directory. If None, the system's default download directory will be used
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

    /// Parses an input file, extracts the names and values for all
    /// properties specified in the file and upserts them to the
    /// asset.
    ///
    /// The current implementation only works with a CSV file format.
    /// The CSV must have only two columns with a header line containing the column names, which are: Name, Value
    ///
    /// Here is an example CSV file that could be used as an input to this function:
    ///
    /// ````csv
    /// Name, Value
    /// My_property_name, My_property_value
    /// ````
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

    pub async fn delete_asset_metadata(
        &mut self,
        identity: &AssetIdentity,
        keys: &Vec<String>,
    ) -> Result<(), ApiError> {
        self.init().await?;
        log::trace!("Deleting asset metadata for asset {}...", identity.id());

        match &self.client {
            Some(client) => {
                let asset = client.get_asset(identity).await?;

                match asset {
                    Some(asset) => {
                        log::trace!("The asset exists");

                        client.delete_metadata(&asset.identity(), keys).await?;

                        Ok(())
                    }
                    None => Err(ApiError::AssetNotFound),
                }
            }
            None => Err(ApiError::ClientNotInitialized),
        }
    }

    /// Generates thumbnails and previews for the specified asset..
    ///
    /// Parameters:
    ///
    /// asset_id - (optional) the asset identity (id, version). If not provided, it will attempt to generate thumbnails for all assets in the project that do not have one.
    pub async fn generate_asset_thumbnail(
        &mut self,
        asset_id: Option<AssetIdentity>,
    ) -> Result<Vec<Asset>, ApiError> {
        self.init().await?;

        match &self.client {
            Some(client) => {
                let assets = client.search_asset(asset_id, None).await?;

                for asset in assets.clone() {
                    log::trace!(
                        "Verifying thumbnails for asset id={}, version={}...",
                        asset.identity().id(),
                        asset.identity().version()
                    );

                    match asset.preview_file() {
                        None => {
                            log::trace!("This asset does not have a thumbnail!");

                            let preview_file_dataset_id =
                                asset.preview_file_dataset_id().unwrap_or_default();

                            log::trace!(
                                "The preview dataset ID is {}",
                                preview_file_dataset_id.to_owned()
                            );

                            //let files: Vec<String> = Vec::new();

                            /*
                            client
                                .generate_thumbnails(
                                    &asset.identity(),
                                    &preview_file_dataset_id,
                                    files,
                                )
                                .await?;
                            */
                        }
                        Some(_) => (),
                    }
                }

                Ok(assets)
            }
            None => Err(ApiError::ClientNotInitialized),
        }
    }
}
