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

#![allow(dead_code, non_camel_case_types)]

use {StreamInfo, Time, StreamCallbackResult};
use error::Error;
use libc::{c_char, c_double, c_ulong, c_void};


pub type DeviceIndex = i32;
pub type HostApiIndex = i32;
pub const PA_NO_DEVICE : DeviceIndex = -1;

// Sample format
pub type SampleFormat = u64;
pub const PA_FLOAT_32        : SampleFormat = 0x00000001;
pub const PA_INT_32          : SampleFormat = 0x00000002;
pub const PA_INT_24          : SampleFormat = 0x00000004;
pub const PA_INT_16          : SampleFormat = 0x00000008;
pub const PA_INT_8           : SampleFormat = 0x00000010;
pub const PA_UINT_8          : SampleFormat = 0x00000020;
pub const PA_CUSTOM_FORMAT   : SampleFormat = 0x00010000;
pub const PA_NON_INTERLEAVED : SampleFormat = 0x80000000;

// Stream flags
pub type StreamFlags = u64;
pub const PA_NO_FLAG                                    : StreamFlags = 0;
pub const PA_CLIP_OFF                                   : StreamFlags = 0x00000001;
pub const PA_DITHER_OFF                                 : StreamFlags = 0x00000002;
pub const PA_NEVER_DROP_INPUT                           : StreamFlags = 0x00000004;
pub const PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK : StreamFlags = 0x00000008;
pub const PA_PLATFORM_SPECIFIC_FLAGS                    : StreamFlags = 0xFFFF0000;

// Stream callback falgs.
pub type StreamCallbackFlags = u64;
pub const INPUT_UNDERFLOW  : StreamCallbackFlags = 0x00000001;
pub const INPUT_OVERFLOW   : StreamCallbackFlags = 0x00000002;
pub const OUTPUT_UNDERFLOW : StreamCallbackFlags = 0x00000004;
pub const OUTPUT_OVERFLOW  : StreamCallbackFlags = 0x00000008;
pub const PRIMING_OUTPUT   : StreamCallbackFlags = 0x00000010;


/// Unchanging unique identifiers for each supported host API
pub type PaHostApiIndex = ::std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum PaHostApiTypeId {
    paInDevelopment = 0,
    paDirectSound = 1,
    paMME = 2,
    paASIO = 3,
    paSoundManager = 4,
    paCoreAudio = 5,
    paOSS = 7,
    paALSA = 8,
    paAL = 9,
    paBeOS = 10,
    paWDMKS = 11,
    paJACK = 12,
    paWASAPI = 13,
    paAudioScienceHPI = 14,
}


#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct C_PaStreamCallbackTimeInfo {
    pub input_buffer_adc_time: Time,
    pub current_time: Time,
    pub output_buffer_dac_time: Time
}


pub type C_PaStream = c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct C_PaStreamParameters {
    pub device : DeviceIndex,
    pub channel_count : i32,
    pub sample_format : SampleFormat,
    pub suggested_latency : Time,
    pub host_api_specific_stream_info : *mut c_void
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct C_PaDeviceInfo {
    pub struct_version: i32,
    pub name: *const c_char,
    pub host_api: HostApiIndex,
    pub max_input_channels: i32,
    pub max_output_channels: i32,
    pub default_low_input_latency: Time,
    pub default_low_output_latency: Time,
    pub default_high_input_latency: Time,
    pub default_high_output_latency: Time,
    pub default_sample_rate: c_double
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct C_PaHostErrorInfo {
    pub error_code: u32,
    pub error_text: *const c_char
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct C_PaHostApiInfo {
    pub struct_version: i32,
    pub host_type: PaHostApiTypeId,
    pub name: *const c_char,
    pub device_count: i32,
    pub default_input_device: i32,
    pub default_output_device: i32
}

pub type C_PaStreamCallbackFn =
    extern "C" fn(*const c_void,
                  *mut c_void,
                  c_ulong,
                  *const C_PaStreamCallbackTimeInfo,
                  StreamCallbackFlags,
                  *mut c_void) -> StreamCallbackResult;


extern "C" {

    /// PortAudio portable API

    pub fn Pa_GetVersion() -> i32;
    pub fn Pa_GetVersionText() -> *const c_char;
    pub fn Pa_GetErrorText(errorCode : Error) -> *const c_char;
    pub fn Pa_Initialize() -> Error;
    pub fn Pa_Terminate() -> Error;
    pub fn Pa_GetHostApiCount() -> HostApiIndex;
    pub fn Pa_GetDefaultHostApi() -> HostApiIndex;
    pub fn Pa_GetHostApiInfo(hostApi : HostApiIndex) -> *const C_PaHostApiInfo;
    pub fn Pa_HostApiTypeIdToHostApiIndex(type_id : PaHostApiTypeId) -> HostApiIndex;
    pub fn Pa_HostApiDeviceIndexToDeviceIndex(hostApi : HostApiIndex, hostApiDeviceIndex : i32) -> DeviceIndex;
    pub fn Pa_GetLastHostErrorInfo() -> *const C_PaHostErrorInfo;
    pub fn Pa_GetDeviceCount() -> DeviceIndex;
    pub fn Pa_GetDefaultInputDevice() -> DeviceIndex;
    pub fn Pa_GetDefaultOutputDevice() -> DeviceIndex;
    pub fn Pa_GetDeviceInfo(device : DeviceIndex) -> *const C_PaDeviceInfo;
    pub fn Pa_IsFormatSupported(input_parameters : *const C_PaStreamParameters, outputParameters : *const C_PaStreamParameters, sampleRate : c_double) -> Error;
    pub fn Pa_GetSampleSize(format : SampleFormat) -> i32;
    pub fn Pa_Sleep(msec : i32) -> ();
    pub fn Pa_OpenStream(stream : *mut *mut C_PaStream,
                         inputParameters : *const C_PaStreamParameters,
                         outputParameters : *const C_PaStreamParameters,
                         sampleRate : c_double,
                         framesPerBuffer : u32,
                         streamFlags : StreamFlags,
                         streamCallback : Option<C_PaStreamCallbackFn>,
                         userData : *mut c_void)
                         -> Error;
    pub fn Pa_OpenDefaultStream(stream : *mut *mut C_PaStream,
                                numInputChannels : i32,
                                numOutputChannels : i32,
                                sampleFormat : SampleFormat,
                                sampleRate : c_double,
                                framesPerBuffer : u32,
                                streamCallback : Option<C_PaStreamCallbackFn>,
                                userData : *mut c_void)
                                -> Error;
    pub fn Pa_CloseStream(stream : *mut C_PaStream) -> Error;
    //pub fn Pa_SetStreamFinishedCallback (stream : *PaStream, PaStreamFinishedCallback *streamFinishedCallback) -> Error;
    pub fn Pa_StartStream(stream : *mut C_PaStream) -> i32;
    pub fn Pa_StopStream(stream : *mut C_PaStream) -> i32;
    pub fn Pa_AbortStream(stream : *mut C_PaStream) -> i32;
    pub fn Pa_IsStreamStopped(stream : *mut C_PaStream) -> i32;
    pub fn Pa_IsStreamActive(stream : *mut C_PaStream) -> i32;
    pub fn Pa_GetStreamInfo(stream : *mut C_PaStream) -> *const StreamInfo;
    pub fn Pa_GetStreamTime(stream : *mut C_PaStream) -> Time;
    pub fn Pa_GetStreamCpuLoad(stream : *mut C_PaStream) -> c_double;
    pub fn Pa_ReadStream(stream : *mut C_PaStream, buffer : *mut c_void, frames : u32) -> i32;
    pub fn Pa_WriteStream(stream : *mut C_PaStream, buffer : *mut c_void, frames : u32) -> i32;
    pub fn Pa_GetStreamReadAvailable(stream : *mut C_PaStream) -> i64;
    pub fn Pa_GetStreamWriteAvailable(stream : *mut C_PaStream) -> i64;

    // PortAudio Specific ASIO
    pub fn PaAsio_GetAvailableBufferSizes(device : DeviceIndex, minBufferSizeFrames : *mut i32, maxBufferSizeFrames : *mut i32, preferredBufferSizeFrames : *mut i32, granularity : *mut i32) -> Error;
    pub fn PaAsio_GetInputChannelName(device : DeviceIndex, channelIndex : i32, channelName : *mut *const c_char) -> Error;
    pub fn PaAsio_GetOutputChannelName(device : DeviceIndex, channelIndex : i32, channelName : *mut *const c_char) -> Error;
    pub fn PaAsio_SetStreamSampleRate(stream : *mut C_PaStream, sampleRate : c_double) -> Error;


    // PortAudio Specific MAC_CORE
    pub fn PaMacCore_GetStreamInputDevice(s : *mut C_PaStream) -> DeviceIndex;
    pub fn PaMacCore_GetStreamOutputDevice(s : *mut C_PaStream) -> DeviceIndex;
    // pub fn PaMacCore_GetChannelName (int device, int channelIndex, bool intput) -> *c_char
    pub fn PaMacCore_GetBufferSizeRange(device : DeviceIndex, minBufferSizeFrames : *mut u32, maxBufferSizeFrames : *mut u32) -> Error;
    //pub fn PaMacCore_SetupStreamInfo(PaMacCoreStreamInfo *data, unsigned long flags) -> ();
    //pub fn PaMacCore_SetupChannelMap(PaMacCoreStreamInfo *data, const SInt32 *const channelMap, unsigned long channelMapSize) -> ();
}

/// A function to convert C `*const char` arrays into Rust `&'a str`s.
pub fn c_str_to_str<'a>(c_str: *const c_char) -> Result<&'a str, ::std::str::Utf8Error> {
    unsafe {
        ::std::ffi::CStr::from_ptr(c_str).to_str()
    }
}

/// A function to convert Rust strings to C strings
pub fn str_to_c_str(rust_str: &str) -> *const c_char {
    rust_str.as_ptr() as *const _
}
