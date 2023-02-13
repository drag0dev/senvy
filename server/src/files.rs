use std::{
    fs::{OpenOptions, remove_file},
    io::{Write, Read}
};
use anyhow::Result;
use serde_json::{
    to_vec, from_str
};
use senvy_common::types::{Project, ProjectEntry};

// TODO: use atomicwrite for writing
// TODO: use tokio async read

/// prefix file names with "data/"
macro_rules! path_prefix {
    ( $x:expr ) => {
        {
            format!("data/{}", $x)
        }
    };
}

/// initializing a new project
/// err indicates fs or json error
/// false means that file already exists
pub async fn create(timestamp: u128, project_info: Project) -> Result<bool> {
    // opening file with create_new option will fail if it already exists
    let path = path_prefix!(project_info.name);
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path);

    if file.is_err() {
        let err = file.err().unwrap();
        match err.kind() {
            std::io::ErrorKind::AlreadyExists => return Ok(false),
            _ => return Err(err.into()),
        }
    }
    let mut file = file.unwrap();

    // serialize
    let data = ProjectEntry {
        timestamp,
        vars: project_info.vars,
        path: project_info.path,
    };
    let data = to_vec(&data)?;

    // write
    file.write_all(&data)?;
    Ok(true)
}

/// reading already existing project
/// err indicates fs error, json error should not happen
/// None means that file doesn't exist
pub async fn read(project_name: &str) -> Result<Option<ProjectEntry>> {
    let path = path_prefix!(project_name);
    let file = OpenOptions::new()
        .read(true)
        .open(path);
    if file.is_err() {
        let err = file.err().unwrap();
        match err.kind() {
            std::io::ErrorKind::NotFound => return Ok(None),
            _ => return Err(err.into()),
        }
    }

    let mut file = file.unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    let data: ProjectEntry = from_str(&buff)?;
    Ok(Some(data))
}

/// updating already existing project
/// err indicates fs or json error
/// false means that file doesn't exist
pub async fn update(timestamp: u128, project_info: Project) -> Result<bool> {
    let path = path_prefix!(project_info.name);
    let file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(path);

    if file.is_err() {
        let err = file.err().unwrap();
        match err.kind() {
            std::io::ErrorKind::NotFound => return Ok(false),
            _ => return Err(err.into()),
        }
    }

    let mut file = file.unwrap();
    let data = ProjectEntry{
        timestamp,
        vars: project_info.vars,
        path: project_info.path
    };
    let data = to_vec(&data)?;
    file.write_all(&data)?;
    Ok(true)
}

/// delete already existing project
/// err indicates fs error
/// false means that it doesn't exit
pub async fn delete(project_name: &str) -> Result<bool> {
    let path = path_prefix!(project_name);
    let res = remove_file(path);
    if res.is_err() {
        let err = res.err().unwrap();
        match err.kind() {
            std::io::ErrorKind::NotFound => return Ok(false),
            _ => return Err(err.into()),
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use senvy_common::types::Var;
    use super::*;

    #[actix_rt::test]
    async fn create_file() {
        let data = Project{
            name: "test-name".to_string(),
            vars: vec![
                Var{name: "port".to_string(), value: "8080".to_string()}
            ],
            path: "./.env".to_string()
        };
        let res = create(123, data.clone()).await;

        // file that doesn't exist
        assert_eq!(res.unwrap(), true);

        // creating already existing file
        let res = create(123, data).await;
        assert_eq!(res.unwrap(), false);
    }

    #[actix_rt::test]
    async fn read_file() {
        let data = Project{
            name: "test-read".to_string(),
            vars: vec![
                Var{name: "port".to_string(), value: "8080".to_string()}
            ],
            path: "./.env".to_string()
        };
        _ = create(123, data.clone()).await.unwrap();

        let data = ProjectEntry{
            timestamp: 123,
            vars: data.vars,
            path: "./.env".to_string()
        };

        // reading an existing file
        let res = read("test-read").await;
        assert_eq!(Some(data), res.unwrap());

        // reading file that doesn't exist
        let res = read("test-read-not-existing").await;
        assert_eq!(None, res.unwrap());
    }

    #[actix_rt::test]
    async fn update_file() {
        let mut data = Project{
            name: "test-update".to_string(),
            vars: vec![
                Var{name: "port".to_string(), value: "8080".to_string()}
            ],
            path: "./.env".to_string()
        };
        _ = create(123, data.clone()).await.unwrap();

        // updating existing file
        data.vars.push(Var{name: "new-var".to_string(), value: "new".to_string()});
        let res = update(125, data.clone()).await.unwrap();
        assert_eq!(res, true);

        // checking if data was updated
        let read_data = read("test-update").await.unwrap();
        let expected_data = ProjectEntry{
            timestamp: 125,
            vars: data.vars.clone(),
            path: "./.env".to_string()
        };
        assert_eq!(Some(expected_data), read_data);

        // updating file that doesn't exist
        data.name = "test-update-wrong-name".to_string();
        let res = update(125, data).await.unwrap();
        assert_eq!(false, res);
    }

    #[actix_rt::test]
    async fn delete_file() {
        // deleting file that doesn't exist
        let res = delete("test-delete").await.unwrap();
        assert_eq!(false, res);

        let data = Project{
            name: "test-delete".to_string(),
            vars: vec![
                Var{name: "port".to_string(), value: "8080".to_string()}
            ],
            path: "./.env".to_string()
        };
        _ = create(123, data.clone()).await.unwrap();

        // deleting file that does exist
        let res = delete("test-delete").await.unwrap();
        assert_eq!(true, res);
    }
}
