use std::{error::Error, process::Command};

use axum::{extract::Path, http::StatusCode, routing::get, Router};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/:proxy/:wspw", get(get_info).delete(stop_streaming));

    let listener = tokio::net::TcpListener::bind("0.0.0.0").await?;
    axum::serve(listener, app).await
}

async fn get_info(
    Path((proxy, wspw)): Path<(String, String)>,
) -> Result<String, (StatusCode, String)> {
    let zrok_output = err(Command::new("zrok")
        .args(["access", "private", proxy.as_str(), "--headless"])
        .spawn())?;

    let obs_output = err(Command::new("obs-cmd")
        .args(["-w", &format!("obsws://localhost:9191/{}", wspw), "info"])
        .output())?;
    Ok(format!("{:?}\n{:?}", zrok_output, obs_output))
}
async fn stop_streaming(
    Path((proxy, wspw)): Path<(String, String)>,
) -> Result<String, (StatusCode, String)> {
    todo!()
}

fn err<T, E>(r: Result<T, E>) -> Result<T, (StatusCode, String)>
where
    E: Error,
{
    r.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))
}
