use generator::{Generator, Metadata};

mod configuration;
mod generator;

#[tokio::main]
async fn main() {
    let mut generator = Generator {
        cube_count: 0,
        metadata: Some(Metadata { cubes: vec![] }),
    };

    if !configuration::validate_configuration() {
        configuration::generate_default_config();
        std::process::exit(0);
    }

    let config = match configuration::read() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Await the async fetch_metadata method
    generator.fetch_metadata(config.cube_url).await;

    // Await the async generate method
    generator
        .generate(config.output, config.file_name, true)
        .await;
}
