use crate::model::{Asset, AssetIdentity, Dataset};
use base64::{engine::general_purpose, Engine};
use dirs;
use futures::StreamExt;
use reqwest::{Body, Client as HttpClient, StatusCode};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};
use strfmt::strfmt;
use thiserror::Error;
use tokio_util::codec::{BytesCodec, FramedRead};
use url::Url;

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
    #[error("no source dataset")]
    NoSourceDataset,
    #[error("no preview dataset")]
    NoPreviewDataset,
    #[error("input/output error")]
    InputOutput(#[from] std::io::Error),
    #[error("no download directory available")]
    NoDownloadDirectory,
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
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "tags")]
    tags: Option<Vec<String>>,
    #[serde(rename = "systemTags")]
    system_tags: Option<Vec<String>>,
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
    datasets: Option<Vec<Dataset>>,
}

impl Into<Asset> for AssetResponse {
    fn into(self) -> Asset {
        Asset::new(
            AssetIdentity::new(self.asset_id, self.asset_version),
            self.name,
            self.description,
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
            self.datasets,
        )
    }
}

#[derive(Debug, Deserialize)]
struct AssetSearchResponse {
    #[serde(rename = "next")]
    _next: String,
    #[serde(rename = "assets")]
    assets: Vec<AssetResponse>,
}

#[derive(Debug, Serialize)]
struct AssetCreateRequest {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "primaryType")]
    primary_type: String,
}

impl AssetCreateRequest {
    fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            primary_type: "3D Model".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AssetCreateResponse {
    #[serde(rename = "assetId")]
    id: String,
    #[serde(rename = "assetVersion")]
    version: String,
    #[serde(rename = "datasets")]
    datasets: Vec<Dataset>,
}

impl Into<AssetIdentity> for AssetCreateResponse {
    fn into(self) -> AssetIdentity {
        AssetIdentity::new(self.id, self.version)
    }
}
#[derive(Debug, Serialize)]
struct DatasetCreateRequest {
    name: String,
    description: Option<String>,
}

impl DatasetCreateRequest {
    fn new(name: String) -> Self {
        DatasetCreateRequest {
            name,
            description: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct DatasetCreateResponse {
    #[serde(rename = "datasetId")]
    id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "systemTags")]
    system_tags: Vec<String>,
}

#[derive(Debug, Serialize)]
struct FileCreateRequest {
    #[serde(rename = "filePath")]
    path: String,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "fileSize")]
    file_size: u64,
}

impl FileCreateRequest {
    fn new(path: String, description: Option<String>, file_size: u64) -> Self {
        Self {
            path,
            description,
            file_size,
        }
    }
}

#[derive(Debug, Deserialize)]
struct FileCreateResponse {
    #[serde(rename = "uploadUrl")]
    upload_url: String,
}

#[derive(Debug, Deserialize)]
struct AssetDownloadUrlResponse {
    #[serde(rename = "datasetId")]
    dataset_id: String,
    #[serde(rename = "filePath")]
    file_path: String,
    #[serde(rename = "url")]
    url: Url,
}

#[derive(Debug, Deserialize)]
struct AllAssetDownloadUrlsResponse {
    #[serde(rename = "files")]
    files: Vec<AssetDownloadUrlResponse>,
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

    async fn create_file(
        &self,
        asset_identity: &AssetIdentity,
        dataset_id: &String,
        file_path: &Path,
    ) -> Result<FileCreateResponse, ClientError> {
        log::trace!("Creating file...");

        let mut url: String = UNITY_PRODUCTION_SERVICES_BASE_URL.to_string();
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("projectId".to_string(), self.project_id.to_owned());
        token_values.insert("assetId".to_string(), asset_identity.id());
        token_values.insert("assetVersion".to_string(), asset_identity.version());
        token_values.insert("datasetId".to_string(), dataset_id.to_owned());
        let path = strfmt("/assets/v1/projects/{projectId}/assets/{assetId}/versions/{assetVersion}/datasets/{datasetId}/files", &token_values).unwrap();
        url.push_str(path.as_str());

        let file = File::open(file_path)?;
        let file_size = file.metadata().unwrap().len();
        let file_name = file_path.file_name().unwrap();
        let file_create_request =
            FileCreateRequest::new(String::from(file_name.to_string_lossy()), None, file_size);

        log::trace!("{:?}", &file_create_request);

        log::trace!("POST {}", url);

        let response = self
            .http
            .post(url)
            .header("cache-control", "no-cache")
            .timeout(Duration::from_secs(120))
            .basic_auth(
                self.client_id.to_owned(),
                Some(self.client_secret.to_owned()),
            )
            .json(&file_create_request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content = response.text().await?;

            log::trace!("Response: {}", content);

            let response = serde_yaml::from_str::<FileCreateResponse>(&content).unwrap();

            Ok(response)
        } else {
            Err(ClientError::UnexpectedResponse(status))
        }
    }

    async fn finalize_file_upload(
        &self,
        asset_identity: &AssetIdentity,
        file_path: &Path,
    ) -> Result<(), ClientError> {
        let file_name = String::from(file_path.file_name().unwrap().to_string_lossy());
        let encoded_file_name: String =
            url::form_urlencoded::byte_serialize(file_name.as_bytes()).collect();
        let path_str = String::from(file_path.to_string_lossy());
        log::trace!("Finalizing file upload for file {}...", &path_str);

        let mut url: String = UNITY_PRODUCTION_SERVICES_BASE_URL.to_string();
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("projectId".to_string(), self.project_id.to_owned());
        token_values.insert("assetId".to_string(), asset_identity.id());
        token_values.insert("assetVersion".to_string(), asset_identity.version());
        token_values.insert("filename".to_string(), encoded_file_name);
        let url_path = strfmt("/assets/v1/projects/{projectId}/assets/{assetId}/versions/{assetVersion}/files/{filename}/finalize", &token_values).unwrap();
        url.push_str(url_path.as_str());

        log::trace!("POST {}", url);
        let response = self
            .http
            .post(url)
            .header("cache-control", "no-cache")
            .header("content-length", "0")
            .timeout(Duration::from_secs(120))
            .basic_auth(
                self.client_id.to_owned(),
                Some(self.client_secret.to_owned()),
            )
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content = response.text().await?;

            log::trace!("Response: {}", content);
            Ok(())
        } else {
            Err(ClientError::UnexpectedResponse(status))
        }
    }

    pub async fn upload_file(
        &self,
        asset_identity: &AssetIdentity,
        dataset_id: &String,
        path: &Path,
    ) -> Result<(), ClientError> {
        let create_result = self.create_file(asset_identity, dataset_id, path).await?;

        log::trace!(
            "Uploading file {} to {}...",
            String::from(path.to_string_lossy()),
            create_result.upload_url
        );
        let file = tokio::fs::File::open(path).await?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        let client = reqwest::Client::new();
        let _ = client
            .post("http://httpbin.org/post")
            .body(body)
            .send()
            .await?;

        self.finalize_file_upload(asset_identity, path).await?;

        Ok(())
    }

    async fn get_asset_download_urls(
        &self,
        asset_identity: &AssetIdentity,
    ) -> Result<AllAssetDownloadUrlsResponse, ClientError> {
        log::trace!("Reading all download URLs for asset...");

        let mut url: String = UNITY_PRODUCTION_SERVICES_BASE_URL.to_string();
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("projectId".to_string(), self.project_id.to_owned());
        token_values.insert("assetId".to_string(), asset_identity.id());
        token_values.insert("assetVersion".to_string(), asset_identity.version());
        let path = strfmt("/assets/v1/projects/{projectId}/assets/{assetId}/versions/{assetVersion}/download-urls", &token_values).unwrap();
        url.push_str(path.as_str());

        log::trace!("GET {}", url);

        let response = self
            .http
            .get(url)
            .header("cache-control", "no-cache")
            .timeout(Duration::from_secs(30))
            .basic_auth(
                self.client_id.to_owned(),
                Some(self.client_secret.to_owned()),
            )
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content = response.text().await?;

            log::trace!("Response: {}", content);

            let response = serde_yaml::from_str::<AllAssetDownloadUrlsResponse>(&content).unwrap();

            Ok(response)
        } else {
            Err(ClientError::UnexpectedResponse(status))
        }
    }

    pub async fn download_all_asset_files(
        &self,
        asset_identity: &AssetIdentity,
        output_path: Option<&PathBuf>,
    ) -> Result<(), ClientError> {
        log::trace!(
            "Downloading files for asset id={}, version={}",
            asset_identity.id(),
            asset_identity.version()
        );
        let urls: Vec<(PathBuf, Url)> = self
            .get_asset_download_urls(asset_identity)
            .await?
            .files
            .into_iter()
            .map(|i| (PathBuf::from(i.file_path), i.url))
            .collect();

        for url in urls {
            self.download_file(url, output_path.to_owned()).await?;
        }
        Ok(())
    }

    async fn download_file(
        &self,
        item: (PathBuf, Url),
        output_path: Option<&PathBuf>,
    ) -> Result<(), ClientError> {
        let path = match output_path {
            Some(path) => Some(path.to_owned()),
            None => dirs::download_dir(),
        };

        match path {
            Some(path) => {
                let path = path.join(item.0);

                log::trace!(
                    "Downloading file from URL {} to local path {}...",
                    item.1,
                    path.clone().into_os_string().into_string().unwrap()
                );

                let response = reqwest::get(item.1).await?;
                let status = &response.status();
                if status.is_success() {
                    let mut file = { File::create(path)? };
                    let mut stream = response.bytes_stream();
                    while let Some(item) = stream.next().await {
                        let chunk = item.unwrap();
                        file.write_all(&chunk)?;
                    }

                    Ok(())
                } else {
                    Err(ClientError::UnexpectedResponse(*status))
                }
            }
            None => Err(ClientError::NoDownloadDirectory),
        }
    }

    pub async fn create_asset(
        &self,
        name: String,
        description: Option<String>,
        data_file: &Path,
    ) -> Result<AssetIdentity, ClientError> {
        log::trace!("Creating an asset...");

        let mut url: String = UNITY_PRODUCTION_SERVICES_BASE_URL.to_string();
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("projectId".to_string(), self.project_id.to_owned());
        let path = strfmt("/assets/v1/projects/{projectId}/assets", &token_values).unwrap();
        url.push_str(path.as_str());

        let asset_create_request = AssetCreateRequest::new(name, description);

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
            .json(&asset_create_request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content = response.text().await?;

            log::trace!("Response: {}", content);

            let response = serde_yaml::from_str::<AssetCreateResponse>(&content).unwrap();
            let datasets = response.datasets.clone();
            let identity: AssetIdentity = response.into();

            let source_dataset = datasets.iter().find(|dataset| dataset.name().eq("Source"));
            match source_dataset {
                Some(source_dataset) => {
                    let source_dataset_id = source_dataset.id();
                    self.upload_file(&identity, &source_dataset_id, data_file)
                        .await?;
                }
                None => return Err(ClientError::NoSourceDataset),
            }

            Ok(identity)
        } else {
            Err(ClientError::UnexpectedResponse(status))
        }
    }

    /// Get asset
    pub async fn get_asset(&self, identity: &AssetIdentity) -> Result<Option<Asset>, ClientError> {
        let mut url: String = UNITY_PRODUCTION_SERVICES_BASE_URL.to_string();
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("projectId".to_string(), self.project_id.to_owned());
        token_values.insert("assetId".to_string(), identity.id());
        token_values.insert("assetVersion".to_string(), identity.version());
        let path = strfmt(
            "/assets/v1/projects/{projectId}/assets/{assetId}/versions/{assetVersion}",
            &token_values,
        )
        .unwrap();
        url.push_str(path.as_str());

        log::trace!("GET {}", url);

        let response = self
            .http
            .get(url)
            .header("cache-control", "no-cache")
            .header("content-length", "0")
            .timeout(Duration::from_secs(30))
            .basic_auth(
                self.client_id.to_owned(),
                Some(self.client_secret.to_owned()),
            )
            .query(&[("IncludeFields", "*")])
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content = response.text().await?;

            log::trace!("Response: {}", content);

            let response = serde_yaml::from_str::<AssetResponse>(&content).unwrap();
            let asset = response.into();

            Ok(Some(asset))
        } else {
            Err(ClientError::UnexpectedResponse(status))
        }
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
            .query(&[("includeFields", "*")])
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

    async fn create_dataset(
        &self,
        name: String,
        asset_id: String,
        asset_version: String,
    ) -> Result<String, ClientError> {
        let mut url: String = UNITY_PRODUCTION_SERVICES_BASE_URL.to_string();
        let mut token_values: HashMap<String, String> = HashMap::new();
        token_values.insert("projectId".to_string(), self.project_id.to_owned());
        token_values.insert("assetId".to_string(), asset_id.to_owned());
        token_values.insert("asset_version".to_string(), asset_version.to_owned());
        let path = strfmt(
            "/assets/v1/projects/{projectId}/assets/{assetId}/versions/{assetVersion}/datasets",
            &token_values,
        )
        .unwrap();
        url.push_str(path.as_str());

        let dataset_request = DatasetCreateRequest::new(name.to_owned());

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
            .json(&dataset_request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let content = response.text().await?;

            log::trace!("Response: {}", content);

            let response: DatasetCreateResponse = serde_yaml::from_str(&content).unwrap();

            Ok(response.id)
        } else {
            Err(ClientError::UnexpectedResponse(status))
        }
    }
}