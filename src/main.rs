use clap::{Parser, Subcommand};
use generator::{Generator, Metadata};

mod configuration;
mod generator;

#[derive(Parser, Debug)]
#[command(name = "main")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new cube with a prefix
    New { cube_name: String, prefix: String },

    /// Delete an entry
    Remove { cube_name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New { cube_name, prefix }) => {
            match configuration::add_to_config(cube_name, prefix) {
                Ok(_) => {
                    println!("Config updated successfully!");
                }
                Err(e) => {
                    eprintln!("Failed to update config: {}", e);
                }
            }
            return;
        }
        Some(Commands::Remove { cube_name }) => {
            println!("NYI: need to remove cube {}", cube_name);
            return;
        }
        None => {}
    }

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
