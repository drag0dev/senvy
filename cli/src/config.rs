use std::ffi::OsString;
use serde_derive::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// full url of the senvy server
    remote_url: String,

    /// used when checking if there are new vars available
    last_version: u128,

    /// path to the current file
    path: Option<OsString>,
}

/// wrapper around write_config just to check if config already exists
// in case there is config present ask before overwriting
pub fn create_config(conf: Config) -> Result<()> {
    todo!();
}

pub fn write_config(conf: Config) -> Result<()> {
    todo!();
}

pub fn read_config() -> Result<Config> {
    todo!();
}
