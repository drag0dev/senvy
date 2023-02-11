use std::{fs::OpenOptions, io::{Write, stdin, stdout, Read}};
use serde_derive::{Serialize, Deserialize};
use anyhow::{Result, Context};
use serde_json::{from_str, to_vec_pretty};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    /// full url of the senvy server
    remote_url: String,

    /// used when checking if there are new vars available
    last_version: u128,

    /// path to the current file
    path: String,

    /// name of the current project
    name: String
}

/// wrapper around write_config just to check if config already exists
/// true if config file was written, otherwise false
// in case there is config present ask before overwriting
pub fn create_config(conf: &Config) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(".senvy");

    let mut found = true;
    if file.is_err() {
        if file.as_ref().err().unwrap().kind() == std::io::ErrorKind::NotFound {
            found = false;
        }else {
            file.context("opening config file")?;
        }
    }

    // if config file already exist check if user wants to overwrite it
    if found {
        let stdin = stdin();
        let mut stdout = stdout();
        let mut buff: String = String::new();
        loop {
            print!("Config file already exists, do you want to continue (y/n): ");
            stdout.flush().context("flushing message to the stdout")?;

            buff.clear();
            stdin.read_line(&mut buff).context("reading user input")?;

            if buff.len() == 0 {
                continue;
            }else {
                buff = buff.to_uppercase();

                // if not, just return
                if buff.chars().nth(0).unwrap() == 'N' {
                    println!("Not writing config file");
                    return Ok(());

                // if yes write config
                }else {
                    break;
                }
            }
        }
    }

    write_config(&conf).context("writing config")?;
    Ok(())
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

    // create_config has user input thus manually tested
}
