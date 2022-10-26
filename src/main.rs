

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
    println!("{}",file);
    create_song_struct(&file);

  }
  





  
}




fn create_song_struct(file_path:&str) {

  let tags = Tag::default().read_from_path(&file_path);

  let
  
  }

  // println!("{:?}",result);
  // // let title = tags.title().unwrap_or("Not found");
  // // let artist = tags.artist().unwrap_or("not found");
  // // let track = tags.track().unwrap_or("not found");
  

  // println!("title:{:?}\nartist:{:?}\ntrack:{:?}",title,artist,track);

