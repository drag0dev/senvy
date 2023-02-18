use crate::{
    config::{Config, write_config, delete_config},
    utils::{confirm, append_endpoint, get_vars, write_env}
};
use anyhow::{Result, Context, anyhow};
use reqwest::StatusCode;
use senvy_common::types::{Project, ProjectEntry};
use serde_json::{to_string, from_str};
use std::time::Duration;

macro_rules! make_client{
    () => {
        reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(5))
            .build()
            .context("building reqwest client")?
    };
}

// makes a local config and an entry on the server
pub async fn init(conf: Option<Config>, name: String, file: String, remote_url: String) -> Result<()> {
    let mut proceed = true;

    // if config exists check if user wants to overwrite it
    if conf.is_some() {
        proceed = confirm("Config file already exists, do you want to continue?")?;
    }

    if proceed {
        // check if project already exists
        let client = make_client!();
        let endpoint = append_endpoint(&remote_url, "/exists")?;
        let res = client.get(endpoint)
            .body(name.clone())
            .send()
            .await
            .context("checking if entry with the given name exists on the server")?;

        // check the results
        let res_status = res.status();
        let res_body = res.text()
            .await
            .context("reading response body")?;

        match res_status {
            StatusCode::OK => {
                if &res_body == "true" {
                    println!("Project entry with name \"{}\" already exists, if you want to overwrite it first delete project entry on the server", &name);
                    return Ok(());
                }
            },
            _ => {
                println!("Unexpected response from the server, server response: {}", res_body);
                return Ok(());
            },
        }

        // parse vars from the file
        let vars = get_vars(&file)?;

        // body for creating a new entry
        let body = Project{
            name: name.clone(),
            vars,
            path: file.clone(),
        };
        let body_str = to_string(&body)
            .context("serializing project info")?;

        // push to the server
        let res = client.post(append_endpoint(&remote_url, "new")?)
            .body(body_str)
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("creating entry on the server")?;

        let res_status = res.status();
        let res_body = res.text()
            .await
            .context("reading response body")?;

        // if creation of the entry on the server was successfull get back the timestamp
        let timestamp;
        match res_status {
            StatusCode::OK => {
                println!("Successfully created entry on the server");
                timestamp = res_body.parse::<u128>()
                    .context("parsing timestamp returned from server")?;
            },
            // it is possible that someone made an entry on the server since we checked
            StatusCode::BAD_REQUEST => {
                println!("Error making a new entry, server response: {}", res_body);
                return Ok(());
            },
            _ => {
                println!("Unexpected response from the server, server response: {}", res_body);
                return Ok(());
            },
        }

        // write config to the file
        let conf = Config{
            remote_url,
            last_version: timestamp,
            path: file,
            name,
        };

        write_config(&conf)?;
        println!("Successfully made local config file");
    }

    Ok(())
}

// new does not update local config, just makes a new entry on the server
pub async fn new(_: Option<Config>, name: String, file: String, remote_url: String) -> Result<()> {
    let client = make_client!();

    // check if project exists
    let endpoint = append_endpoint(&remote_url, "/exists")?;
    let res = client.get(endpoint)
        .body(name.clone())
        .send()
        .await
        .context("checking if entry with the given name exists on the server")?;

    // check the results
    let res_status = res.status();
    let res_body = res.text()
        .await
        .context("reading response body")?;

    match res_status {
        StatusCode::OK => {
            if &res_body == "true" {
                println!("Project entry with name \"{}\" already exists, if you want to overwrite it first delete project entry on the server", &name);
                return Ok(());
            }
        },
        _ => {
            println!("Unexpected response from the server, server response: {}", res_body);
            return Ok(());
        },
    }

    // parse vars from the file
    let vars = get_vars(&file)?;

    // body for creating a new entry
    let body = Project{
        name,
        vars,
        path: file
    };
    let body_str = to_string(&body)
        .context("serializing project info")?;

    // create a new entry
    let endpoint = append_endpoint(&remote_url, "new")?;
    let res = client.post(endpoint)
        .body(body_str)
        .header("Content-Type", "application/json")
        .send()
        .await
        .context("creating entry on the server")?;

    // check the results
    let res_status = res.status();
    let res_body = res.text()
        .await
        .context("reading response body")?;
    match res_status {
        StatusCode::OK =>
            println!("Successfully created entry on the server"),
        // it is possible that someone made an entry on the server since we checked
        StatusCode::BAD_REQUEST => {
            println!("Error making a new entry, server response: {}", res_body);
            return Ok(());
        },
        _ => {
            println!("Unexpected response from the server, server response: {}", res_body);
            return Ok(());
        },
    }

    Ok(())
}

// delete entry on the server
// only delete local if user confirms
// making a delete request based on combination of name and remote_url and config
pub async fn delete(conf: Option<Config>, name: Option<String>, remote_url: Option<String>) -> Result<()> {
    if (name.is_none() || remote_url.is_none()) && conf.is_none() {
        let err = anyhow!("name and remote url are both required when there is no local config")
            .context("gathering information about project");
        return Err(err);
    }

    // take both provided information and information from config
    let conf = conf.unwrap();
    let name = if name.is_some() {
        name.unwrap()
    } else {
        conf.name
    };
    let remote_url = if remote_url.is_some() {
        remote_url.unwrap()
    } else {
        conf.remote_url
    };

    // send delete request
    let client = make_client!();
    let endpoint = append_endpoint(&remote_url, "delete")?;
    let res = client.delete(endpoint)
        .body(name)
        .send()
        .await
        .context("deleting project entry on the server")?;

    // check the results
    let res_status = res.status();
    let res_body = res.text()
        .await
        .context("reading response body")?;
    match res_status {
        StatusCode::OK => println!("Successfully deleted project entry from the server"),
        StatusCode::BAD_REQUEST => {
            println!("Error deleting entry, server response: {}", res_body);
            return Ok(());
        },
        _ => {
            println!("Unexpected response from the server, server response: {}", res_body);
            return Ok(());
        },
    }

    // deleting local config
    let proceed = confirm("Do you want to delete local config?")?;

    if proceed {
        delete_config()?;
        println!("Successfully deleted local config");
    }

    Ok(())
}

// pull entry from the server
// confirm overwriting with user
pub async fn pull(conf: Option<Config>, name: Option<String>, remote_url: Option<String>)  -> Result<()> {
    if (name.is_none() || remote_url.is_none()) && conf.is_none() {
        let err = anyhow!("name and remote url are both required when there is no local config")
            .context("gathering information about project");
        return Err(err);
    }

    // take both provided information and information from config
    let name = if name.is_some() {
        name.unwrap()
    } else {
        conf.as_ref().unwrap().name.to_owned()
    };
    let remote_url = if remote_url.is_some() {
        remote_url.unwrap()
    } else {
        conf.as_ref().unwrap().remote_url.to_owned()
    };

    // send read request
    let client = make_client!();
    let endpoint = append_endpoint(&remote_url, "read")?;
    let res = client.get(endpoint)
        .body(name.clone())
        .send()
        .await
        .context("pulling entry from the server")?;

    // check the results
    let res_status = res.status();
    let res_body = res.text()
        .await
        .context("reading response body")?;

    match res_status {
        StatusCode::OK => {},
        StatusCode::BAD_REQUEST => {
            println!("Error getting entry from the server: {}", res_body);
            return Ok(());
        },
        _ => {
                println!("Unexpected response from the server, server response: {}", res_body);
                return Ok(());
        },
    }

    if conf.is_some() {
        let proceed = confirm("Local config already exists, do you want to overwrite it?")?;
        if !proceed {
            return Ok(());
        }
    }

    // write a new config file
    let entry: ProjectEntry = from_str(&res_body)
        .context("deserializing config")?;

    let config = Config{
        remote_url,
        last_version: entry.timestamp,
        name,
        path: entry.path.clone()
    };

    write_config(&config)?;
    println!("Successfully updated local config");

    write_env(entry)?;
    println!("Successfully updated local var file");

    Ok(())
}

pub async fn push(conf: Option<Config>, name: Option<String>, file: Option<String>, remote_url: Option<String>) -> Result<()> {
    if (file.is_none() || remote_url.is_none()) && conf.is_none() {
        let err = anyhow!("name and remote url are both required when there is no local config")
            .context("gathering information about project");
        return Err(err);
    }

    // take both provided information and information from config
    let name = if name.is_some() {
        name.unwrap()
    } else {
        conf.as_ref().unwrap().name.to_owned()
    };
    let remote_url = if remote_url.is_some() {
        remote_url.unwrap()
    } else {
        conf.as_ref().unwrap().remote_url.to_owned()
    };

    let file = if file.is_some() {
        file.unwrap()
    } else {
        conf.as_ref().unwrap().path.to_owned()
    };

    let vars = get_vars(&file)?;
    let body = Project{
        name,
        path: file,
        vars
    };
    let body_str = to_string(&body)
        .context("serializing project info")?;

    // send the update request
    let client = make_client!();
    let endpoint = append_endpoint(&remote_url, "update")?;
    let res = client.post(endpoint)
        .body(body_str)
        .header("Content-Type", "application/json")
        .send()
        .await
        .context("updating entry on the server")?;

    let res_status = res.status();
    let res_body = res.text()
        .await
        .context("reading response body")?;
    match res_status {
        StatusCode::OK => println!("Successfully updated entry on the server"),
        StatusCode::BAD_REQUEST => println!("Error making a new entry, server response: {}", res_body),
        _ => println!("Unexpected response from the server, server response: {}", res_body),
    }
    Ok(())
}

// check if there is a new version by puling entry for the current project
pub async fn check(conf: Option<Config>) -> Result<()> {
    if conf.is_none() {
        println!("Senvy is not initialized in the current directory");
        return Ok(());
    }
    let conf = conf.unwrap();

    // send the read request
    let client = make_client!();
    let endpoint = append_endpoint(&conf.remote_url, "read")?;
    let res = client.get(endpoint)
        .body(conf.name.clone())
        .send()
        .await
        .context("checking entry on the server")?;

    // check the results
    let res_status = res.status();
    let res_body = res.text()
        .await
        .context("reading response body")?;

    match res_status {
        StatusCode::OK => {},
        StatusCode::BAD_REQUEST => {
            println!("Error getting entry from the server: {}", res_body);
            return Ok(());
        },
        _ => {
                println!("Unexpected response from the server, server response: {}", res_body);
                return Ok(());
        },
    }

    let new_conf: ProjectEntry = from_str(&res_body)
        .context("deserializing config")?;

    if new_conf.timestamp > conf.last_version {
        println!("New version avaiable");
        let proceed = confirm("Do you want to update local config?")?;
        if proceed {
            let new_conf = Config {
                remote_url: conf.remote_url,
                last_version: new_conf.timestamp,
                path: new_conf.path,
                name: conf.name,
            };
            write_config(&new_conf)?;
            println!("Successfully updated local config");
        } else {
            println!("Not updating local config");
        }
    } else {
        println!("Everything is up to date")
    }

    Ok(())
}
