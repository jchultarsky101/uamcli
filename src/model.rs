use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    id: String,
    name: String,
}

impl Asset {
    pub fn new(id: String, name: String) -> Self {
        Asset { id, name }
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
}
