#![feature(core)]

extern crate portaudio;

use portaudio::pa;
use std::error::Error;
use std::num::Float;
use std::f64::consts::PI_2;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: usize = 256;
const DELTATIME: f64 = 1.0 / SAMPLE_RATE;

fn init() -> pa::Stream<f32, f32>{
    match pa::initialize() {
        Ok(()) => println!("Successfully initialized PortAudio"),
        Err(err) => println!("An error occurred while initializing PortAudio: {}", err.description()),
    }

    let def_output = pa::device::get_default_output();
    let output_info = match pa::device::get_info(def_output) {
        Ok(info) => info,
        Err(err) => panic!("An error occurred while retrieving output info: {}", err.description()),
    };
    println!("Default output device name : {}", output_info.name);
    let out_params = pa::StreamParameters {
        device : def_output,
        channel_count : 2,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : output_info.default_low_output_latency
    };
    let mut stream : pa::Stream<f32, f32> = pa::Stream::new();
    match stream.open(None,
                      Some(&out_params),
                      SAMPLE_RATE,
                      FRAMES as u32,
                      pa::StreamFlags::ClipOff) {
        Ok(()) => println!("Successfully opened the stream."),
        Err(err) => println!("An error occurred while opening the stream: {}", err.description()),
    }

    match stream.start() {
        Ok(()) => println!("Successfully started the stream."),
        Err(err) => println!("An error occurred while starting the stream: {}", err.description()),
    }
    stream
}


fn drop(mut stream: pa::Stream<f32, f32>) {
    match stream.close() {
        Ok(()) => println!("Successfully closed the stream."),
        Err(err) => println!("An error occurred while closing the stream: {}", err.description()),
    }
    match pa::terminate() {
        Ok(()) => println!("Successfully terminated PortAudio."),
        Err(err) => println!("An error occurred while terminating PortAudio: {}", err.description()),
    }
}


fn standard_pitch(time: f64, left: &mut f32, right: &mut f32) {
    let a440 = 440.0;
    let volume = 0.1;

    *left = (Float::sin(time*a440*PI_2) * volume) as f32;
    *right = (Float::sin(time*a440*PI_2) * volume) as f32;
}


fn generate(stream: &pa::Stream<f32, f32>, generator: &Fn(f64, &mut f32, &mut f32)){
    let mut time = 0.0;
    let mut buffer = [0.0; FRAMES*2];

    loop {
        {
            let mut iter = buffer.iter_mut();
            while let (Some(left), Some(right)) = (iter.next(), iter.next()) {
                time += DELTATIME;
                generator(time, left, right);
            }
        }
        match stream.write(&buffer[..], FRAMES as u32) {
            Ok(()) => (),
            Err(err) => {
                println!("Error when writing to the stream: {:?}", err);
                break;
            }
        }
    }
}


fn main() {
    let stream = init();
    generate(&stream, &standard_pitch);
    drop(stream);
}
