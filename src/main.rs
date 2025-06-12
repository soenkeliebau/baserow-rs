use crate::baserow_config::BaserowConfig;
use snafu::{ResultExt, Snafu};
use std::process::exit;

mod baserow_config;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Error obtaining configuration:\n {source}"))]
    Config { source: baserow_config::Error },
}

fn main() {
    match run() {
        Ok(_) => exit(0),
        Err(e) => {
            eprint!("{}", e.to_string());
            exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    let config = BaserowConfig::new().context(ConfigSnafu)?;
    println!("Importing from these databases: {:?}", config.databases);
    Ok(())
}
