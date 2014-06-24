#![crate_id = "tests"]
#![feature(globs)]
#![allow(unreachable_code, dead_assignment)]

extern crate portaudio;

use portaudio::*;

fn main() -> () {
    println!("Portaudio version : {}", pa::get_version());
    println!("Portaudio version text : {}", pa::get_version_text());
    println!("Portaudio error text : {}", pa::get_error_text(types::PaNotInitialized));

    println!("Portaudio init error : {}", pa::get_error_text(pa::initialize()));

    let host_count = pa::get_host_api_count();
    println!("Portaudio host count : {}", host_count as int);

    let default_host = pa::get_default_host_api();
    println!("Portaudio default host : {}", default_host as int);

    let host_info = pa::get_host_api_info(default_host);
    println!("Portaudio host name : {}", host_info.unwrap().name);

    println!("Portaudio type id : {}",
             pa::host_api_type_id_to_host_api_index(types::PaCoreAudio) as int);

    let def_input = pa::get_default_input_device();
    let info_input = pa::get_device_info(def_input).unwrap();
    println!("Default input device info :");
    println!("version : {}", info_input.struct_version);
    println!("name : {}", info_input.name);
    println!("max input channels : {}", info_input.max_input_channels);
    println!("max output channels : {}", info_input.max_output_channels);
    println!("default sample rate : {}", info_input.default_sample_rate);


    if pa::get_device_info(def_input).is_none() {
       println!("error");
    }
    // PaStream test :
    let stream_params  = types::PaStreamParameters {
        device : def_input,
        channel_count : 2,
        sample_format : types::PaFloat32,
        suggested_latency : pa::get_device_info(def_input).unwrap().default_low_input_latency
    };

    let def_output = pa::get_default_output_device();
    println!("name : {}", pa::get_device_info(def_output).unwrap().name);

    let stream_params_out = types::PaStreamParameters {
        device : def_output,
        channel_count : 2,
        sample_format : types::PaFloat32,
        suggested_latency : pa::get_device_info(def_output).unwrap().default_low_output_latency
    };


    let mut stream : pa::PaStream<f32> = pa::PaStream::new(types::PaFloat32);

    let mut err= stream.open(Some(&stream_params), Some(&stream_params_out), 44100., 1024, types::PaClipOff);

    println!("Portaudio Open error : {}", pa::get_error_text(err));

    //println!("Stream is active : {}", pa::get_error_text(stream.is_active().unwrap()));

    err = stream.start();
    println!("Portaudio Start error : {}", pa::get_error_text(err));

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
            Err(err)    => fail!(format!("Portaudio error read : {}", pa::get_error_text(err)))
        };
    }

    err = types::PaNotInitialized;

    err = stream.close();
    println!("Portaudio Close stream error : {}", pa::get_error_text(err));

    println!("");


    println!("Portaudio terminate error : {}", pa::get_error_text(pa::terminate()));
}
