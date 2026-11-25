use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Song {
    pub track: u32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub length_secs: u64,
    pub filename: String,
}

pub fn get_songs(path: &Path) -> Vec<Song> {
    let mut songs = Vec::new();
    collect_songs(path, &mut songs);
    songs
}

fn collect_songs(path: &Path, songs: &mut Vec<Song>) {
    if path.is_dir() {
        let Ok(entries) = fs::read_dir(path) else {
            return;
        };

        for entry in entries.flatten() {
            collect_songs(&entry.path(), songs);
        }
        return;
    }

    if path.is_file()
        && let Some(song) = parse_song(path)
    {
        songs.push(song);
    }
}

fn parse_song(path: &Path) -> Option<Song> {
    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_string();

    let length_secs = tagged_file.properties().duration().as_secs();

    let Some(tag) = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())
    else {
        return Some(Song {
            track: 0,
            title: filename.clone(),
            artist: String::new(),
            album: String::new(),
            length_secs,
            filename,
        });
    };

    Some(Song {
        track: tag.track().unwrap_or(0),
        title: tag
            .title()
            .map(|s| s.into_owned())
            .unwrap_or_else(|| filename.clone()),
        artist: tag
            .artist()
            .map(|s| s.into_owned())
            .unwrap_or_default(),
        album: tag
            .album()
            .map(|s| s.into_owned())
            .unwrap_or_default(),
        length_secs,
        filename,
    })
}