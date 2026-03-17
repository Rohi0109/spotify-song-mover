use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct FullSongDetails {
    ts: DateTime<Utc>,
    platform: String,
    ms_played: u64,
    conn_country: String,
    ip_addr: String,
    pub master_metadata_track_name: Option<String>,
    pub master_metadata_album_artist_name: Option<String>,
    master_metadata_album_album_name: Option<String>,
    spotify_track_uri: Option<String>,
    episode_name: Option<String>,
    episode_show_name: Option<String>,
    spotify_episode_uri: Option<String>,
    audiobook_title: Option<String>,
    audiobook_uri: Option<String>,
    audiobook_chapter_uri: Option<String>,
    audiobook_chapter_title: Option<String>,
    reason_start: String,
    reason_end: String,
    shuffle: bool,
    skipped: bool,
    offline: Option<bool>,
    offline_timestamp: Option<i64>,
    incognito_mode: bool,
}
impl FullSongDetails {
    pub fn filter_tracks(songs: Vec<FullSongDetails>) -> Vec<FullSongDetails> {
        songs
            .into_iter()
            .filter(|song| song.spotify_track_uri.is_some())
            .collect()
    }

    pub fn is_shuffled(&self) -> bool {
        self.shuffle
    }

    pub fn is_skipped(&self) -> bool {
        self.skipped
    }
}
struct SongDetails {
    timestamp: DateTime<Utc>,
    platform: String,
    ms_played: u64,
    conn_country: String,
    master_metadata_track_name: Option<String>,
    master_metadata_album_artist_name: Option<String>,
    master_metadata_album_album_name: Option<String>,
    spotify_track_uri: Option<String>,
    reason_start: String,
    reason_end: String,
    shuffle: bool,
    skipped: bool,
    offline: bool,
    offline_timestamp: Option<i64>,
}
