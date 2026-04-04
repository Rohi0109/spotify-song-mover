use super::playing_with_algo::SongStats;
use log::info;
use std::fs;
use std::process::Command;

/// Downloads songs using yt-dlp CLI directly.
/// 
/// # Requirements (install separately)
/// - yt-dlp: `pip install yt-dlp` or `sudo curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp && sudo chmod a+rx /usr/local/bin/yt-dlp`
/// - ffmpeg: `sudo apt install ffmpeg`
/// - node.js: `sudo apt install nodejs`
/// - cookies.txt: Export from browser with `yt-dlp --cookies-from-browser firefox --cookies cookies.txt "https://youtube.com"`
pub async fn downloading_songs(
    songs: &[(String, SongStats)],
) -> Result<(), Box<dyn std::error::Error>> {
    info!("lets gooooo!");

    // Create the songs directory if it doesn't exist
    fs::create_dir_all("songs")?;

    for (track_name, stats) in songs.iter() {
        let query = format!("ytsearch1:{} {}", stats.artist_name, track_name);
        println!("Searching for: {} - {}", stats.artist_name, track_name);

        // Let yt-dlp handle filename sanitization with %(title)s
        let output = Command::new("yt-dlp")
            .args([
                "--js-runtimes", "node:/usr/bin/node",
                "--extractor-args", "youtube:player_client=web",
                "--cookies", "cookies.txt",
                "-x",
                "--audio-format", "mp3",
                "--no-playlist",
                "-o", "songs/%(title)s.%(ext)s",
                &query,
            ])
            .output()?;

        if output.status.success() {
            println!("Downloaded: {} - {}", stats.artist_name, track_name);
        } else {
            eprintln!(
                "Failed to download {} - {}: {}",
                stats.artist_name,
                track_name,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    Ok(())
}
