use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Error, ErrorKind},
};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub cube_url: String,
    pub output: String,
    pub file_name: String,
    pub prefixes: Vec<Prefix>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Prefix {
    pub name: String,
    pub prefix: String,
}

pub fn generate_default_config() {
    let default_config = Configuration {
        cube_url: "http://localhost:4000/cubejs-api".to_string(),
        output: "./".to_string(),
        file_name: "cubejs-types".to_string(),
        prefixes: vec![Prefix {
            name: "Placeholder".to_string(),
            prefix: "Main".to_string(),
        }],
    };

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

pub fn add_to_config(cube: &str, prefix: &str) -> Result<(), Error> {
    if !validate_configuration() {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Config not found or not supported",
        ));
    }

    let file_path = "type-gen-config.json"; // Adjust path if necessary

    // Open the config file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Deserialize JSON
    let mut config: Configuration = from_reader(reader)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse config"))?;

    // New prefix entry
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

    // Serialize back to JSON
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
