use crate::HandleResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use urlencoding;

const CLIENT_ID: &str = "kimne78kx3ncx6brgo4mv6wki5h1ko";

#[derive(Debug, Clone)]
pub struct M3U8Playlist {
    pub quality: String,
    pub resolution: Option<String>,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    pub value: String,
    pub signature: String,
}

#[derive(Serialize)]
struct PlaybackAccessTokenRequest {
    #[serde(rename = "operationName")]
    operation_name: String,
    extensions: Extensions,
    variables: Variables,
}

#[derive(Serialize)]
struct Extensions {
    #[serde(rename = "persistedQuery")]
    persisted_query: PersistedQuery,
}

#[derive(Serialize)]
struct PersistedQuery {
    version: u8,
    #[serde(rename = "sha256Hash")]
    sha256_hash: String,
}

#[derive(Serialize)]
struct Variables {
    #[serde(rename = "isLive")]
    is_live: bool,
    login: String,
    #[serde(rename = "isVod")]
    is_vod: bool,
    #[serde(rename = "vodID")]
    vod_id: String,
    #[serde(rename = "playerType")]
    player_type: String,
}


#[derive(Deserialize)]
struct PlaybackAccessTokenResponse {
    data: TokenData,
}

#[derive(Deserialize)]
struct TokenData {
    #[serde(rename = "streamPlaybackAccessToken")]
    stream: Option<AccessToken>,
    
    #[serde(rename = "videoPlaybackAccessToken")]
    vod: Option<AccessToken>,
}

pub async fn get_access_token(id: &str, is_vod: bool) -> Result<AccessToken, Box<dyn std::error::Error + Send + Sync>> {
    let body = PlaybackAccessTokenRequest {
        operation_name: "PlaybackAccessToken".to_string(),
        extensions: Extensions {
            persisted_query: PersistedQuery {
                version: 1,
                sha256_hash: "0828119ded1c13477966434e15800ff57ddacf13ba1911c129dc2200705b0712"
                    .to_string(),
            },
        },
        variables: Variables {
            is_live: !is_vod,
            login: if is_vod { "".to_string() } else { id.to_string() },
            is_vod,
            vod_id: if is_vod { id.to_string() } else { "".to_string() },
            player_type: "embed".to_string(),
        },
    };

    let client = Client::new();
    let res = client
        .post("https://gql.twitch.tv/gql")
        .header("Client-Id", CLIENT_ID)
        .json(&body)
        .send()
        .await?;
    let text = res.text().await?;

    let parsed: Result<PlaybackAccessTokenResponse, _> = serde_json::from_str(&text);
    match parsed {
        Ok(parsed_response) => {
            Ok(if is_vod {
                parsed_response.data.vod.unwrap()
            } else {
                parsed_response.data.stream.unwrap()
            })
        }
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn fetch_m3u8_playlists(vod: i64) -> HandleResult<String> {
    let token = get_access_token(&vod.to_string(), true)
        .await
        .expect("Failed to fetch access token");

    let playlist_url = format!(
        "https://usher.ttvnw.net/vod/{}.m3u8?client_id={}&token={}&sig={}&allow_source=true&allow_audio_only=true",
        vod,
        CLIENT_ID,
        urlencoding::encode(&token.value),
        token.signature
    );

    let playlist_text = reqwest::get(&playlist_url)
        .await?
        .text()
        .await?;

    Ok(playlist_text)
}

use std::collections::HashMap;

pub fn parse_m3u8_playlists(raw: &str) -> Vec<M3U8Playlist> {
    let mut result = Vec::new();
    let mut media_name_map = HashMap::new();
    let lines: Vec<&str> = raw.lines().collect();

    // First pass: build VIDEO group ID → quality name map from #EXT-X-MEDIA lines
    for line in &lines {
        if line.starts_with("#EXT-X-MEDIA") && line.contains("TYPE=VIDEO") {
            if let (Some(group_id), Some(name)) = (
                extract_attr(line, "GROUP-ID"),
                extract_attr(line, "NAME"),
            ) {
                media_name_map.insert(group_id.trim_matches('"'), name.trim_matches('"'));
            }
        }
    }

    // Second pass: match #EXT-X-STREAM-INF with next URL and resolve name from VIDEO attr
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("#EXT-X-STREAM-INF") {
            let resolution = extract_attr(lines[i], "RESOLUTION").map(|r| r.to_string());

            let video_id = extract_attr(lines[i], "VIDEO")
                .unwrap_or("")
                .trim_matches('"');

            let quality = media_name_map
                .get(video_id)
                .unwrap_or(&"unknown")
                .to_string();

            if i + 1 < lines.len() {
                let url = lines[i + 1].to_string();
                result.push(M3U8Playlist {
                    quality,
                    resolution,
                    url,
                });
                i += 1;
            }
        }
        i += 1;
    }

    result
}

fn extract_attr<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    line.split(',')
        .find(|part| part.trim_start().starts_with(key))
        .and_then(|part| part.split('=').nth(1))
}