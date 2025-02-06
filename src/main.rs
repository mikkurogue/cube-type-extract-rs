use generator::{Generator, Metadata};

mod configuration;
mod generator;

fn main() {
    let generator = Generator {
        cube_count: 5,
        metadata: Metadata { cube: vec![] },
    };

    println!("Hello, world!");
}
