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
}

impl Arguments {
    pub fn custom_validate(&self, framework_configs: &[&str]) -> Result<(), String> {
        for framework in &self.frameworks {
            validate_list(framework, framework_configs)?;
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
