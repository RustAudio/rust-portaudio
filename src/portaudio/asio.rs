/*!
* The ASIO specific API.
*/

use std::libc::{c_double, c_char};
use std::str;
use std::ptr;

use types::*;
use ffi;
use pa::*;

pub trait Asio {
    fn set_stream_sample_rate(&self, sampleRate : f64) -> PaError;
}

impl<I, O> Asio for PaStream<I, O> {
    /**
    * Set the sample rate of an open paASIO stream.
    * 
    * # Arguments
    * sample_rate - The new sample rate.
    *
    * Note that this function may fail if the stream is alredy running and the 
    * ASIO driver does not support switching the sample rate of a running stream.
    *
    * Returns PaIncompatibleStreamHostApi if stream is not a PaASIO stream.
    */
        fn set_stream_sample_rate(&self, sample_rate : f64) -> PaError {
        unsafe {
            ffi::PaAsio_SetStreamSampleRate(self.get_c_pa_stream(), sample_rate as c_double)
        }
    }
} 

/**
* Retrieve legal native buffer sizes for the specificed device, in sample frames.
*
* # Arguments
* * device - The global index of the device about which the query is being made.
*/
pub fn get_available_buffer_sizes(device : PaDeviceIndex) -> Result<(i32, i32, i32, i32), PaError> {
    let min_buffer_size_frames : i32 = 0;
    let max_buffer_size_frames : i32 = 0;
    let preferred_buffer_size_frames : i32 = 0;
    let granularity : i32 = 0;
    let mut error : PaError;
    unsafe {
        error = ffi::PaAsio_GetAvailableBufferSizes(device, &min_buffer_size_frames, &max_buffer_size_frames, &preferred_buffer_size_frames, &granularity);
    }

    match error {
        PaNoError   => Ok((min_buffer_size_frames, max_buffer_size_frames, preferred_buffer_size_frames, granularity)),
        _           => Err(error)
    }
}

/**
* Retrieve a string containing the name of the specified input channel. The string is valid until Pa_Terminate is called.
*
* The string will be no longer than 32 characters including the null terminator.
*/
pub fn get_input_channel_name(device : PaDeviceIndex, channel_index : i32) -> Result<~str, PaError> {
    let c_string : *c_char = ptr::null();
    let err = unsafe {
        ffi::PaAsio_GetInputChannelName(device, channel_index, &c_string)
    };
    match err {
        PaNoError   => Ok(unsafe { str::raw::from_c_str(c_string) } ),
        _           => Err(err)
    }
}

/**
* Retrieve a string containing the name of the specified input channel. The string is valid until Pa_Terminate is called.
*
* The string will be no longer than 32 characters including the null terminator.
*/
pub fn get_output_channel_name(device : PaDeviceIndex, channel_index : i32) -> Result<~str, PaError> {
    let c_string : *c_char = ptr::null();
    let err = unsafe {
        ffi::PaAsio_GetOutputChannelName(device, channel_index, &c_string)
    };
    match err {
        PaNoError   => Ok(unsafe { str::raw::from_c_str(c_string) } ),
        _           => Err(err)
    }
}