use crate::backend::{start_benchmark_runner_application, ImageType};
use crate::config::benchmark::{Backend, Benchmark, BenchmarkResult, Setup};
use crate::config::framework::FrameworkConfig;
use crate::config::oha::OhaResult;
use docker_api::opts::LogsOptsBuilder;
use docker_api::{Docker, Network};
use futures_util::TryStreamExt;
use log::info;
use std::error::Error;

pub async fn run_benchmark(
    docker: &Docker,
    network: &Network,
    image_type: ImageType,
    framework: &FrameworkConfig,
    backend: &Backend,
    benchmark: &Benchmark,
    setup: &Setup,
) -> Result<BenchmarkResult, Box<dyn Error>> {
    info!(
        "Running {}: [{}] [{}] [{} connections] [{} seconds]",
        framework.name, backend.name, benchmark.name, setup.connections, setup.duration
    );
    let result = start_benchmark_runner_application(
        docker,
        network,
        image_type,
        format!(
            "oha -z {}s -c {} --latency-correction --disable-keepalive --no-tui -j {}{}",
            setup.duration, setup.connections, framework.url, benchmark.path
        )
            .as_str(),
    )
        .await?;
    result.wait().await?;
    let reader = result.logs(&LogsOptsBuilder::default().stdout(true).stderr(true).build());
    let logs = reader
        .try_collect::<Vec<_>>()
        .await
        .map_err(|e| format!("Error reading logs: {}", e))?;
    let logs = logs.iter().map(|x| x.as_slice()).collect::<Vec<&[u8]>>();
    let logs =
        String::from_utf8(logs.concat()).map_err(|e| format!("Error parsing logs: {}", e))?;
    let result: OhaResult = serde_json::from_str(&logs)
        .map_err(|e| format!("Error parsing logs into Struct: {e} {logs}"))?;
    Ok(BenchmarkResult {
        benchmark_name: benchmark.name.clone(),
        framework_name: framework.name.clone(),
        setup: setup.clone(),
        stats: result,
    })
}
