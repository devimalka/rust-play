#[allow(unused_imports)]
#[allow(non_snake_case)]
mod audio;
mod files;
mod terminalui;

use audio::Playing;
use files::get_files;
use terminalui::ui;
#[allow(dead_code)]

fn main() {
    let files = get_files::get_mp3_files();

    let songs = audio::Song::song_vector(files);

    for song in songs{
        Playing::play_song(song);
    }

}