use crate::config::oha::OhaResult;
use serde::{Deserialize, Serialize};
use std::error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Benchmark {
    pub name: String,
    pub path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Backend {
    pub name: String,
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Setup {
    pub connections: u32,
    pub duration: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BenchmarkConfig {
    pub benchmarks: Vec<Benchmark>,
    pub backends: Vec<Backend>,
}

pub fn get_benchmark_configs() -> Result<BenchmarkConfig, Box<dyn error::Error>> {
    Ok(serde_yaml::from_str(&read_to_string(Path::new(
        "benchmarks.yml",
    ))?)?)
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BenchmarkResult {
    pub benchmark_name: String,
    pub framework_name: String,
    pub setup: Setup,
    pub stats: OhaResult,
}
