use std::{error::Error, process::Command};

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/:proxy/:wspw", get(get_info).delete(stop_streaming));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    axum::serve(listener, app).await
}

async fn get_info(
    Path((proxy, wspw)): Path<(String, String)>,
    port: Option<Query<Port>>,
) -> Result<String, (StatusCode, String)> {
    let Query(Port { port }) = port.unwrap_or_default();
    let mut zrok_output = err(Command::new("zrok")
        .args([
            "access",
            "private",
            proxy.as_str(),
            "--headless",
            "-b",
            &format!("127.0.0.1:{}", port),
        ])
        .spawn())?;

    let obs_output = err(Command::new("obs-cmd")
        .args([
            "-w",
            &format!("obsws://localhost:{}/{}", port, wspw),
            "info",
        ])
        .output())?;
    err(zrok_output.kill())?;
    Ok(format!(
        "{:?}\n{:?}",
        zrok_output,
        String::from_utf8(if obs_output.stderr.is_empty() {
            obs_output.stdout
        } else {
            obs_output.stderr
        })
    ))
}
async fn stop_streaming(
    Path((proxy, wspw)): Path<(String, String)>,
    port: Option<Query<Port>>,
) -> Result<String, (StatusCode, String)> {
    let Query(Port { port }) = port.unwrap_or_default();
    let mut zrok_output = err(Command::new("zrok")
        .args([
            "access",
            "private",
            proxy.as_str(),
            "--headless",
            "-b",
            &format!("127.0.0.1:{}", port),
        ])
        .spawn())?;

    let obs_output = err(Command::new("obs-cmd")
        .args([
            "-w",
            &format!("obsws://localhost:{}/{}", port, wspw),
            "streaming",
            "stop",
        ])
        .output())?;
    err(zrok_output.kill())?;
    Ok(format!(
        "{:?}\n{:?}",
        zrok_output,
        String::from_utf8(if obs_output.stderr.is_empty() {
            obs_output.stdout
        } else {
            obs_output.stderr
        })
    ))
}

#[derive(Deserialize)]
struct Port {
    port: String,
}

impl Default for Port {
    fn default() -> Self {
        Self {
            port: String::from("9191"),
        }
    }
}

fn err<T, E>(r: Result<T, E>) -> Result<T, (StatusCode, String)>
where
    E: Error,
{
    r.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
}
