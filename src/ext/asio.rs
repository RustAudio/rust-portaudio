// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
    fn set_stream_sample_rate(&self, sampleRate : f64) -> Error;
}

impl<I, O> Asio for Stream<I, O> {
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
pub fn get_input_channel_name(device : PaDeviceIndex, channel_index : i32) -> Result<String, PaError> {
    let c_string : *const c_char = ptr::null();
    let err = unsafe {
        ffi::PaAsio_GetInputChannelName(device, channel_index, &mut c_string)
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
pub fn get_output_channel_name(device : PaDeviceIndex, channel_index : i32) -> Result<String, PaError> {
    let c_string : *const c_char = ptr::null();
    let err = unsafe {
        ffi::PaAsio_GetOutputChannelName(device, channel_index, &mut c_string)
    };
    match err {
        PaNoError   => Ok(unsafe { str::raw::from_c_str(c_string) } ),
        _           => Err(err)
    }
}
