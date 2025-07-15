use clap::Parser;
use std::path::PathBuf;

/// Clippy utility that accepts quality, VOD ID, and output folder
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// VOD ID
    vod: String,

    /// Quality level (e.g., 1080p, 720p)
    quality: String,

    /// Output folder path
    output_folder: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("VOD ID: {}", args.vod);
    println!("Quality: {}", args.quality);
    println!("Output folder: {:?}", args.output_folder);

    // Here you could perform further logic (e.g., create output folder, call a function, etc.)
}