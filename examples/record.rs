//! A demonstration of recording input
//!
//! Audio from the default input device is recorded into memory until
//! the user presses Enter. They are then played back to the default
//! output device.

extern crate portaudio;

use portaudio as pa;
use std::io;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 256;
const CHANNELS: i32 = 2;
const INTERLEAVED: bool = true;


fn main() {
    match run() {
        Ok(_) => {},
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), pa::Error> {

    let pa = try!(pa::PortAudio::new());

    println!("PortAudio:");
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

    // Check that the stream format is supported.
    try!(pa.is_input_format_supported(input_params, SAMPLE_RATE));

    // Construct the settings with which we'll open our input stream.
    let input_settings = pa::InputStreamSettings::new(input_params, SAMPLE_RATE, FRAMES);

    // We'll use this channel to send the samples back to the main thread.
    let (sender, receiver) = ::std::sync::mpsc::channel();

    // A callback to pass to the non-blocking input stream.
    let input_callback = move |pa::InputStreamCallbackArgs { buffer, frames, .. }| {
        assert!(frames == FRAMES as usize);

        // We'll construct a copy of the input buffer and send that
        // onto the channel. This doesn't block, even though nothing
        // is waiting on the receiver yet.
        let vec_buffer = Vec::from(buffer);
        // There are actually 512 samples here. 256 for the left, 256 for the right.
        assert!(vec_buffer.len() == FRAMES as usize * CHANNELS as usize);

        // If sending fails (the receiver has been dropped), stop recording
        match sender.send(vec_buffer) {
            Ok(_) => pa::Continue,
            Err(_) => pa::Complete
        }
    };

    // Construct a stream with input sample types of f32.
    let mut input_stream = try!(pa.open_non_blocking_stream(input_settings, input_callback));

    try!(input_stream.start());

    println!("Recording has started. Press Enter to stop.");
    
    // Wait for enter to be pressed
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();

    try!(input_stream.stop());


    // We now have a channel filled with the samples from the input
    // device. Let's pipe it all to the output device.

    
    let def_output = try!(pa.default_output_device());
    let output_info = try!(pa.device_info(def_output));
    println!("Default output device info: {:#?}", &output_info);
    
    // Construct the output stream parameters.
    let latency = output_info.default_low_output_latency;
    let output_params = pa::StreamParameters::new(def_output, CHANNELS, INTERLEAVED, latency);

    // Check that the stream format is supported.
    try!(pa.is_output_format_supported(output_params, SAMPLE_RATE));

    // Construct the settings with which we'll open our output stream.
    let output_settings = pa::OutputStreamSettings::new(output_params, SAMPLE_RATE, FRAMES);

    // A callback to pass to the non-blocking output stream.
    let output_callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        // like with the input, frames is the number of samples that
        // buffer expects per channel
        assert!(frames == FRAMES as usize);

        // try_recv will return immediately, with an error if there
        // isn't any data waiting. This is reading in the data that we
        // sent from the input callback.
        match receiver.try_recv() {
            Ok(samples) => {
                assert!(samples.len() == FRAMES as usize * CHANNELS as usize);

                // Pass the previous input over to the output. No
                // feedback issues in this example, since we aren't
                // recording anymore at this point.
                for (output_sample, input_sample) in buffer.iter_mut().zip(samples.iter()) {
                    *output_sample = *input_sample;
                }
                pa::Continue
            },
            Err(_) => pa::Complete
        }
    };

    // Construct a stream with output sample types of f32.
    let mut output_stream = try!(pa.open_non_blocking_stream(output_settings, output_callback));

    try!(output_stream.start());
    
    println!("Playback has started. Press Enter to stop.");

    // Wait for enter to be pressed
    io::stdin().read_line(&mut buffer).ok();

    try!(output_stream.stop());
    
    Ok(())
}
