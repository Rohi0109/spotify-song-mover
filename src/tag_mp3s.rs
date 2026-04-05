use std::collections::HashMap;
use std::fs::File;
use rayon::prelude::*;
use walkdir::WalkDir;
use lofty::{TagType, ItemKey, TagExt, TaggedFileExt};
use serde::Deserialize;

#[derive(Deserialize)]
struct SpotifyTrack {
    master_metadata_track_name: Option<String>,
    master_metadata_album_artist_name: Option<String>,
    master_metadata_album_album_name: Option<String>,
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .trim()
        .to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load combined.json into a map: normalized "artist - track" -> (title, artist, album)
    let file = File::open("spotify_files/combined.json")?;
    let data: Vec<SpotifyTrack> = serde_json::from_reader(file)?;
    let mut track_map: HashMap<String, (String, String, String)> = HashMap::new();

    for song in data {
        if let (Some(track), Some(artist)) = (&song.master_metadata_track_name, &song.master_metadata_album_artist_name) {
            let album = song.master_metadata_album_album_name.clone().unwrap_or_default();
            let key = normalize(&format!("{} - {}", artist, track));
            track_map.insert(key, (track.clone(), artist.clone(), album));
        }
    }

    // 2. Find all mp3 files in songs/
    let files: Vec<_> = WalkDir::new("songs")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map(|x| x == "mp3").unwrap_or(false))
        .collect();

    // 3. Parallel tagging
    files.par_iter().for_each(|entry| {
        let path = entry.path();
        let fname = path.file_stem().unwrap().to_string_lossy();
        let norm = normalize(&fname);

        // Try exact match, fallback to substring match
        let meta = track_map.get(&norm)
            .or_else(|| {
                // Fuzzy: find first where both artist and title are substrings
                track_map.iter().find(|(k, _)| norm.contains(&k[..]))
            });

        if let Some((title, artist, album)) = meta.map(|x| x.clone()) {
            if let Ok(mut tagged_file) = lofty::read_from_path(path, false) {
                let tag = tagged_file.primary_tag_mut().unwrap_or_else(|| {
                    tagged_file.insert_tag(lofty::Tag::new(TagType::ID3v2));
                    tagged_file.primary_tag_mut().unwrap()
                });
                tag.set_string(ItemKey::TrackTitle, title.clone());
                tag.set_string(ItemKey::AlbumTitle, album.clone());
                tag.set_string(ItemKey::TrackArtist, artist.clone());
                if let Err(e) = tagged_file.save_to_path(path) {
                    eprintln!("Failed to tag {}: {}", path.display(), e);
                } else {
                    println!("Tagged: {} - {} [{}]", artist, title, album);
                }
            }
        } else {
            println!("No match for: {}", fname);
        }
    });

    Ok(())
}
