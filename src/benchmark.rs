use crate::config::benchmark::{Backend, Benchmark};
use log::info;
use std::error::Error;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use which::which;

const OHA_URL: &str = "https://github.com/hatoo/oha/releases/download/v1.4.5/oha-linux-amd64";

#[derive(Debug, Clone)]
pub struct OhaExecutable {
    pub executable_path: PathBuf,
}

impl OhaExecutable {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        match which("oha") {
            Ok(p) => {
                info!("Found oha executable at: {:?}", p);
                Ok(OhaExecutable { executable_path: p })
            }
            Err(_) => {
                info!("Could not find oha executable in PATH, seeing if it is in tmp directory");
                let tmp_path = PathBuf::from("/tmp/restful-benchmarks/oha");
                if tmp_path.exists() {
                    info!("Found oha executable at: {:?}", tmp_path);
                    Ok(OhaExecutable {
                        executable_path: tmp_path,
                    })
                } else {
                    info!("Could not find oha executable in tmp directory, downloading it");
                    let output = reqwest::get(OHA_URL)
                        .await
                        .map_err(|e| format!("Failed to download oha executable: {}", e))?
                        .bytes()
                        .await
                        .map_err(|e| format!("Failed to download oha executable: {}", e))?;
                    std::fs::create_dir_all("/tmp/restful-benchmarks")
                        .map_err(|e| format!("Failed to create tmp directory: {}", e))?;
                    std::fs::write(&tmp_path, output).map_err(|e| {
                        format!("Failed to write oha executable to tmp directory: {}", e)
                    })?;
                    std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))
                        .map_err(|e| {
                            format!(
                                "Failed to set permissions on oha executable in tmp directory: {}",
                                e
                            )
                        })?;
                    info!("Downloaded oha executable to: {:?}", tmp_path);
                    Ok(OhaExecutable {
                        executable_path: tmp_path,
                    })
                }
            }
        }
    }

    pub fn run_benchmark(
        setup: &Backend,
        benchmark: &Benchmark,
        oha_executable: &OhaExecutable,
        url: &str,
    ) {
    }
}
