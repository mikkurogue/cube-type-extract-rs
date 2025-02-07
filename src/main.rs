use generator::{Generator, Metadata};

mod configuration;
mod generator;

fn main() {
    let mut generator = Generator {
        cube_count: 0,
        metadata: Some(Metadata { cubes: vec![] }),
    };

    let res = generator.fetch_metadata("http://localhost:4000/cubejs-api".to_string());

    println!("{:?}", generator.metadata);
}
