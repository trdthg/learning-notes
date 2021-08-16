extern crate rodio;

use std::fs::File;
use std::io::BufReader;
use std::thread;

use rodio::{Sink, Decoder, OutputStream, source::Source};

 
fn main() {
    let mu = "assets/1.flac";
    // example(mu);
    // sink(mu);
    c(mu);
}

fn example(mu: &str) {

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(mu).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

}

fn sink(mu: &str) {
 
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    
    // // 出于示例原因，添加一个虚拟源。
    // let source = rodio::source::SineWave::new(440);

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(mu).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
     
    sink.append(source);
    std::thread::sleep(std::time::Duration::from_secs(5));
 
}

fn c(mu: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(mu).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device

    let sink1 = rodio::Sink::try_new(&stream_handle).unwrap();
    let thread1 = std::thread::spawn(move || {
        sink1.append(source);
        loop{}
    });
    
    thread1.join().unwrap();
}