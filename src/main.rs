mod utils;
use std::time::Instant;
use utils::downloading_to_navidrome::downloading_songs;
use utils::playing_with_algo::{
    build_unique_song_stats, count_unique_shuffled, sorted_unique_by, SortKey,
};
use utils::pull_data::from_json_file;
use utils::song::FullSongDetails;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let start_time = Instant::now();
    println!("starting!");

    // --- load & analyse ---
    let full_song_list = from_json_file("spotify_files/combined.json");
    let filtered = FullSongDetails::filter_tracks(full_song_list);
    println!("{}", filtered.len());
    let stats = build_unique_song_stats(&filtered);
    println!("unique shuffled songs: {}", count_unique_shuffled(&stats));
    let sorted_count = sorted_unique_by(stats, SortKey::FullPlay);
    println!("unique songs: {}", sorted_count.len());
    // if let Err(err) = write_unique_song_stats_to_csv("docs/unique_songs.csv", &sorted_count) {
    //     eprintln!("failed to write csv: {}", err);
    // }
    if let Err(err) = downloading_songs(&sorted_count).await {
        eprintln!("failed to download songs: {}", err);
    }

    let duration = start_time.elapsed();
    println!("finished! took {:?} seconds", duration.as_secs_f64());
}
