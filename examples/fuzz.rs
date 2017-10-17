//! Distort input like a fuzz box
//!
//! Audio from the default input device is passed through a filter and
//! then directly to the default output device in a duplex stream, so
//! beware of feedback!

extern crate portaudio;

use portaudio as pa;


const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 64;
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


// the fuzz filter, when applied to all samples, will add some
// distortion
fn fuzz(input: f32) -> f32 {
    (0..4).fold(input, |acc, _| cubic_amplifier(acc))
}

fn cubic_amplifier(input: f32) -> f32 {
    // samples should be between -1.0 and 1.0
    if input < 0.0 {
        // if it's negative (-1.0 to 0), then adding 1.0 takes it to
        // the 0 to 1.0 range. If it's cubed, it still won't leave the
        // 0 to 1.0 range.
        let negated = input + 1.0;
        (negated * negated * negated) - 1.0
    } else {
        // if it's positive (0 to 1.0), then subtracting 1.0 takes it
        // to the -1.0 to 0 range. If it's cubed, it still won't leave
        // the -1.0 to 0 range.
        let negated = input - 1.0;
        (negated * negated * negated) + 1.0
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

    let def_output = try!(pa.default_output_device());
    let output_info = try!(pa.device_info(def_output));
    println!("Default output device info: {:#?}", &output_info);

    // Construct the output stream parameters.
    let latency = output_info.default_low_output_latency;
    let output_params = pa::StreamParameters::new(def_output, CHANNELS, INTERLEAVED, latency);

    // Check that the stream format is supported.
    try!(pa.is_duplex_format_supported(input_params, output_params, SAMPLE_RATE));

    // Construct the settings with which we'll open our duplex stream.
    let settings = pa::DuplexStreamSettings::new(input_params, output_params, SAMPLE_RATE, FRAMES);

    // Once the countdown reaches 0 we'll close the stream.
    let mut count_down = 3.0;

    // Keep track of the last `current_time` so we can calculate the delta time.
    let mut maybe_last_time = None;

    // We'll use this channel to send the count_down to the main thread for fun.
    let (sender, receiver) = ::std::sync::mpsc::channel();

    // A callback to pass to the non-blocking stream.
    let callback = move |pa::DuplexStreamCallbackArgs { in_buffer, out_buffer, frames, time, .. }| {
        let current_time = time.current;
        let prev_time = maybe_last_time.unwrap_or(current_time);
        let dt = current_time - prev_time;
        count_down -= dt;
        maybe_last_time = Some(current_time);

        assert!(frames == FRAMES as usize);
        sender.send(count_down).ok();

        // Pass the input through the fuzz filter and then to the output
        // BEWARE OF FEEDBACK!
        for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
            *output_sample = fuzz(*input_sample);
        }

        if count_down > 0.0 { pa::Continue } else { pa::Complete }
    };

    // Construct a stream with input and output sample types of f32.
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    // Loop while the non-blocking stream is active.
    while let true = try!(stream.is_active()) {

        // Watch the countdown while we wait for the stream to finish
        while let Ok(count_down) = receiver.try_recv() {
            println!("count_down: {:?}", count_down);
        }

    }

    try!(stream.stop());

    Ok(())
}
