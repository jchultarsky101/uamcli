use keyring::Entry;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("keyring error")]
    CannotAccessKeyringEntity(#[from] keyring::Error),
}

pub struct Keyring<'a> {
    application: &'a str,
    project: &'a str,
}

impl<'a> Keyring<'a> {
    pub fn new(application: &'a str, project: &'a str) -> Keyring<'a> {
        Keyring {
            application,
            project,
        }
    }

    fn compose_key(&self, key: &str) -> String {
        format!(
            "{application}:{project}:{key}",
            application = self.application,
            project = self.project,
            key = key
        )
    }

    fn get_entry(&self, key: &str) -> Result<Entry, KeyringError> {
        Ok(Entry::new(self.application, key)?)
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, KeyringError> {
        match self
            .get_entry(self.compose_key(key).as_str())?
            .get_password()
        {
            Ok(value) => Ok(Some(value)),
            Err(e) => match e {
                keyring::Error::NoEntry => Ok(None),
                _ => Err(KeyringError::from(e)),
            },
        }
    }

    pub fn put(&self, key: &str, value: &str) -> Result<(), KeyringError> {
        self.get_entry(self.compose_key(key).as_str())?
            .set_password(value)?;
        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<(), KeyringError> {
        self.get_entry(self.compose_key(key).as_str())?
            .delete_password()?;
        Ok(())
    }
}
