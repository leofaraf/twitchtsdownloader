use futures::{stream, StreamExt};
use reqwest::Client;
use tokio::{fs::File, io::{stdout, AsyncWriteExt}};
use std::{path::{Path, PathBuf}, sync::{atomic::{AtomicUsize, Ordering}, Arc}};

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

async fn download_with_retry(
    client: Arc<Client>,
    url: String,
    filename: String,
    progress: Arc<AtomicUsize>,
    total: usize,
) -> HandleResult<()> {
    // Start downloading
    let mut resp = client.get(url).send().await?.error_for_status()?;

    let mut file = File::create(&filename).await?;
    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk).await?;
    }

    Ok(())
}

pub async fn download_ts_with_buffered_lib(
    urls: Vec<String>,
    output_dir: &str,
    max_concurrent: usize,
) -> HandleResult<()> {
    let client = Arc::new(Client::new());
    let total = urls.len();
    let progress = Arc::new(AtomicUsize::new(0));

    let download_tasks = stream::iter(
        urls.into_iter().enumerate().map(|(i, url)| {
            let client = client.clone();
            let progress = progress.clone();
            let filename = format!("{}/{}.ts", output_dir, i);

            async move {
                // let done = progress.fetch_add(1, Ordering::SeqCst) + 1;
                // print!("Progress: {}/{}", done, total);
                download_with_retry(client, url, filename, progress, total).await
            }
        })
    );

    download_tasks
        .buffered(max_concurrent)
        .collect::<Vec<_>>()
        .await;

    println!("\n✅ Done downloading all fragments.");
    Ok(())
}