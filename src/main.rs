

#[allow(unused_imports)]
#[allow(non_snake_case)]

use rust_play::audio::playing;
use rust_play::files::get_files;
use audiotags::Tag;
use std::io::ErrorKind;

#[allow(dead_code)]
#[derive(Debug)]
struct Song{
    file_path:String,
    title:String,
    artist:String,
    song_name:String,
}

fn main(){

  let files = get_files::get_mp3_files();

    
  for file in files{
  let song =    create_song_struct(&file);
  println!("{0}=>{1}",song.title,song.artist);
  playing::play_song(&song.file_path);

  }
  
}




fn create_song_struct(file_path:&str) -> Song{

  let tags = Tag::default().read_from_path(&file_path);

  // let title = tags.expect("unable to found title").title().unwrap_or("no found");
  let binding = tags.expect("unable to found title");
  let title = binding.title().unwrap_or("no found");
  let artist = binding.artist().unwrap_or("not found artist");
 
  Song{
    file_path:file_path.to_string(),
    title:title.to_string(),
    artist:artist.to_string(),
    song_name:title.to_string(),
  }
  
  }

  
