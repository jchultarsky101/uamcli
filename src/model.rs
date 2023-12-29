use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    id: String,
    name: String,
    version: String,
    tags: Vec<String>,
    system_tags: Vec<String>,
    labels: Vec<String>,
    primary_type: String,
    status: String,
    frozen: bool,
    source_project_id: String,
    project_ids: Vec<String>,
    preview_file: Option<String>,
    preview_file_dataset_id: Option<String>,
}

impl Asset {
    pub fn new(
        id: String,
        name: String,
        version: String,
        tags: Vec<String>,
        system_tags: Vec<String>,
        labels: Vec<String>,
        primary_type: String,
        status: String,
        frozen: bool,
        source_project_id: String,
        project_ids: Vec<String>,
        preview_file: Option<String>,
        preview_file_dataset_id: Option<String>,
    ) -> Self {
        Asset {
            id,
            name,
            version,
            tags,
            system_tags,
            labels,
            primary_type,
            status,
            frozen,
            source_project_id,
            project_ids,
            preview_file,
            preview_file_dataset_id,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_owned()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id.to_string();
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.to_owned();
    }

    pub fn version(&self) -> String {
        self.version.to_owned()
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version.to_owned();
    }

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn set_tabs(&mut self, tags: Vec<String>) {
        self.tags = tags.clone();
    }

    pub fn system_tags(&self) -> Vec<String> {
        self.system_tags.clone()
    }

    pub fn set_system_tags(&mut self, system_tags: Vec<String>) {
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

    pub fn frozen(&self) -> bool {
        self.frozen
    }

    pub fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen
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

    pub fn preview_file_dataset_if(&self) -> Option<String> {
        self.preview_file_dataset_id.to_owned()
    }

    pub fn set_preview_file_dataset_id(&mut self, preview_file_dataset_id: Option<String>) {
        self.preview_file_dataset_id = preview_file_dataset_id.to_owned();
    }
}
