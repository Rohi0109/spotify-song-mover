use super::song::FullSongDetails;

pub fn from_json_file(path: &str) -> Vec<FullSongDetails> {
    let json = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {}", e);
        String::new()
    });
    let full_song_list: Vec<FullSongDetails> = serde_json::from_str(&json).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON: {}", e);
        Vec::new()
    });
    full_song_list
}
