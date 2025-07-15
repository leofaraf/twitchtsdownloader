# Twitch TS Fragments (*MPEG-4 transport stream*) Downloader

A command-line tool to download Twitch VODs (`.ts` video segments) at a specified quality using concurrent downloading and M3U8 playlist parsing.

## ✨ Features

* Fetches and parses M3U8 playlists for a given Twitch VOD
* Downloads `.ts` video segments for the desired quality
* Supports concurrent downloads to speed up retrieval
* Written in Rust with async support (`tokio`)

## 🚀 Usage

### 1. Install

Ensure you have Rust installed.

</span></div></div></div><div class="overflow-y-auto p-4" dir="ltr"><code class="whitespace-pre! language-bash"><span><span>git </span><span>clone</span><span> https://github.com/yourusername/twitch-vod-downloader.git
</span><span>cd</span><span> twitch-vod-downloader
cargo build --release
</span></span></code></div></div></pre>

### 2. Run

</span></div></div></div><div class="overflow-y-auto p-4" dir="ltr"><code class="whitespace-pre! language-bash"><span><span>./target/release/twitch-vod-downloader --vod <VOD_ID> --quality <QUALITY> --output <OUTPUT_DIR> [--concurrency <NUM>]
</span></span></code></div></div></pre>

#### Example

</span></div></div></div><div class="overflow-y-auto p-4" dir="ltr"><code class="whitespace-pre! language-bash"><span><span>./target/release/twitch-vod-downloader --vod 877379571 --quality 720p --output ./downloads --concurrency 10
</span></span></code></div></div></pre>

## 📦 Arguments

| Flag                     | Description                                | Required | Example         |
| ------------------------ | ------------------------------------------ | -------- | --------------- |
| `--vod`,`-v`         | Twitch VOD ID                              | ✅       | `877379571`   |
| `--quality`,`-q`     | Desired quality (e.g.,`720p`,`480p`)   | ✅       | `720p`        |
| `--output`,`-o`      | Output directory for `.ts`files          | ✅       | `./downloads` |
| `--concurrency`,`-c` | Number of parallel downloads (default: 20) | ❌       | `10`          |

## 🔧 Notes

* The tool does **not** assemble `.ts` files into a single video. You may use tools like `ffmpeg` for post-processing.
* Make sure the provided quality (e.g., `720p`) matches one of the available options for the VOD.

## 📜 License

MIT

---
