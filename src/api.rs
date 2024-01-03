use crate::{
    client::Client,
    configuration::Configuration,
    model::{Asset, AssetIdentity},
};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
};
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

        let client = Client::new(project_id, environment_id, client_id, client_secret)?;

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
        data_file: &Path,
    ) -> Result<AssetIdentity, ApiError> {
        self.init().await?;
        log::trace!("Creating asset {}...", name.to_owned());
        match &self.client {
            Some(client) => Ok(client.create_asset(name, description, data_file).await?),
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
}
