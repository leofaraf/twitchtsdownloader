use clap::Parser;
use std::path::PathBuf;

mod m3u8;

pub type HandleResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Clippy utility that accepts quality, VOD ID, and output folder
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// VOD ID
    vod: i64,

    /// Quality level (e.g., 1080p, 720p)
    quality: String,

    /// Output folder path
    output_folder: PathBuf,
}

#[tokio::main]
async fn main() -> HandleResult<()> {
    let args = Args::parse();

    println!("VOD ID: {}", args.vod);
    println!("Quality: {}", args.quality);
    println!("Output folder: {:?}", args.output_folder);

    // Here you could perform further logic (e.g., create output folder, call a function, etc.)
    let token = m3u8::get_access_token(&args.vod.to_string(), true).await.unwrap();
    println!("Fetched access token: {:?}", token);

    let playlist = m3u8::fetch_m3u8_playlist(args.vod).await?;

    Ok(())
}