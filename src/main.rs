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
    // ui::ui();
    let files = get_files::get_mp3_files();

    let songs = audio::Song::song_vector(files);
    ui::ui(songs);
    // for song in songs{
    //   println!("{0}",song.title);
    // }
}
