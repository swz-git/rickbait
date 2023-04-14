use std::io::Cursor;

use rodio::{Decoder, OutputStream, Sink};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let laugh_bin_flac = include_bytes!("../laugh.mp3");
    let cursor = Cursor::new(laugh_bin_flac);
    let source = Decoder::new(cursor).unwrap();

    sink.append(source);

    sink.sleep_until_end();

    opener::open("https://youtu.be/dQw4w9WgXcQ").expect("opening rickroll");
}
