use configuration::Configuration;
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
    }

    let config = match configuration::read() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // make sure to use config.output on real implementatin
    println!("{:?}", config.output);

    let res = generator.fetch_metadata("http://localhost:4000/cubejs-api".to_string());

    generator.generate("./".to_string(), "cubejs-types".to_string(), true);
}
