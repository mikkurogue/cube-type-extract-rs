use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub cube_url: String,
    pub output: String,
    pub file_name: String,
    pub prefixes: Vec<Prefix>,
    pub ignore_error_cube: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Prefix {
    pub name: String,
    pub prefix: String,
}

pub fn generate_default_config() {
    let default_config = Configuration {
        cube_url: "http://localhost:4000/cubejs.api".to_owned(),
        output: "./".to_owned(),
        file_name: "cubejs-types".to_owned(),
        prefixes: vec![Prefix {
            name: "Placeholder".to_owned(),
            prefix: "Main".to_owned(),
        }],
        ignore_error_cube: true,
    };

    let json_data = match serde_json::to_string_pretty(&default_config) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("{}, {}", "Could not marshal config file: ".red(), err);
            std::process::exit(1);
        }
    };

    let filename = "type-gen-config.json";

    if let Err(err) = std::fs::write(filename, json_data) {
        eprintln!("{} {}", "Could not write to file: ".red(), err);
        std::process::exit(1);
    }

    println!(
        "{}",
        "Successfully created the default configuration file!".green()
    );
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
