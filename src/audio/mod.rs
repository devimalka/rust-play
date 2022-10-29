use std::fs;
use std::io::BufReader;
use rodio::{Decoder,OutputStream};
use rodio::Sink;

pub mod playing{

    use super::*;
    pub fn play_song(song:&str){

       
    let  (_stream,stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(fs::File::open(song).unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    // println!("Playing {}",&song.replace("_"," "));
    sink.sleep_until_end();




}
}