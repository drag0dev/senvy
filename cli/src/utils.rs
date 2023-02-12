use std::{
    io::{stdin, stdout, Write, Read},
    time::{UNIX_EPOCH, SystemTime},
    fs::OpenOptions
};
use anyhow::{Result, Context, anyhow};
use dotenv_parser::parse_dotenv;
use senvy_common::types::Var;
use url::Url;

/// confirm with user via stdio
pub fn confirm(msg: &str) -> Result<bool> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buff: String = String::new();
    loop {
        print!("{} (y/n): ", msg);
        stdout.flush().context("flushing message to the stdout")?;

        buff.clear();
        stdin.read_line(&mut buff).context("reading user input")?;

        if buff.len() == 0 {
            continue;
        }else {
            buff = buff.to_uppercase();
            if buff.chars().nth(0).unwrap() == 'N' {
                return Ok(false);
            }else {
                return Ok(true);
            }
        }
    }
}

/// append endpoint to a given url
pub fn append_endpoint(url: &String, endpoint: &str) -> Result<String> {
    let mut parsed_url = Url::parse(&url)
        .context("parsing remote url")?;
    parsed_url.set_path(endpoint);
    Ok(parsed_url.as_str().to_string())
}

/// get current timestamp in nanos
pub fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // safe to just unwrap beacuse UNIX_EPOCH is passed
        .as_nanos()
}

/// given the file path to the dot file, parse vars
pub fn get_vars(file: &str) -> Result<Vec<Var>> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(file)
        .context("opening env var file")?;

    let mut lines = String::new();
    file.read_to_string(&mut lines)
        .context("reading vars")?;

    let mut vars: Vec<Var> = Vec::new();
    let vars_map = parse_dotenv(&lines)
        .map_err(|e| anyhow!(e))
        .context("parsing env vars")?;

    for (key, value) in vars_map.into_iter() {
        vars.push(Var{name: key, value});
    }

    Ok(vars)
}
