//! A demonstration of constructing and using a blocking stream.
//!
//! Audio from the default input device is passed directly to the default output device in a duplex
//! stream, so beware of feedback!

extern crate portaudio;

use portaudio as pa;
use std::collections::VecDeque;


const SAMPLE_RATE: f64 = 44_100.0;
const CHANNELS: i32 = 2;
const FRAMES: u32 = 256;
const INTERLEAVED: bool = true;


fn main() {
    run().unwrap()
}

fn run() -> Result<(), pa::Error> {

    let pa = try!(pa::PortAudio::new());

    println!("PortAudio");
    println!("version: {}", pa.version());
    println!("version text: {:?}", pa.version_text());
    println!("host count: {}", try!(pa.host_api_count()));

    let default_host = try!(pa.default_host_api());
    println!("default host: {:#?}", pa.host_api_info(default_host));

    let def_input = try!(pa.default_input_device());
    let input_info = try!(pa.device_info(def_input));
    println!("Default input device info: {:#?}", &input_info);

    // Construct the input stream parameters.
    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<f32>::new(def_input, CHANNELS, INTERLEAVED, latency);

    let def_output = try!(pa.default_output_device());
    let output_info = try!(pa.device_info(def_output));
    println!("Default output device info: {:#?}", &output_info);

    // Construct the output stream parameters.
    let latency = output_info.default_low_output_latency;
    let output_params = pa::StreamParameters::<f32>::new(def_output, CHANNELS, INTERLEAVED, latency);

    // Check that the stream format is supported.
    try!(pa.is_duplex_format_supported(input_params, output_params, SAMPLE_RATE));

    // Construct the settings with which we'll open our duplex stream.
    let settings = pa::DuplexStreamSettings {
        in_params: input_params,
        out_params: output_params,
        sample_rate: SAMPLE_RATE,
        frames_per_buffer: FRAMES,
        flags: pa::StreamFlags::empty(),
    };

    let mut stream = try!(pa.open_blocking_stream(settings));

    // We'll use this buffer to transfer samples from the input stream to the output stream.
    let mut buffer: VecDeque<f32> = VecDeque::with_capacity(FRAMES as usize * CHANNELS as usize);

    try!(stream.start());

    // We'll use this function to wait for read/write availability.
    fn wait_for_stream<F>(f: F, name: &str) -> u32
        where F: Fn() -> Result<pa::StreamAvailable, pa::error::Error>
    {
        'waiting_for_stream: loop {
            match f() {
                Ok(available) => match available {
                    pa::StreamAvailable::Frames(frames) => return frames as u32,
                    pa::StreamAvailable::InputOverflowed => println!("Input stream has overflowed"),
                    pa::StreamAvailable::OutputUnderflowed => println!("Output stream has underflowed"),
                },
                Err(err) => panic!("An error occurred while waiting for the {} stream: {}", name, err),
            }
        }
    };

    // Now start the main read/write loop! In this example, we pass the input buffer directly to
    // the output buffer, so watch out for feedback.
    'stream: loop {

        // How many frames are available on the input stream?
        let in_frames = wait_for_stream(|| stream.read_available(), "Read");

        // If there are frames available, let's take them and add them to our buffer.
        if in_frames > 0 {
            let input_samples = try!(stream.read(in_frames));
            buffer.extend(input_samples.into_iter());
            println!("Read {:?} frames from the input stream.", in_frames);
        }

        // How many frames are available for writing on the output stream?
        let out_frames = wait_for_stream(|| stream.write_available(), "Write");

        // How many frames do we have so far?
        let buffer_frames = (buffer.len() / CHANNELS as usize) as u32;

        // If there are frames available for writing and we have some to write, then write!
        if out_frames > 0 && buffer_frames > 0 {

            // If we have more than enough frames for writing, take them from the start of the buffer.
            // Otherwise if we have less, just take what we can for now.
            let write_frames = if buffer_frames >= out_frames { out_frames } else { buffer_frames };
            let n_write_samples = write_frames as usize * CHANNELS as usize;

            try!(stream.write(write_frames, |output| {
                for i in 0..n_write_samples {
                    output[i] = buffer.pop_front().unwrap();
                }
                println!("Wrote {:?} frames to the output stream.", out_frames);
            }));
        }

    }

}
