use std::{fs, io};
use std::path::Path;
use crate::baserow_config::BaserowConfig;
use snafu::{ResultExt, Snafu};
use std::process::exit;
use crate::generator::Generator;

mod baserow_config;
mod field_types;
mod generator;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Error obtaining configuration:\n {source}"))]
    Config { source: baserow_config::Error },
    #[snafu(display("Error creating target directory [{path}]: {source}"))]
    CreateTargetDir { source: io::Error, path: String },
    
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => exit(0),
        Err(e) => {
            eprint!("{}", e.to_string());
            exit(1);
        }
    };
}

async fn run() -> Result<(), Error> {
    let config = BaserowConfig::new().context(ConfigSnafu)?;
    let client = Generator::new(&config.token);

    fs::create_dir_all(&config.target_directory).context(CreateTargetDirSnafu {path: &config.target_directory.to_string()})?;
    let target_path = Path::new(&config.target_directory);
    
        client.generate_structs(&config.databases, target_path).await;
    Ok(())
}

