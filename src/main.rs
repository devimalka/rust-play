extern crate rodio;
use std::fs;
use std::io::BufReader;
use rodio::{Decoder,OutputStream};
use rodio::Sink;

fn main(){
    let paths = fs::read_dir(".").unwrap();
    let mut file_data: Vec<String> = Vec::new();

    for path in paths {
        let file = path.unwrap().path().display().to_string();
        if file.contains("mp3"){
            file_data.push(file);
        }
    }

    for song in file_data{
        play_song(&song);

    }



    

 

}


fn play_song(song:&str){

    let (_stream,stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(fs::File::open(song).unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    println!("Playing {}",&song.replace("_"," "));
    sink.sleep_until_end();



}