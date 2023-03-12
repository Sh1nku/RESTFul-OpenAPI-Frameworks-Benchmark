use std::collections::HashMap;
use std::error;
use std::fs::read_to_string;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameworkConfig {
    name: String,
    language: String,
    url: String,
    image: String,
    dockerfile: String,
    color: String
}

pub fn get_framework_configs() -> Result<HashMap<String, FrameworkConfig>, Box<dyn error::Error>> {
    Ok(serde_yaml::from_str(&read_to_string(Path::new("frameworks.yml"))?)?)
}