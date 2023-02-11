use crate::{
    config::Config,
    args_structure::Commands,
};
use anyhow::Result;

pub fn init(conf: Option<Config>, name: String, file: String, remote_url: String) {}

pub fn new(conf: Option<Config>, name: String, file: String, remote_url: String) {}

pub fn delete(conf: Option<Config>, name: Option<String>, remote_url: Option<String>) {}

pub fn pull(conf: Option<Config>, name: Option<String>, remote_url: Option<String>) {}

pub fn push(conf: Option<Config>, name: Option<String>, file: Option<String>, remote_url: Option<String>) {}

pub fn check(conf: Option<Config>) {}
