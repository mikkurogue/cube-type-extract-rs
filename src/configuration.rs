use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Error, ErrorKind},
};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_reader, to_writer_pretty};

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub link: String,
    pub headers: Option<Map<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub url: Url,
    pub output: String,
    pub file_name: String,
    pub prefixes: Vec<Prefix>,
    pub enable_count_types: bool,
    pub enable_check_existence_fields: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Prefix {
    pub name: String,
    pub prefix: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            url: Url {
                link: "http://localhost:4000/cubejs-api".to_string(),
                headers: None,
            },
            output: "./".to_string(),
            file_name: "cubejs-types".to_string(),
            prefixes: vec![Prefix {
                name: "Placeholder".to_string(),
                prefix: "Main".to_string(),
            }],
            enable_count_types: true,
            enable_check_existence_fields: true,
        }
    }
}

pub fn generate_default_config() {
    let default_config = Configuration::default();

    let json_data = match serde_json::to_string_pretty(&default_config) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("{}, {}", "Could not marshal config file: ".red(), err);
            std::process::exit(1);
        }
    };

    if let Err(err) = std::fs::write("type-gen-config.json", json_data) {
        eprintln!("{} {}", "Could not write to file: ".red(), err);
        std::process::exit(1);
    }

    println!(
        "{}",
        "Successfully created the default configuration file! Apply your necessary changes and run again".green()
    );
}

pub fn remove_from_config(cube: &str) -> Result<(), Error> {
    if !validate_configuration() {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Config not found or not supported",
        ));
    }

    let file_path = "type-gen-config.json";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut config: Configuration = from_reader(reader)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse config"))?;

    if let Some(index) = config.prefixes.iter().position(|p| p.name == cube) {
        config.prefixes.remove(index);
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Cube not found in config"));
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let writer = BufWriter::new(file);

    to_writer_pretty(writer, &config)?;

    Ok(())
}

pub fn add_to_config(cube: &str, prefix: &str) -> Result<(), Error> {
    if !validate_configuration() {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Config not found or not supported",
        ));
    }

    let file_path = "type-gen-config.json";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut config: Configuration = from_reader(reader)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse config"))?;

    let new_prefix = Prefix {
        name: cube.to_string(),
        prefix: prefix.to_string(),
    };

    if config.prefixes.iter().any(|p| *p == new_prefix) {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Cube and Prefix combination already exists",
        ));
    }

    config.prefixes.push(new_prefix);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let writer = BufWriter::new(file);

    to_writer_pretty(writer, &config)?;

    println!(
        "Added cube {} and mapped to prefix {} to the config.",
        cube.to_string(),
        prefix.to_string()
    );

    Ok(())
}
pub fn validate_configuration() -> bool {
    std::fs::metadata("type-gen-config.json").is_ok()
}

pub fn read() -> Result<Configuration, std::io::Error> {
    let data = std::fs::read_to_string("type-gen-config.json")?;

    let config: Configuration = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(config)
}
