use std::collections::HashSet;

use colored::Colorize;
use serde::{Deserialize, Serialize};

pub struct Generator {
    pub cube_count: usize,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub cubes: Vec<Cube>,
}

#[derive(Debug, Deserialize)]
pub struct Cube {
    pub name: String,
    pub dimensions: Vec<String>,
    pub measures: Vec<String>,
}

pub struct Meta {
    pub extractable: bool,
}

pub struct FieldSet {
    pub name: String,
    pub meta: Meta,
}

impl Generator {
    pub fn fetch_metadata(&mut self, cube_url: String) {
        let resp = match fetch_cube_metadata(&cube_url) {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("{} {}", "Error fetching cube metadata: ".red(), err);
                std::process::exit(0);
            }
        };

        self.metadata = match serde_json::from_slice(&resp) {
            Ok(metadata) => Some(metadata),
            Err(err) => {
                eprintln!("{} {}", "Error parsing cube metadata: ".red(), err);
                std::process::exit(1);
            }
        };

        if let Some(metadata) = &self.metadata {
            self.cube_count = metadata.cubes.len();
        }
    }

    pub fn generate(&self, output_dir: String, file_name: String, skip_errors: bool) {
        // TODO:
    }
}

#[tokio::main]
async fn fetch_cube_metadata(cube_url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/v1/meta", cube_url);

    let body = client
        .get(&url)
        .send()
        .await?
        .error_for_status()? // check for http errors
        .bytes()
        .await?
        .to_vec();

    Ok(body)
}

fn extract_name(full_name: &str) -> String {
    "".to_string()
}

fn join_union_fields(items: Vec<String>) -> Vec<String> {
    let mut unique_items: HashSet<String> = HashSet::new();
    items.into_iter().for_each(|item| {
        unique_items.insert(item);
    });
    unique_items.into_iter().collect()
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map_or_else(String::new, |c| {
        c.to_uppercase().collect::<String>() + chars.as_str()
    })
}

fn contains_ignore_case(input: &str, sub_string: &str) -> bool {
    return input.to_lowercase().contains(&sub_string.to_lowercase());
}

fn skip(name: &str, check_case: &str) {
    if contains_ignore_case(name, check_case) {
        println!("{}", "Skipping cube due to ignore case match".yellow());
    }
}
