use std::error;
use std::fs::read_to_string;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Benchmark {
    pub name: String,
    pub path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Setup {
    pub name: String,
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub benchmarks: Vec<Benchmark>,
    pub setups: Vec<Setup>
}

pub fn get_benchmark_configs() -> Result<BenchmarkConfig, Box<dyn error::Error>> {
    Ok(serde_yaml::from_str(&read_to_string(Path::new("benchmarks.yml"))?)?)
}