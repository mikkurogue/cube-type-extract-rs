use std::collections::HashSet;

use colored::Colorize;

pub struct Generator {
    pub cube_count: usize,
    pub metadata: Metadata,
}

pub struct Metadata {
    pub cube: Vec<Cube>,
}

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
    pub fn fetch_metadata(&self, cube_url: String) {
        // TODO:
    }

    pub fn generate(&self, output_dir: String, file_name: String, skip_errors: bool) {
        // TODO:
    }
}

fn fetch_cube_metadata(cube_url: String) -> Result<Vec<u8>, std::io::Error> {
    // TODO: implement func
    Ok(vec![])
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
