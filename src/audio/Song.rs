use audiotags::Tag;
use std::path::Path;
#[derive(Debug)]

pub struct SongStruct {
    pub title: String,
    pub artist: String,
    pub file_path: String,
}

pub fn create_song_struct(file_path: &str) -> SongStruct {
    let tags = Tag::default().read_from_path(file_path);

    let binding = tags.expect("Unable to find Tags");
    let title = binding
        .title()
        .map(|title| title.to_owned())
        .unwrap_or_else(|| {
            Path::new(file_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .into()
        });
    let artist = binding
        .artist()
        .map(|artist| artist.to_owned())
        .unwrap_or_else(|| "Artist Not Found".to_string());

    SongStruct {
        file_path: file_path.to_string(),
        title,
        artist,
    }
}

pub fn song_vector(song_list: Vec<String>) -> Vec<SongStruct> {
    use super::*;
    let mut list: Vec<SongStruct> = Vec::new();

    for file in song_list {
        list.push(create_song_struct(&file))
    }

    list
}
