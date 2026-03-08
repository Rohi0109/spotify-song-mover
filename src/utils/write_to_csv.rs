use super::playing_with_algo::SongStats;
use serde::Serialize;
use std::error::Error;
use std::fs::File;

#[derive(Serialize)]
struct CsvSongRow {
    track_name: String,
    artist_name: String,
    play_count: u32,
    shuffled_count: u32,
    full_play_count: u32,
}

pub fn write_unique_song_stats_to_csv(
    path: &str,
    rows: &[(String, SongStats)],
) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(file);

    for (track_name, stats) in rows {
        writer.serialize(CsvSongRow {
            track_name: track_name.clone(),
            artist_name: stats.artist_name.clone(),
            play_count: stats.play_count,
            shuffled_count: stats.shuffled_count,
            full_play_count: stats.full_play_count,
        })?;
    }

    writer.flush()?;
    Ok(())
}
