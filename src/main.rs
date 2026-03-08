mod utils;
use std::time::Instant;
use utils::playing_with_algo::{
    SortKey, build_unique_song_stats, count_unique_shuffled, sorted_unique_by,
};
use utils::pull_data::from_json_file;
use utils::song::FullSongDetails;
use utils::write_to_csv::write_unique_song_stats_to_csv;
use utils::youtube;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY not set in .env");
    let client_secret_path =
        std::env::var("CLIENT_SECRET_PATH").expect("CLIENT_SECRET_PATH not set in .env");

    let start_time = Instant::now();
    println!("starting!");

    // --- load & analyse ---
    let full_song_list = from_json_file("spotify_files/combined.json");
    let filtered = FullSongDetails::filter_tracks(full_song_list);
    println!("{}", filtered.len());
    let stats = build_unique_song_stats(filtered);
    println!("unique shuffled songs: {}", count_unique_shuffled(&stats));
    let sorted_count = sorted_unique_by(stats, SortKey::FullPlay);
    println!("unique songs: {}", sorted_count.len());
    if let Err(err) = write_unique_song_stats_to_csv("docs/unique_songs.csv", &sorted_count) {
        eprintln!("failed to write csv: {}", err);
    }

    // --- youtube: top 100 ---
    let top_100: Vec<_> = sorted_count.iter().take(100).collect();

    // load cache
    let mut cache = youtube::load_video_cache();
    let hub = youtube::build_youtube_hub(&client_secret_path)
        .await
        .expect("failed to build YouTube hub (check client_secret json)");

    // search for video IDs (uses cache to save quota)
    println!("\n--- searching YouTube for top 100 songs ---");
    for (i, (track, stats)) in top_100.iter().enumerate() {
        if cache.contains_key(track.as_str()) {
            println!("[{}/100] cached: {}", i + 1, track);
            continue;
        }
        match youtube::search_video(&hub, &api_key, &stats.artist_name, track).await {
            Ok(Some(vid)) => {
                println!("[{}/100] found: {} -> {}", i + 1, track, vid);
                cache.insert(track.clone(), vid);
            }
            Ok(None) => {
                eprintln!("[{}/100] NOT FOUND: {}", i + 1, track);
            }
            Err(e) => {
                eprintln!("[{}/100] error: {} - {}", i + 1, track, e);
            }
        }
        // save cache after each search so progress is not lost
        let _ = youtube::save_video_cache(&cache);
    }
    youtube::save_video_cache(&cache).expect("failed to save video cache");

    // create playlist and add videos
    println!("\n--- creating YouTube playlist ---");
    match youtube::create_playlist(&hub, "Top 100 Spotify Songs").await {
        Ok(playlist_id) => {
            println!("created playlist: {}", playlist_id);
            for (i, (track, _)) in top_100.iter().enumerate() {
                if let Some(video_id) = cache.get(track.as_str()) {
                    match youtube::add_to_playlist(&hub, &playlist_id, video_id).await {
                        Ok(_) => println!("[{}/100] added: {}", i + 1, track),
                        Err(e) => eprintln!("[{}/100] failed to add {}: {}", i + 1, track, e),
                    }
                } else {
                    eprintln!("[{}/100] skipped (no video): {}", i + 1, track);
                }
            }
            println!(
                "\nplaylist complete! https://www.youtube.com/playlist?list={}",
                playlist_id
            );
        }
        Err(e) => eprintln!("failed to create playlist: {}", e),
    }

    let duration = start_time.elapsed();
    println!("finished! took {:?} seconds", duration.as_secs_f64());
}
