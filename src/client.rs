use base64::{engine::general_purpose, Engine};
use reqwest::{Client as HttpClient, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, time::Duration};
use strfmt::strfmt;
use thiserror::Error;

use crate::{api::ApiError, model::Asset};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("failed to obtain access token from provider")]
    FailedToObtainToken,
    #[error("invalid client ID in configuration")]
    InvalidClientId,
    #[error("invalid client secret in configuration")]
    InvalidClientSecret,
    #[error("error during HTTP request")]
    HttpError(#[from] reqwest::Error),
    #[error("unexpected response from server: {0}")]
    UnexpectedResponse(StatusCode),
    #[error("parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

#[derive(Debug, Deserialize)]
struct AuthenticationResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
}

#[derive(Debug, Serialize)]
struct PaginationRequest {
    #[serde(rename = "sortingField")]
    sorting_field: String,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        PaginationRequest {
            sorting_field: "name".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct AssetSearchRequest {
    #[serde(rename = "projectIds")]
    project_ids: Vec<String>,
    #[serde(rename = "pagination")]
    pagination: PaginationRequest,
}

impl AssetSearchRequest {
    fn new(project_id: String) -> Self {
        AssetSearchRequest {
            project_ids: vec![project_id.to_owned()],
            pagination: PaginationRequest::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AssetResponse {
    #[serde(rename = "assetId")]
    asset_id: String,
    #[serde(rename = "assetVersion")]
    asset_version: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "tags")]
    tags: Vec<String>,
    #[serde(rename = "systemTags")]
    system_tags: Vec<String>,
    #[serde(rename = "labels")]
    labels: Vec<String>,
    #[serde(rename = "primaryType")]
    primary_type: String,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "isFrozen")]
    frozen: bool,
    #[serde(rename = "sourceProjectId")]
    source_project_id: String,
    #[serde(rename = "projectIds")]
    project_ids: Vec<String>,
    #[serde(rename = "previewFile")]
    preview_file: Option<String>,
    #[serde(rename = "previewFileDatasetId")]
    preview_file_dataset_id: Option<String>,
}

impl Into<Asset> for AssetResponse {
    fn into(self) -> Asset {
        Asset::new(
            self.asset_id,
            self.name,
            self.asset_version,
            self.tags,
            self.system_tags,
            self.labels,
            self.primary_type,
            self.status,
            self.frozen,
            self.source_project_id,
            self.project_ids,
            self.preview_file,
            self.preview_file_dataset_id,
        )
    }
}

#[derive(Debug, Deserialize)]
struct AssetSearchResponse {
    #[serde(rename = "next")]
    next: String,
    #[serde(rename = "assets")]
    assets: Vec<AssetResponse>,
}

const UNITY_TOKEN_EXCHANGE_URL: &'static str = "https://services.api.unity.com/auth/v1/token-exchange?projectId={PROJECT_ID}&environmentId={ENVIRONMENT_ID}";
const UNITY_PRODUCTION_SERVICES_BASE_URL: &'static str = "https://services.api.unity.com";

#[derive(Debug)]
pub struct Client {
    http: HttpClient,
    project_id: String,
    environment_id: String,
    client_id: String,
    client_secret: String,
}

impl Client {
    pub fn new(
        project_id: String,
        environment_id: String,
        client_id: String,
        client_secret: String,
    ) -> Result<Client, ClientError> {
        let connection_timeout = Duration::from_secs(30);
        let request_timeout = connection_timeout.clone();

        let http = HttpClient::builder()
            .user_agent("uamcli")
            .connect_timeout(connection_timeout)
            .timeout(request_timeout)
            .build()?;

        let client = Self {
            http,
            project_id,
            environment_id,
            client_id,
            client_secret,
        };

        Ok(client)
    }

    fn encode_credentials(client_id: String, client_secret: String) -> String {
        let combined_credentials = [client_id.clone(), client_secret.clone()]
            .join(":")
            .to_owned();
        let encoded_credentials = general_purpose::STANDARD.encode(combined_credentials.to_owned());
        let mut authorization_header_value = String::from("Basic ");
        authorization_header_value.push_str(encoded_credentials.as_str());

        authorization_header_value
    }

    /// Login by exchanging client ID and client secret for an access token
    pub async fn login(&mut self) -> Result<String, ClientError> {
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("PROJECT_ID".to_string(), self.project_id.to_owned());
        token_values.insert("ENVIRONMENT_ID".to_string(), self.environment_id.to_owned());
        let url = strfmt(UNITY_TOKEN_EXCHANGE_URL, &token_values).unwrap();

        log::trace!("Request: POST {}", url);

        let response = self
            .http
            .post(url)
            .header(
                "Authorization",
                Self::encode_credentials(self.client_id.to_owned(), self.client_secret.to_owned()),
            )
            .header("cache-control", "no-cache")
            .header("content-length", 0)
            .send()
            .await;

        match response {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    let response_text = response.text().await;
                    match response_text {
                        Ok(response_text) => {
                            log::trace!("Response: {}", response_text.to_owned());

                            let response: AuthenticationResponse =
                                serde_yaml::from_str(&response_text).unwrap();
                            let token = response.access_token;
                            Ok(token)
                        }
                        Err(_) => Err(ClientError::UnexpectedResponse(status)),
                    }
                } else {
                    Err(ClientError::UnexpectedResponse(status))
                }
            }
            Err(_) => Err(ClientError::FailedToObtainToken),
        }
    }

    /// Get asset
    pub async fn get_asset(&self) -> Result<(), ClientError> {
        todo!("implement get_asset")
    }

    /// Searches for an asset and returns a list of assets found
    pub async fn search_asset(&self) -> Result<Vec<Asset>, ClientError> {
        let mut url: String = UNITY_PRODUCTION_SERVICES_BASE_URL.to_string();
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("projectId".to_string(), self.project_id.to_owned());
        let path = strfmt(
            "/assets/v1/projects/{projectId}/assets/search",
            &token_values,
        )
        .unwrap();
        url.push_str(path.as_str());

        let asset_search_request = AssetSearchRequest::new(self.project_id.to_owned());

        log::trace!("POST {}", url);

        let response = self
            .http
            .post(url)
            .header("cache-control", "no-cache")
            .timeout(Duration::from_secs(30))
            .basic_auth(
                self.client_id.to_owned(),
                Some(self.client_secret.to_owned()),
            )
            .json(&asset_search_request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content = response.text().await?;

            log::trace!("Response: {}", content);

            let response: AssetSearchResponse = serde_yaml::from_str(&content).unwrap();
            let assets: Vec<Asset> = response.assets.into_iter().map(|a| a.into()).collect();

            Ok(assets)
        } else {
            Err(ClientError::UnexpectedResponse(status))
        }
    }
}
