use super::song::FullSongDetails;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct SongStats {
    pub play_count: u32,
    pub shuffled_count: u32,
    pub skipped_count: u32,
    pub full_play_count: u32,
    pub artist_name: String,
}

#[derive(Debug, Clone, Copy)]
pub enum SortKey {
    FullPlay,
}

pub fn build_unique_song_stats(songs: &[FullSongDetails]) -> HashMap<String, SongStats> {
    let start_time = Instant::now();

    let mut stats: HashMap<String, SongStats> = HashMap::new();
    for song in songs {
        if let Some(ref track_name) = song.master_metadata_track_name {
            let artist = song
                .master_metadata_album_artist_name
                .as_deref()
                .unwrap_or("Unknown Artist")
                .to_string();
            let entry = stats.entry(track_name.clone()).or_insert(SongStats {
                play_count: 0,
                shuffled_count: 0,
                skipped_count: 0,
                full_play_count: 0,
                artist_name: artist.clone(),
            });
            entry.play_count += 1;
            if song.is_shuffled() {
                entry.shuffled_count += 1;
            }
            if song.is_skipped() {
                entry.skipped_count += 1;
            }
            entry.full_play_count = entry.play_count.saturating_sub(entry.skipped_count);
        }
    }

    let time_elapsed = Instant::now() - start_time;
    println!("duration: {:?}", time_elapsed);
    stats
}

pub fn sorted_unique_by(
    stats: HashMap<String, SongStats>,
    _sort_key: SortKey,
) -> Vec<(String, SongStats)> {
    let mut sorted: Vec<(String, SongStats)> = stats.into_iter().collect();
    sorted.sort_unstable_by(|a, b| {
        b.1.full_play_count
            .cmp(&a.1.full_play_count)
            .then_with(|| a.0.cmp(&b.0))
    });
    sorted
}

pub fn count_unique_shuffled(stats: &HashMap<String, SongStats>) -> usize {
    stats
        .values()
        .filter(|entry| entry.shuffled_count > 0)
        .count()
}
