//! Data model structures for interfacing with Unity
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

/// A wrapper for all errors generated when working with asset status.
#[derive(Debug, Error)]
pub struct AssetStatusParseError(String);

impl std::fmt::Display for AssetStatusParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to parse status value of {}", self.0)
    }
}

/// Unity Asset status abstraction.
#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub enum AssetStatus {
    Draft,
    InReview,
    Approved,
    Published,
    Rejected,
    Withdrawn,
}

impl std::fmt::Display for AssetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssetStatus::Draft => "draft",
                AssetStatus::InReview => "inreview",
                AssetStatus::Approved => "approved",
                AssetStatus::Published => "published",
                AssetStatus::Rejected => "rejected",
                AssetStatus::Withdrawn => "withdrawn",
            }
        )
    }
}

impl FromStr for AssetStatus {
    type Err = AssetStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.to_lowercase();
        match value.as_ref() {
            "draft" => Ok(AssetStatus::Draft),
            "inreview" => Ok(AssetStatus::InReview),
            "approved" => Ok(AssetStatus::Approved),
            "published" => Ok(AssetStatus::Published),
            "rejected" => Ok(AssetStatus::Rejected),
            "withdrawn" => Ok(AssetStatus::Withdrawn),
            _ => Err(AssetStatusParseError(value.to_string())),
        }
    }
}

/// Asset identity abstraction.
#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct AssetIdentity {
    id: String,      // Unity asset ID
    version: String, // Unity asset version
}

impl AssetIdentity {
    pub fn new(id: String, version: String) -> Self {
        AssetIdentity { id, version }
    }

    pub fn id(&self) -> String {
        self.id.to_owned()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id.to_owned();
    }

    pub fn version(&self) -> String {
        self.version.to_owned()
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version.to_owned();
    }
}

/// Unity Dataset abstraction.
#[derive(Debug, PartialOrd, PartialEq, Clone, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "datasetId")]
    id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "primaryType")]
    primary_type: Option<String>,
}

impl Dataset {
    pub fn new(id: String, name: String, primary_type: Option<String>) -> Self {
        Self {
            id,
            name,
            primary_type,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_owned()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id.to_owned();
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.to_owned();
    }

    pub fn set_primary_type(&mut self, primary_type: Option<String>) {
        self.primary_type = primary_type.to_owned();
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThumbnailGenerationRequest {
    #[serde(rename = "inputFiles")]
    input_files: Vec<String>,
}

impl ThumbnailGenerationRequest {
    pub fn new(input_files: Vec<String>) -> Self {
        Self { input_files }
    }
}

/// Unity Asset abstraction
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Asset {
    identity: AssetIdentity,
    name: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
    system_tags: Option<Vec<String>>,
    labels: Vec<String>,
    primary_type: String,
    status: String,
    source_project_id: String,
    project_ids: Vec<String>,
    preview_file: Option<String>,
    preview_file_dataset_id: Option<String>,
    datasets: Option<Vec<Dataset>>,
    metadata: Option<Option<::std::collections::HashMap<String, String>>>,
}

impl Asset {
    pub fn new(
        identity: AssetIdentity,
        name: String,
        description: Option<String>,
        tags: Option<Vec<String>>,
        system_tags: Option<Vec<String>>,
        labels: Vec<String>,
        primary_type: String,
        status: String,
        source_project_id: String,
        project_ids: Vec<String>,
        preview_file: Option<String>,
        preview_file_dataset_id: Option<String>,
        datasets: Option<Vec<Dataset>>,
        metadata: Option<Option<::std::collections::HashMap<String, String>>>,
    ) -> Self {
        Asset {
            identity,
            name,
            description,
            tags,
            system_tags,
            labels,
            primary_type,
            status,
            source_project_id,
            project_ids,
            preview_file,
            preview_file_dataset_id,
            datasets,
            metadata,
        }
    }

    pub fn identity(&self) -> AssetIdentity {
        self.identity.to_owned()
    }

    pub fn set_id(&mut self, identity: AssetIdentity) {
        self.identity = identity.to_owned();
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.to_owned();
    }

    pub fn description(&self) -> Option<String> {
        self.description.to_owned()
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description.to_owned();
    }

    pub fn tags(&self) -> Option<Vec<String>> {
        self.tags.clone()
    }

    pub fn set_tabs(&mut self, tags: Option<Vec<String>>) {
        self.tags = tags.clone();
    }

    pub fn system_tags(&self) -> Option<Vec<String>> {
        self.system_tags.clone()
    }

    pub fn set_system_tags(&mut self, system_tags: Option<Vec<String>>) {
        self.system_tags = system_tags.clone();
    }

    pub fn labels(&self) -> Vec<String> {
        self.labels.clone()
    }

    pub fn set_labels(&mut self, labels: Vec<String>) {
        self.labels = labels.clone();
    }

    pub fn primary_type(&self) -> String {
        self.primary_type.to_owned()
    }

    pub fn set_primary_type(&mut self, primary_type: String) {
        self.primary_type = primary_type.to_owned();
    }

    pub fn status(&self) -> String {
        self.status.to_owned()
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status.to_owned();
    }

    pub fn source_project_id(&self) -> String {
        self.source_project_id.to_owned()
    }

    pub fn set_source_project_id(&mut self, source_project_id: String) {
        self.source_project_id = source_project_id.to_owned();
    }

    pub fn project_ids(&self) -> Vec<String> {
        self.project_ids.clone()
    }

    pub fn set_project_ids(&mut self, project_ids: Vec<String>) {
        self.project_ids = project_ids.clone();
    }

    pub fn preview_file(&self) -> Option<String> {
        self.preview_file.to_owned()
    }

    pub fn set_preview_file(&mut self, preview_file: Option<String>) {
        self.preview_file = preview_file.to_owned();
    }

    pub fn preview_file_dataset_id(&self) -> Option<String> {
        self.preview_file_dataset_id.to_owned()
    }

    pub fn set_preview_file_dataset_id(&mut self, preview_file_dataset_id: Option<String>) {
        self.preview_file_dataset_id = preview_file_dataset_id.to_owned();
    }

    pub fn datasets(&self) -> Option<Vec<Dataset>> {
        self.datasets.clone()
    }

    pub fn set_datasets(&mut self, datasets: Option<Vec<Dataset>>) {
        self.datasets = datasets.clone();
    }

    pub fn metadata(&self) -> Option<Option<HashMap<String, String>>> {
        self.metadata.clone()
    }

    pub fn set_metadata(&mut self, metadata: Option<Option<HashMap<String, String>>>) {
        self.metadata = metadata.clone();
    }
}

/// Unity metadata property abstraction.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

/// Unity metadata field definition abstraction.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataDefinition {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub value_type: String,
}
