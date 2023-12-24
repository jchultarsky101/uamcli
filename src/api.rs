use std::cell::RefCell;

use crate::configuration::Configuration;

pub struct Api {
    configuration: RefCell<Configuration>,
}

impl Api {
    pub fn new(configuration: &RefCell<Configuration>) -> Api {
        Api {
            configuration: configuration.clone(),
        }
    }

    pub fn configuration(&self) -> RefCell<Configuration> {
        self.configuration.clone()
    }
}
