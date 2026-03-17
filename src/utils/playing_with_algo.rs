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
    PlayCount,
    ShuffledCount,
    FullPlayCount,
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
    sort_key: SortKey,
) -> Vec<(String, SongStats)> {
    let mut sorted: Vec<(String, SongStats)> = stats.into_iter().collect();
    sorted.sort_unstable_by(|a, b| {
        let a_val = match sort_key {
            SortKey::PlayCount => a.1.play_count,
            SortKey::ShuffledCount => a.1.shuffled_count,
            SortKey::FullPlayCount => a.1.full_play_count,
        };
        let b_val = match sort_key {
            SortKey::PlayCount => b.1.play_count,
            SortKey::ShuffledCount => b.1.shuffled_count,
            SortKey::FullPlayCount => b.1.full_play_count,
        };
        b_val.cmp(&a_val).then_with(|| a.0.cmp(&b.0))
    });
    sorted
}

pub fn count_unique_shuffled(stats: &HashMap<String, SongStats>) -> usize {
    stats.values().filter(|entry| entry.shuffled_count > 0).count()
}

pub fn counting_unique(songs:Vec<FullSongDetails>) ->  HashMap<String,i32> {
    let start_time = Instant::now(); // Start the timer

    let  mut counts =HashMap::new();
    for song in songs { //i should filter these out but rust yells at me if i dont
        if let Some(track_name) = song.master_metadata_track_name {
            *counts.entry(track_name).or_insert(0) += 1;
        }
    }
    let time_elapsed = Instant::now() - start_time;
    println!("duration: {:?}", time_elapsed);
    counts
}

pub fn sorting_count(count:HashMap<String,i32>) -> Vec<(std::string::String, i32)>{
    let mut sorted : Vec<(String,i32)> = count.into_iter().collect();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted

}

