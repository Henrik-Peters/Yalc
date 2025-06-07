use std::fs::{File, metadata};
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;

use crate::config::toml_parser;
use crate::constants::{DEFAULT_CONFIG_CONTENT, DEFAULT_CONFIG_PATH};

/// This command is called via "yalc config init".
/// This will create a new default config file.
/// Will result in an error if a config file already exists.
pub fn execute_init_config_command() -> Result<(), io::Error> {
    let path = Path::new(DEFAULT_CONFIG_PATH);

    //First check if the file already exists
    if metadata(path).is_ok() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Config file already exists",
        ));
    }

    //Create new config file
    create_default_config_file(path)
}

fn create_default_config_file(path: &Path) -> Result<(), io::Error> {
    println!("Creating new template config file at: {}", path.display());

    //Create new file handle
    let mut file = File::create(&path)?;

    let content = DEFAULT_CONFIG_CONTENT;
    file.write_all(content.as_bytes())?;

    //Log the successful write operation
    println!("Successfully written template config file content");
    Ok(())
}

pub fn execute_check_config_command() -> Result<(), io::Error> {
    let path = Path::new(DEFAULT_CONFIG_PATH);

    //The config is validated by the load function
    let _ = toml_parser::load_config(&path)?;

    //Final print that the config is valid
    println!("Yalc config check: [VALID]");
    Ok(())
}
