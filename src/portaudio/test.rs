extern mod portaudio;

use std::io;

use portaudio::*;


fn main() -> () {
	io::println(fmt!("Portaudio version : %d", pa::get_version() as int));
	io::println(fmt!("Portaudio version text : %s", pa::get_version_text()));
	io::println(fmt!("Portaudio error text : %s", pa::get_error_text(types::PaNotInitialized)));

	io::println(fmt!("Portaudio init error : %s", pa::get_error_text(pa::initialize())));
	
	let host_count = pa::get_host_api_count();
	io::println(fmt!("Portaudio host count : %d", host_count as int));

	let default_host = pa::get_default_host_api();
	io::println(fmt!("Portaudio default host : %d", default_host as int));

	let host_info = pa::get_host_api_info(default_host);
	io::println(fmt!("Portaudio host name : %s", host_info.unwrap().name));

	io::println(fmt!("Portaudio type id : %d",
					pa::host_api_type_id_to_host_api_index(types::PaCoreAudio) as int));

	let def_input = pa::get_default_input_device();
	let info_input = pa::get_device_info(def_input).unwrap();
	io::println("Default input device info :");
	io::println(fmt!("version : %d", info_input.struct_version));
	io::println(fmt!("name : %s", info_input.name));
	io::println(fmt!("max input channels : %d", info_input.max_input_channels));
	io::println(fmt!("max output channels : %d", info_input.max_output_channels));
	io::println(fmt!("default sample rate : %f", info_input.default_sample_rate as float));


	if pa::get_device_info(def_input).is_none() {
		io::println("BOULET");
	}
	// PaStream test :
	let stream_params  = types::PaStreamParameters {
		device : def_input,
		channel_count : 2,
		sample_format : types::PaFloat32,
		suggested_latency : pa::get_device_info(def_input).unwrap().default_low_input_latency
	};

	let def_output = pa::get_default_output_device();
	io::println(fmt!("name : %s", pa::get_device_info(def_output).unwrap().name));

	let stream_params_out = types::PaStreamParameters {
		device : def_output,
		channel_count : 2,
		sample_format : types::PaFloat32,
		suggested_latency : pa::get_device_info(def_output).unwrap().default_low_output_latency
	};


	let mut stream = pa::PaStream::new(types::PaFloat32);

	let mut err= stream.open_stream(Some(&stream_params), Some(&stream_params_out), 44100., 1024, types::PaClipOff);

	io::println(fmt!("Portaudio Open error : %s", pa::get_error_text(err)));

	io::println(fmt!("Stream is active : %s", pa::get_error_text(stream.is_active())));

	err = stream.start();
	io::println(fmt!("Portaudio Start error : %s", pa::get_error_text(err)));

 	let mut test;
 	loop {
 		test = stream.get_stream_write_available();
 		while test == 0 {
 			test = stream.get_stream_write_available();
		}
			io::println(fmt!("Stream Write available : %d", test as int)); 

 		match stream.read::<f32>(1024) {
 			Ok(res)		=> {
 				for i in res.iter() {
 					io::println(fmt!("%f", *i as float));
 				}
 				stream.write::<f32>(res, 1024)
 			},
 			Err(err) 	=> fail!(fmt!("Portaudio error read : %s", pa::get_error_text(err)))
 		};
 	}

    err = types::PaNotInitialized;

    err = stream.close_stream();
	io::println(fmt!("Portaudio Close stream error : %s", pa::get_error_text(err)));

	io::println("");


	io::println(fmt!("Portaudio terminate error : %s", pa::get_error_text(pa::terminate())));
}