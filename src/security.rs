//! Implements methods to interface with the underlying operating system
//! secret vault. Used to store sensitive information, such as the Unity Key Secret.
use keyring::Entry;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("keyring error")]
    CannotAccessKeyringEntity(#[from] keyring::Error),
}

/// A wrapper for all security errors.
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

    /// Helper method to format the key to store data in the vault.
    ///
    /// Parameters:
    ///
    /// * key: the key
    fn compose_key(&self, key: &str) -> String {
        format!(
            "{application}:{project}:{key}",
            application = self.application,
            project = self.project,
            key = key
        )
    }

    /// Returns data for a key.
    ///
    /// Parameters:
    ///
    /// * key: the key
    fn get_entry(&self, key: &str) -> Result<Entry, KeyringError> {
        Ok(Entry::new(self.application, key)?)
    }

    /// Returns the value for a specific key. If no such key exists, it returns None.
    ///
    /// Parameters:
    ///
    /// * key: the key value
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

    /// Updates the value for a given key. If no such key exists, a new one will be created first.
    ///
    /// Parameters:
    ///
    /// * key: the key
    /// * value: the value for tis key
    pub fn put(&self, key: &str, value: &str) -> Result<(), KeyringError> {
        self.get_entry(self.compose_key(key).as_str())?
            .set_password(value)?;
        Ok(())
    }

    /// Deletes a key and its value from the vault.
    ///
    /// Parameters:
    ///
    /// * key: the key
    pub fn delete(&self, key: &str) -> Result<(), KeyringError> {
        self.get_entry(self.compose_key(key).as_str())?
            .delete_password()?;
        Ok(())
    }
}
