use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// Only Start Solr and Varnish
    #[clap(long)]
    pub development: bool,
    /// Use remote images to run the benchmark
    #[clap(long, conflicts_with = "local")]
    pub remote: bool,
    /// Build images locally to run the benchmark
    #[clap(long, conflicts_with = "remote")]
    pub local: bool,
    /// Forward the varnish image to the current port on host
    #[clap(short, long, default_value_t = 8983)]
    pub port: u16,
    /// Specify which frameworks to run
    #[clap(long, conflicts_with = "development")]
    pub frameworks: Vec<String>,
    /// Specify which benchmarks to run
    #[clap(long, conflicts_with = "development")]
    pub benchmarks: Vec<String>,
    /// Specify which backends to run
    #[clap(long, conflicts_with = "development")]
    pub backends: Vec<String>,
    /// Specify which setups to run
    #[clap(long, conflicts_with = "development")]
    pub setups: Vec<String>,
}

impl Arguments {
    pub fn custom_validate(&self, framework_configs: &[&str], benchmarks_configs: &[&str], backends: &[&str], setups: &[&str]) -> Result<(), String> {
        for framework in &self.frameworks {
            validate_list(framework, framework_configs)?;
        }
        for benchmark in &self.benchmarks {
            validate_list(benchmark, benchmarks_configs)?;
        }
        for backend in &self.backends {
            validate_list(backend, backends)?;
        }
        for setup in &self.setups {
            validate_list(setup, setups)?;
        }
        Ok(())
    }
}

fn validate_list(v: &str, valid_values: &[&str]) -> Result<(), String> {
    if valid_values.contains(&v) {
        Ok(())
    } else {
        Err(format!(
            "{} is not a valid value, valid values are {}",
            v,
            valid_values.join(", ")
        ))
    }
}
