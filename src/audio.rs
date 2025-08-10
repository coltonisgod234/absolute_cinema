use std::fs::File;
use rodio::{Decoder, OutputStreamBuilder, OutputStream};

pub fn start_audio(path: &str) -> Result<OutputStream, Box<dyn std::error::Error>> {
    let stream_handle: OutputStream = OutputStreamBuilder::open_default_stream()?;

    let file: File = File::open(path)?;
    let source: Decoder<std::io::BufReader<File>> = Decoder::try_from(file)?;

    stream_handle.mixer().add(source);

    Ok(stream_handle)  // return it so caller holds it alive
}
