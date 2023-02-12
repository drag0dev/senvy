use std::io::{stdin, stdout, Write};
use anyhow::{Result, Context};
use url::Url;

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

pub fn append_endpoint(url: &String, endpoint: &str) -> Result<String> {
    let mut parsed_url = Url::parse(&url)
        .context("reading")?;
    parsed_url.set_path(endpoint);
    Ok(parsed_url.as_str().to_string())
}
