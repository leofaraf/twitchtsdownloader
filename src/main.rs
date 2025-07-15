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
    let playlists = m3u8::parse_m3u8_playlists(
        &m3u8::fetch_m3u8_playlists(args.vod).await?
    );
    println!("Parsed M3U8 playlists: {}", playlists.len());

    if let Some(playlist) = playlists.iter().find(|p| p.quality == args.quality) {
        println!("Found matching playlist: {:#?}", playlist);

        // Here you could add code to download the playlist or perform other actions
        let ts_segments = m3u8::get_ts_segments(&playlist.url).await?;
        println!("Found TS segments: {:#?}", ts_segments[0]);
    } else {
        println!("No matching playlist found.");
        println!("Available playlists: {:#?}", playlists);
    }

    Ok(())
}