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
///
/// Set SKIP_SONGS=N environment variable to skip first N songs (for resuming)
pub async fn downloading_songs(
    songs: &[(String, SongStats)],
) -> Result<(), Box<dyn std::error::Error>> {
    info!("lets gooooo!");

    // Create the songs directory if it doesn't exist
    fs::create_dir_all("songs")?;

    // Count existing songs to skip (resume from where we left off)
    let skip_count = fs::read_dir("songs")?.count();
    if skip_count > 0 {
        println!("Found {} existing songs, skipping...", skip_count);
    }

    for (i, (track_name, stats)) in songs.iter().enumerate() {
        // Skip already downloaded songs
        if i < skip_count {
            continue;
        }

        let query = format!("ytsearch1:{} {}", stats.artist_name, track_name);
        println!(
            "[{}/{}] Searching for: {} - {}",
            i + 1,
            songs.len(),
            stats.artist_name,
            track_name
        );

        // Format metadata strings
        let artist_meta = format!("artist:{}", stats.artist_name);
        let title_meta = format!("title:{}", track_name);

        // Let yt-dlp handle filename sanitization with %(title)s
        let output = Command::new("yt-dlp")
            .args([
                "--cookies",
                "cookies.txt",
                "-x",
                "--audio-format",
                "mp3",
                "--no-playlist",
                "--embed-metadata",
                "--parse-metadata",
                &artist_meta,
                "--parse-metadata",
                &title_meta,
                "-o",
                "songs/%(title)s.%(ext)s",
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
