use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use snafu::{ResultExt, Snafu};
use std::fs;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Could not read config file {path}"))]
    ReadConfigFile {
        source: std::io::Error,
        path: String,
    },
    #[snafu(display("error parsing config file"))]
    ParseConfigFile { source: serde_json::Error },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaserowConfig {
    token: String,
    pub databases: Vec<usize>,
}

impl BaserowConfig {
    pub fn new() -> Result<Self, Error> {
        let path = "./baserow_config.json";
        serde_json::from_str(&std::fs::read_to_string(&path).context(ReadConfigFileSnafu { path })?)
            .context(ParseConfigFileSnafu)
    }
}
