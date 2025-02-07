use generator::{Generator, Metadata};

mod configuration;
mod generator;

fn main() {
    let mut generator = Generator {
        cube_count: 0,
        metadata: Some(Metadata { cubes: vec![] }),
    };

    if !configuration::validate_configuration() {
        configuration::generate_default_config();
        // Exit the program after generating the default configuration
        std::process::exit(0);
    }

    let config = match configuration::read() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let _ = generator.fetch_metadata(config.cube_url);

    generator.generate(config.output, config.file_name, true);
}
