use clap::{Parser};


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
}