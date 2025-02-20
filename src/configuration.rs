use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub cube_url: String,
    pub output: String,
    pub file_name: String,
    pub prefixes: Vec<Prefix>,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub fn validate_configuration() -> bool {
    std::fs::metadata("type-gen-config.json").is_ok()
}

pub fn read() -> Result<Configuration, std::io::Error> {
    let data = std::fs::read_to_string("type-gen-config.json")?;

    let config: Configuration = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(config)
}
