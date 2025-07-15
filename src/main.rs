use clap::Parser;

mod m3u8;
mod ts;

pub type HandleResult<T> = Result<T, Box<dyn std::error::Error>>;
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Twitch VOD ID (e.g. 877379571)
    #[arg(short, long)]
    pub vod: i64,

    /// Desired quality level (e.g. 720p, 480p)
    #[arg(short, long)]
    pub quality: String,

    /// Output folder for .ts files
    #[arg(short, long)]
    pub output: String,

    /// Number of concurrent downloads
    #[arg(short, long, default_value_t = 20)]
    pub concurrency: usize,
}

#[tokio::main]
async fn main() -> HandleResult<()> {
    let args = Args::parse();

    println!("VOD ID: {}", args.vod);
    println!("Quality: {}", args.quality);
    println!("Output folder: {:?}", args.output);

    // Here you could perform further logic (e.g., create output folder, call a function, etc.)
    let playlists = m3u8::parse_m3u8_playlists(
        &m3u8::fetch_m3u8_playlists(args.vod).await?
    );
    println!("Parsed M3U8 playlists: {}", playlists.len());

    if let Some(playlist) = playlists.iter().find(|p| p.quality == args.quality) {
        println!("Found matching playlist: {:#?}", playlist);

        // Here you could add code to download the playlist or perform other actions
        let ts_segments = ts::get_ts_segments(&playlist.url).await?;

        ts::download_ts_with_buffered_lib(
            ts_segments,
            &args.output,
            args.concurrency
        ).await?;
    } else {
        println!("No matching playlist found.");
        println!("Available playlists: {:#?}", playlists);
    }

    Ok(())
}