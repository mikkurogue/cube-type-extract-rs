use generator::{Generator, Metadata};

mod configuration;
mod generator;

fn main() {
    let generator = Generator {
        cube_count: 0,
        metadata: Some(Metadata { cubes: vec![] }),
    };

    println!("Hello, world!");
}
