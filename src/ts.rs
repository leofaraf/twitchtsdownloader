use reqwest::Client;
use tokio::{fs::File, io::AsyncWriteExt};
use std::path::{Path, PathBuf};

use crate::HandleResult;

pub async fn get_ts_segments(playlist_url: &str) -> HandleResult<Vec<String>> {
    let text = reqwest::get(playlist_url).await?.text().await?;
    let base_url = playlist_url.rsplitn(2, '/').collect::<Vec<_>>()[1];

    let mut segments = Vec::new();

    for line in text.lines() {
        if !line.starts_with("#") && line.ends_with(".ts") {
            segments.push(format!("{}/{}", base_url, line));
        }
    }

    Ok(segments)
}

pub async fn download_ts_fragment(
    url: &str,
    output_path: PathBuf,
) -> HandleResult<()> {
    // Start downloading
    let client = Client::new();
    let mut resp = client.get(url).send().await?.error_for_status()?;

    let mut file = File::create(output_path.clone()).await?;
    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk).await?;
    }

    println!("✅ Downloaded to {:?}", output_path);
    Ok(())
}