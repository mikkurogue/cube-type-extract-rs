pub struct Configuration {
    pub cube_url: String,
    pub output: String,
    pub file_name: String,
    pub prefix_list: Vec<Prefix>,
    pub ignore_error_cubes: bool,
}

pub struct Prefix {
    pub name: String,
    pub prefix: String,
}

pub fn generate_default_config() {
    // TODO
}

pub fn validate_configuration() -> bool {
    std::fs::metadata("type-gen-config.json").is_ok()
}

pub fn read() -> Result<Configuration, std::io::Error> {
    // TODO: read the file and parse it here.

    Ok(Configuration {
        cube_url: "".to_owned(),
        output: "".to_owned(),
        file_name: "".to_owned(),
        prefix_list: vec![Prefix {
            name: "".to_owned(),
            prefix: "".to_owned(),
        }],
        ignore_error_cubes: true,
    })
}
