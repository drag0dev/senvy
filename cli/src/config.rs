use std::{
    fs::{OpenOptions, remove_file},
    io::{Write, Read}
};
use senvy_common::types::ProjectEntry;
use serde_derive::{Serialize, Deserialize};
use anyhow::{Result, Context};
use serde_json::{from_str, to_vec_pretty};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    /// full url of the senvy server
    pub remote_url: String,

    /// used when checking if there are new vars available
    pub last_version: u128,

    /// path to the current file
    pub path: String,

    /// name of the current project
    pub name: String
}

/// writing config to ".senvy" in current working directory
pub fn write_config(conf: &Config) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(".senvy")
        .context("creating config file")?;

    let conf_str = to_vec_pretty(&conf)
        .context("serializing config into str json")?;

    file.write_all(&conf_str)
        .context("writing config to a file")?;

    Ok(())
}

pub fn read_config() -> Result<Option<Config>> {
    let file = OpenOptions::new()
        .read(true)
        .open(".senvy");

    if file.is_err() {
        let err = file.as_ref().err().unwrap();
        match err.kind() {
            std::io::ErrorKind::NotFound => return Ok(None),
            _ => {
                let file = file.context("reading config file");
                return Err(file.err().unwrap());
            }
        };
    }

    let mut file = file.unwrap();
    let mut buff: String = String::new();
    file.read_to_string(&mut buff)
        .context("reading config file")?;
    let data: Config = from_str(&buff)
        .context("deserializing date from file")?;

    Ok(Some(data))
}

pub fn delete_config() -> Result<()> {
    remove_file("./.senvy")
        .context("deleting config file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests both write and read at the same time
    #[test]
    fn write_and_read() {
        let conf = Config{
            remote_url: "https://remote-url.test".to_string(),
            last_version: 0,
            path: ".env".to_string(),
            name: "test".to_string()
        };

        write_config(&conf).unwrap();
        let read_conf = read_config().unwrap();

        assert_eq!(Some(conf), read_conf);
    }

    #[test]
    fn delete() {
        let conf = Config{
            remote_url: "https://remote-url.test".to_string(),
            last_version: 0,
            path: ".env".to_string(),
            name: "test".to_string()
        };

        write_config(&conf).unwrap();
        assert_eq!((), delete_config().unwrap());

        let read_conf = read_config().unwrap();
        assert_eq!(None, read_conf);
    }
}
