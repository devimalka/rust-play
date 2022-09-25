
#[allow(unused_imports)]
use rust_play::audio::playing;
use rust_play::files::get_files;

fn main(){

  let files = get_files::get_mp3_files();

    
  for file in files{
    println!("{}",file);
  }




}




