

#[allow(unused_imports)]
#[allow(non_snake_case)]



mod audio;
mod files;

use files::get_files;
use audio::Playing;

#[allow(dead_code)]




fn main(){

  let files = get_files::get_mp3_files();

    
  for file in files{
  let song = audio::Song::create_song_struct(&file);
  Playing::play_song(song);

  }
  
}






  
