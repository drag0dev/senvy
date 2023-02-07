use std::{fs::OpenOptions, io::Write};
use anyhow::Result;
use serde_json::to_vec;
use crate::types::{Project, ProjectEntry};

// TODO: use atomicwrite for writing

/// prefix file names with "./data/"
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
        vars: project_info.vars
    };
    let data = to_vec(&data)?;

    // write
    file.write_all(&data)?;
    Ok(true)
}

/// reading already existing project
pub async fn read(project_name: &str) -> Result<()> {
    todo!();
}

/// updating already existing project
pub async fn update(timestamp: u128, project_info: Project) -> Result<()> {
    todo!();
}

/// delete already existing project
pub async fn delete(project_name: &str) -> Result<()> {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::types::Var;
    use super::*;

    #[actix_rt::test]
    async fn create_file() {
        let data = Project{
            name: "test-name".to_string(),
            vars: vec![
                Var{name: "port".to_string(), value: "8080".to_string()}
            ]};
        let res = create(123, data.clone()).await;

        assert_eq!(res.unwrap(), true);

        // already existing file
        let res = create(123, data).await;
        assert_eq!(res.unwrap(), false);
    }
}
