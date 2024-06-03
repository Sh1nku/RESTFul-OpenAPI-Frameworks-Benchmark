use serde::{Deserialize, Serialize};
use std::error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Benchmark {
    pub name: String,
    pub path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Backend {
    pub name: String,
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Setup {
    pub connections: u32,
    pub duration: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub benchmarks: Vec<Benchmark>,
    pub backends: Vec<Backend>,
}

pub fn get_benchmark_configs() -> Result<BenchmarkConfig, Box<dyn error::Error>> {
    Ok(serde_yaml::from_str(&read_to_string(Path::new(
        "benchmarks.yml",
    ))?)?)
}
