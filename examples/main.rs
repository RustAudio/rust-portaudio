#![feature(globs)]
#![allow(unreachable_code, unused_assignments)]

extern crate portaudio;

use portaudio::*;

fn main() -> () {
    println!("Portaudio version : {}", pa::get_version());
    println!("Portaudio version text : {}", pa::get_version_text());
    // println!("Portaudio error text : {}", pa::get_error_text(error::Error::NotInitialized));

    pa::initialize();
    // println!("Portaudio init error : {}", pa::get_error_text(pa::initialize()));

    let host_count = pa::host::get_api_count();
    println!("Portaudio host count : {}", host_count as int);

    let default_host = pa::host::get_default_api();
    println!("Portaudio default host : {}", default_host as int);

    let host_info = pa::host::get_api_info(default_host);
    println!("Portaudio host name : {}", host_info.unwrap().name);

    println!("Portaudio type id : {}",
             pa::host::api_type_id_to_host_api_index(pa::HostApiTypeId::CoreAudio) as int);

    let def_input = pa::device::get_default_input();
    let info_input = pa::device::get_info(def_input).unwrap();
    println!("Default input device info :");
    println!("version : {}", info_input.struct_version);
    println!("name : {}", info_input.name);
    println!("max input channels : {}", info_input.max_input_channels);
    println!("max output channels : {}", info_input.max_output_channels);
    println!("default sample rate : {}", info_input.default_sample_rate);


    if pa::device::get_info(def_input).is_none() {
       println!("error");
    }
    // PaStream test :
    let stream_params  = pa::StreamParameters {
        device : def_input,
        channel_count : 2,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : pa::device::get_info(def_input).unwrap().default_low_input_latency
    };

    let def_output = pa::device::get_default_output();
    println!("name : {}", pa::device::get_info(def_output).unwrap().name);

    let stream_params_out = pa::StreamParameters {
        device : def_output,
        channel_count : 2,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : pa::device::get_info(def_output).unwrap().default_low_output_latency
    };


    let mut stream : pa::Stream<f32, f32> = pa::Stream::new();

    let mut err= stream.open(Some(&stream_params), Some(&stream_params_out), 44100., 1024, pa::StreamFlags::ClipOff);

    // println!("Portaudio Open error : {}", pa::get_error_text(err));

    //println!("Stream is active : {}", pa::get_error_text(stream.is_active().unwrap()));

    err = stream.start();
    // println!("Portaudio Start error : {}", pa::get_error_text(err));

    let mut test;
    loop {
        test = stream.get_stream_write_available();
        while test == 0 {
            test = stream.get_stream_write_available();
        }
            println!("Stream Write available : {}", test as int);

        match stream.read(1024) {
            Ok(res)     => {
                // for i in res.iter() {
                //     io::println(fmt!("%f", *i as float));
                // }
                stream.write(res, 1024)
            },
            Err(err)    => panic!(format!("Portaudio error read : {}", pa::get_error_text(err)))
        };
    }

    err = Ok(());

    err = stream.close();
    // println!("Portaudio Close stream error : {}", pa::get_error_text(err));

    println!("");

    pa::terminate();
    // println!("Portaudio terminate error : {}", pa::get_error_text(pa::terminate()));
}
