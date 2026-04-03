use log::info;
use std::path::PathBuf;
use std::fs;
use yt_dlp::Downloader;
use super::playing_with_algo::SongStats;

pub async fn downloading_songs(songs: &[(String, SongStats)]) -> Result<(), Box<dyn std::error::Error>> {
    info!("lets gooooo!");
    
    // Create the songs directory if it doesn't exist
    fs::create_dir_all("../songs")?;
    
    // Auto-download yt-dlp and ffmpeg binaries
    let downloader = Downloader::with_new_binaries(
        PathBuf::from("libs"),
        PathBuf::from("output")
    ).await?
        .build().await?;


    for (track_name, stats) in songs.iter() {
        let query = format!("{} {}", stats.artist_name, track_name);
        println!("Searching for: {}", query);
        
        let youtube = downloader.youtube_extractor();
        let results = youtube.search(&query, 1).await?;
        
        // Get the first video result
        if let Some(first_result) = results.entries.first() {
            // Construct the YouTube URL from the video ID
            let video_url = format!("https://youtube.com/watch?v={}", first_result.id);
            
            // Fetch the full video info
            let video = downloader.fetch_video_infos(video_url).await?;
            
            // Download as audio (mp3) to songs folder
            let filename = format!("../songs/{}.mp3", video.title);
            downloader.download_audio_stream(&video, &filename).await?;
        }
    }
    
    Ok(())
}