use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use google_youtube3::api::{Playlist, PlaylistItem, PlaylistItemSnippet, PlaylistSnippet, PlaylistStatus, ResourceId};
use google_youtube3::YouTube;
use google_youtube3::hyper_rustls::HttpsConnectorBuilder;
use hyper_util::client::legacy::connect::HttpConnector;
use rustls;

type YtHub = YouTube<hyper_rustls::HttpsConnector<HttpConnector>>;

const VIDEO_CACHE_PATH: &str = "docs/video_cache.json";

/// Load cached video IDs from disk (track_name -> video_id)
pub fn load_video_cache() -> HashMap<String, String> {
    if Path::new(VIDEO_CACHE_PATH).exists() {
        let data = fs::read_to_string(VIDEO_CACHE_PATH).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

/// Save video ID cache to disk
pub fn save_video_cache(cache: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(cache)?;
    fs::write(VIDEO_CACHE_PATH, json)?;
    Ok(())
}

/// Build an authenticated YouTube hub using OAuth 2.0
pub async fn build_youtube_hub(client_secret_path: &str) -> Result<YtHub, Box<dyn Error>> {
    // Install default crypto provider for rustls
    let _ = rustls::crypto::ring::default_provider().install_default();
    
    let secret = yup_oauth2::read_application_secret(client_secret_path).await?;

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("docs/youtube_tokens.json")
    .build()
    .await?;

    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_or_http()
        .enable_http2()
        .build();

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(connector);

    let hub = YouTube::new(client, auth);
    Ok(hub)
}

/// Search YouTube for a video matching "artist - track", returns video ID
pub async fn search_video(
    hub: &YtHub,
    api_key: &str,
    artist: &str,
    track: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let query = format!("{} - {}", artist, track);

    let result = hub
        .search()
        .list(&vec!["snippet".into()])
        .q(&query)
        .add_type("video")
        .max_results(1)
        .param("key", api_key)
        .doit()
        .await;

    match result {
        Ok((_, search_response)) => {
            if let Some(items) = search_response.items {
                if let Some(item) = items.first() {
                    if let Some(ref id) = item.id {
                        return Ok(id.video_id.clone());
                    }
                }
            }
            Ok(None)
        }
        Err(e) => {
            eprintln!("search error for '{}': {}", query, e);
            Ok(None)
        }
    }
}

/// Create a new YouTube playlist, returns playlist ID
pub async fn create_playlist(
    hub: &YtHub,
    title: &str,
) -> Result<String, Box<dyn Error>> {
    let playlist = Playlist {
        snippet: Some(PlaylistSnippet {
            title: Some(title.to_string()),
            description: Some("Auto-generated from Spotify listening history".to_string()),
            ..Default::default()
        }),
        status: Some(PlaylistStatus {
            privacy_status: Some("private".to_string()),
        }),
        ..Default::default()
    };

    let (_, created) = hub
        .playlists()
        .insert(playlist)
        .doit()
        .await?;

    created
        .id
        .ok_or_else(|| "playlist creation returned no ID".into())
}

/// Add a video to a playlist
pub async fn add_to_playlist(
    hub: &YtHub,
    playlist_id: &str,
    video_id: &str,
) -> Result<(), Box<dyn Error>> {
    let item = PlaylistItem {
        snippet: Some(PlaylistItemSnippet {
            playlist_id: Some(playlist_id.to_string()),
            resource_id: Some(ResourceId {
                kind: Some("youtube#video".to_string()),
                video_id: Some(video_id.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    hub.playlist_items().insert(item).doit().await?;
    Ok(())
}
