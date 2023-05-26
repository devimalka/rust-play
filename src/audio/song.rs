use rodio::{Decoder, OutputStream, Sink, StreamError};
use std::{fs::File, io::BufReader};

#[allow(dead_code)]
pub struct SinkStruct {
    sink: Sink,
    stream: OutputStream,
}

impl SinkStruct {
    // create a new Sink Struct and Return
    pub fn new() -> Result<Self, StreamError> {
        let (stream, _stream_handle) = match OutputStream::try_default() {
            Ok((stream, streamhandle)) => (stream, streamhandle),
            Err(e) => {
                println!("Couldn't find Stream {}", e);
                return Err(e);
            }
        };

        let sink = Sink::try_new(&_stream_handle).unwrap();

        Ok(SinkStruct { sink, stream })
    }

    // append a source to sink
    pub fn append(&self, path: &str) {
        let file = BufReader::new(File::open(path).unwrap());

        let source = Decoder::new(file).unwrap();

        self.sink.append(source);
    }

    // play the source from sink
    pub fn play(&self) {
        self.sink.play();
        self.sink.sleep_until_end();
    }

}
