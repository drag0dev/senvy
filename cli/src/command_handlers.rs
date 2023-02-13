use crate::{
    config::{Config, write_config},
    utils::{confirm, append_endpoint, get_vars}
};
use anyhow::{Result, Context};
use reqwest::StatusCode;
use senvy_common::types::Project;
use serde_json::to_string;
use std::time::Duration;

// makes a local config and an entry on the server
pub async fn init(conf: Option<Config>, name: String, file: String, remote_url: String) -> Result<()> {
    let mut proceed = true;

    // if config exists check if user wants to overwrite it
    if conf.is_some() {
        proceed = confirm("Config file already exists, do you want to continue?")?;
    }

    if proceed {
        // check if project already exists
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(5))
            .build()
            .context("building reqwest client")?;

        let endpoint = append_endpoint(&remote_url, "/exists")?;

        let res = client.get(endpoint)
            .body(name.clone())
            .send()
            .await
            .context("checking if entry with the given name exists on the server")?;

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
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(5))
        .build()
        .context("building reqwest client")?;

    // check if project exists
    let endpoint = append_endpoint(&remote_url, "/exists")?;
    let res = client.get(endpoint)
        .body(name.clone())
        .send()
        .await
        .context("checking if entry with the given name exists on the server")?;

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
    };
    let body_str = to_string(&body)
        .context("serializing project info")?;

    let endpoint = append_endpoint(&remote_url, "new")?;
    let res = client.post(endpoint)
        .body(body_str)
        .header("Content-Type", "application/json")
        .send()
        .await
        .context("creating entry on the server")?;

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

pub async fn delete(conf: Option<Config>, name: Option<String>, remote_url: Option<String>) -> Result<()> {
    todo!();
}

pub async fn pull(conf: Option<Config>, name: Option<String>, remote_url: Option<String>)  -> Result<()> {
    todo!();
}

pub async fn push(conf: Option<Config>, name: Option<String>, file: Option<String>, remote_url: Option<String>) -> Result<()> {
    todo!();
}

pub async fn check(conf: Option<Config>) -> Result<()> {
    todo!();
}
