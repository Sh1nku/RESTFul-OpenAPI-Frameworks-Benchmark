use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameworkConfig {
    pub name: String,
    pub language: String,
    pub url: String,
    pub image: String,
    pub dockerfile: String,
    pub color: String,
}
pub fn get_framework_configs() -> Result<HashMap<String, FrameworkConfig>, Box<dyn error::Error>> {
    Ok(serde_yaml::from_str(&read_to_string(Path::new(
        "frameworks.yml",
    ))?)?)
}
