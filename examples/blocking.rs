//!
//! A demonstration of constructing and using a blocking stream.
//!
//! Audio from the default input device is passed directly to the default output device in a duplex
//! stream, so beware of feedback!
//!

extern crate portaudio;

use portaudio::pa;
use std::error::Error;
use std::mem::replace;

const SAMPLE_RATE: f64 = 44_100.0;
const CHANNELS: u32 = 2;
const FRAMES: u32 = 256;

fn main() {

    println!("PortAudio version : {}", pa::get_version());
    println!("PortAudio version text : {}", pa::get_version_text());

    match pa::initialize() {
        Ok(()) => println!("Successfully initialized PortAudio"),
        Err(err) => println!("An error occurred while initializing PortAudio: {}", err.description()),
    }

    println!("PortAudio host count : {}", pa::host::get_api_count() as isize);

    let default_host = pa::host::get_default_api();
    println!("PortAudio default host : {}", default_host as isize);

    match pa::host::get_api_info(default_host) {
        None => println!("Couldn't retrieve api info for the default host."),
        Some(info) => println!("PortAudio host name : {}", info.name),
    }

    let def_input = pa::device::get_default_input();
    let input_info = match pa::device::get_info(def_input) {
        Ok(info) => info,
        Err(err) => panic!("An error occurred while retrieving input info: {}", err.description()),
    };
    println!("Default input device info :");
    println!("\tversion : {}", input_info.struct_version);
    println!("\tname : {}", input_info.name);
    println!("\tmax input channels : {}", input_info.max_input_channels);
    println!("\tmax output channels : {}", input_info.max_output_channels);
    println!("\tdefault sample rate : {}", input_info.default_sample_rate);

    // Construct the input stream parameters.
    let input_stream_params = pa::StreamParameters {
        device : def_input,
        channel_count : CHANNELS as i32,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : input_info.default_low_input_latency
    };

    let def_output = pa::device::get_default_output();
    let output_info = match pa::device::get_info(def_output) {
        Ok(info) => info,
        Err(err) => panic!("An error occurred while retrieving output info: {}", err.description()),
    };

    println!("Default output device name : {}", output_info.name);

    // Construct the output stream parameters.
    let output_stream_params = pa::StreamParameters {
        device : def_output,
        channel_count : CHANNELS as i32,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : output_info.default_low_output_latency
    };

    // Check that the stream format is supported.
    if let Err(err) = pa::is_format_supported(Some(&input_stream_params), Some(&output_stream_params), SAMPLE_RATE) {
        panic!("The given stream format is unsupported: {:?}", err.description());
    }

    let mut stream : pa::Stream<f32, f32> = pa::Stream::new();

    match stream.open(Some(&input_stream_params),
                      Some(&output_stream_params),
                      SAMPLE_RATE,
                      FRAMES,
                      pa::StreamFlags::empty(),
                      None) {
        Ok(()) => println!("Successfully opened the stream."),
        Err(err) => println!("An error occurred while opening the stream: {}", err.description()),
    }

    match stream.start() {
        Ok(()) => println!("Successfully started the stream."),
        Err(err) => println!("An error occurred while starting the stream: {}", err.description()),
    }

    // We'll use this function to wait for read/write availability.
    fn wait_for_stream<F: Fn() -> Result<pa::StreamAvailable, pa::error::Error>>(f: F, name: &str)
        -> u32
    {
        'waiting_for_stream: loop {
            match f() {
                Ok(available) => match available {
                    pa::StreamAvailable::Frames(frames) => return frames as u32,
                    pa::StreamAvailable::InputOverflowed => println!("Input stream has overflowed"),
                    pa::StreamAvailable::OutputUnderflowed => println!("Output stream has underflowed"),
                },
                Err(err) => panic!("An error occurred while waiting for the {} stream: {}", name, err.description()),
            }
        }
    };

    // We'll use this buffer to transfer samples from the input stream to the output stream.
    let mut buffer = Vec::with_capacity((FRAMES * CHANNELS) as usize);

    // Now start the main read/write loop! In this example, we pass the input buffer directly to
    // the output buffer, so watch out for feedback.
    'stream: loop {

        // How many frames are available on the input stream?
        let in_frames = wait_for_stream(|| stream.get_stream_read_available(), "Read");

        // If there are frames available, let's take them and add them to our buffer.
        if in_frames > 0 {
            match stream.read(in_frames) {
                Ok(input_samples) => {
                    buffer.extend(input_samples.into_iter());
                    println!("Read {:?} frames from the input stream.", in_frames);
                },
                Err(err) => {
                    println!("An error occurred while reading from the input stream: {}", err.description());
                    break 'stream
                },
            }
        }

        // How many frames are available for writing on the output stream?
        let out_frames = wait_for_stream(|| stream.get_stream_write_available(), "Write");

        // How many frames do we have so far?
        let buffer_frames = (buffer.len() / CHANNELS as usize) as u32;

        // If there are frames available for writing and we have some to write, then write!
        if out_frames > 0 && buffer_frames > 0 {
            // If we have more than enough frames for writing, take them from the start of the buffer.
            let (write_buffer, write_frames) = if buffer_frames >= out_frames {
                let out_samples = (out_frames * CHANNELS as u32) as usize;
                let remaining_buffer = buffer[out_samples..].iter().map(|&sample| sample).collect();
                buffer.truncate(out_samples);
                let write_buffer = replace(&mut buffer, remaining_buffer);
                (write_buffer, out_frames)
            }
            // Otherwise if we have less, just take what we can for now.
            else {
                let write_buffer = replace(&mut buffer, Vec::with_capacity((FRAMES * CHANNELS) as usize));
                (write_buffer, buffer_frames)
            };
            match stream.write(write_buffer, write_frames) {
                Ok(_) => println!("Wrote {:?} frames to the output stream.", out_frames),
                Err(err) => {
                    println!("An error occurred while writing to the output stream: {}", err.description());
                    break 'stream
                },
            }
        }

    }

    match stream.close() {
        Ok(()) => println!("Successfully closed the stream."),
        Err(err) => println!("An error occurred while closing the stream: {}", err.description()),
    }

    println!("");

    match pa::terminate() {
        Ok(()) => println!("Successfully terminated PortAudio."),
        Err(err) => println!("An error occurred while terminating PortAudio: {}", err.description()),
    }

}
