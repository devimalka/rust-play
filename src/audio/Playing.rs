use std::fs;
use std::io::BufReader;
use rodio::{Decoder,OutputStream};
use rodio::Sink;

use crate::audio;
use audio::Song::SongStruct;


    pub fn play_song(song:SongStruct){

       
    let  (_stream,stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(fs::File::open(song.file_path).unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    println!("Playing {0}",song.title);
    sink.sleep_until_end();




}
