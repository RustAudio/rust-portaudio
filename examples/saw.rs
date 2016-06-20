//! Play a sawtooth wave for several seconds.
//!
//! A rusty adaptation of the official PortAudio C "paex_saw.c" example by Phil Burk and Ross
//! Bencina.

extern crate portaudio;

use portaudio as pa;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn main() {
    run().unwrap()
}


fn run() -> Result<(), pa::Error> {

    println!("PortAudio Test: output sawtooth wave. SR = {}, BufSize = {}", SAMPLE_RATE, FRAMES_PER_BUFFER);

    let mut left_saw = 0.0;
    let mut right_saw = 0.0;

    let pa = try!(pa::PortAudio::new());

    let mut settings = try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
    // we won't output out of range samples so don't bother clipping them.
    settings.flags = pa::stream_flags::CLIP_OFF;

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            buffer[idx] = left_saw;
            buffer[idx+1] = right_saw;
            left_saw += 0.01;
            if left_saw >= 1.0 { left_saw -= 2.0; }
            right_saw += 0.03;
            if right_saw >= 1.0 { right_saw -= 2.0; }
            idx += 2;
        }
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    println!("Play for {} seconds.", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    try!(stream.stop());
    try!(stream.close());

    println!("Test finished.");

    Ok(())
}
