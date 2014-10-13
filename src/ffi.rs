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

use libc::{c_char, c_double, c_void};

use types::{PaError, PaDeviceIndex, PaHostApiIndex, PaStreamCallbackFlags,
    PaStreamCallbackTimeInfo, PaStreamInfo, PaTime, PaStreamCallbackResult};

// Sample format
pub type PaSampleFormat = u64;
pub const PA_FLOAT_32: PaSampleFormat = 0x00000001;
pub const PA_INT_32: PaSampleFormat = 0x00000002;
// pub const PA_INT_24: PaSampleFormat = 0x00000004;
pub const PA_INT_16: PaSampleFormat = 0x00000008;
pub const PA_INT_8: PaSampleFormat = 0x00000010;
pub const PA_UINT_8: PaSampleFormat = 0x00000020;
pub const PA_CUSTOM_FORMAT: PaSampleFormat = 0x00010000;
pub const PA_NON_INTERLEAVED: PaSampleFormat = 0x80000000;

// Stream flags
pub type PaStreamFlags = u64;
pub const PA_NO_FLAG: PaStreamFlags = 0;
pub const PA_CLIP_OFF: PaStreamFlags = 0x00000001;
pub const PA_DITHER_OFF: PaStreamFlags = 0x00000002;
pub const PA_NEVER_DROP_INPUT: PaStreamFlags = 0x00000004;
pub const PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK: PaStreamFlags = 0x00000008;
pub const PA_PLATFORM_SPECIFIC_FLAGS: PaStreamFlags = 0xFFFF0000;

/// Unchanging unique identifiers for each supported host API
pub type PaHostApiTypeId = i32;
pub const PA_IN_DEVELOPMENT: PaHostApiTypeId = 0;
pub const PA_DIRECT_SOUND: PaHostApiTypeId = 1;
pub const PA_MME: PaHostApiTypeId = 2;
pub const PA_ASIO: PaHostApiTypeId = 3;
pub const PA_SOUND_MANAGER: PaHostApiTypeId = 4;
pub const PA_CORE_AUDIO: PaHostApiTypeId = 5;
pub const PA_OSS: PaHostApiTypeId = 7;
pub const PA_ALSA: PaHostApiTypeId = 8;
pub const PA_AL: PaHostApiTypeId = 9;
pub const PA_BE_OS: PaHostApiTypeId = 10;
pub const PA_WDMKS: PaHostApiTypeId = 11;
pub const PA_JACK: PaHostApiTypeId = 12;
pub const PA_WASAPI: PaHostApiTypeId = 13;
pub const PA_AUDIO_SCIENCE_HPI: PaHostApiTypeId = 14;

pub type C_PaStream = c_void;

#[repr(C)]
pub struct C_PaStreamParameters {
    pub device : PaDeviceIndex,
    pub channel_count : i32,
    pub sample_format : PaSampleFormat,
    pub suggested_latency : PaTime,
    pub host_api_specific_stream_info : *mut c_void
}

#[repr(C)]
pub struct C_PaDeviceInfo {
    pub struct_version: i32,
    pub name: *const c_char,
    pub host_api: PaHostApiIndex,
    pub max_input_channels: i32,
    pub max_output_channels: i32,
    pub default_low_input_latency: PaTime,
    pub default_low_output_latency: PaTime,
    pub default_high_input_latency: PaTime,
    pub default_high_output_latency: PaTime,
    pub default_sample_rate: c_double
}

#[repr(C)]
pub struct C_PaHostErrorInfo {
    pub error_code: u32,
    pub error_text: *const c_char
}

#[repr(C)]
pub struct C_PaHostApiInfo {
    pub struct_version: i32,
    pub host_type: i32,
    pub name: *const c_char,
    pub device_count: i32,
    pub default_input_device: i32,
    pub default_output_device: i32
}

extern "C" {

    /// PortAudio portable API

    pub fn Pa_GetVersion() -> i32;
    pub fn Pa_GetVersionText() -> *const c_char;
    pub fn Pa_GetErrorText(errorCode : PaError) -> *const c_char;
    pub fn Pa_Initialize() -> PaError;
    pub fn Pa_Terminate() -> PaError;
    pub fn Pa_GetHostApiCount() -> PaHostApiIndex;
    pub fn Pa_GetDefaultHostApi() -> PaHostApiIndex;
    pub fn Pa_GetHostApiInfo(hostApi : PaHostApiIndex) -> *const C_PaHostApiInfo;
    pub fn Pa_HostApiTypeIdToHostApiIndex(type_id : PaHostApiTypeId) -> PaHostApiIndex;
    pub fn Pa_HostApiDeviceIndexToDeviceIndex(hostApi : PaHostApiIndex, hostApiDeviceIndex : i32) -> PaDeviceIndex;
    pub fn Pa_GetLastHostErrorInfo() -> *const C_PaHostErrorInfo;
    pub fn Pa_GetDeviceCount() -> PaDeviceIndex;
    pub fn Pa_GetDefaultInputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDefaultOutputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDeviceInfo(device : PaDeviceIndex) -> *const C_PaDeviceInfo;
    pub fn Pa_IsFormatSupported(input_parameters : *const C_PaStreamParameters, outputParameters : *const C_PaStreamParameters, sampleRate : c_double) -> PaError;
    pub fn Pa_GetSampleSize(format : PaSampleFormat) -> PaError;
    pub fn Pa_Sleep(msec : i32) -> ();
    pub fn Pa_OpenStream(stream : *mut *mut C_PaStream,
                         inputParameters : *const C_PaStreamParameters,
                         outputParameters : *const C_PaStreamParameters,
                         sampleRate : c_double,
                         framesPerBuffer : u32,
                         streamFlags : PaStreamFlags,
                         streamCallback : Option<extern "C" fn(*const c_void, *mut c_void, u32, *const PaStreamCallbackTimeInfo, PaStreamCallbackFlags, *mut c_void) -> PaStreamCallbackResult>,
                         userData : *mut c_void)
                         -> PaError;
    pub fn Pa_OpenDefaultStream(stream : *mut *mut C_PaStream,
                                numInputChannels : i32,
                                numOutputChannels : i32,
                                sampleFormat : PaSampleFormat,
                                sampleRate : c_double,
                                framesPerBuffer : u32,
                                streamCallback : Option<extern "C" fn(*const c_void, *mut c_void, u32, *const PaStreamCallbackTimeInfo, PaStreamCallbackFlags, *mut c_void) -> PaStreamCallbackResult>,
                                userData : *mut c_void)
                                -> PaError;
    pub fn Pa_CloseStream(stream : *mut C_PaStream) -> PaError;
    //pub fn Pa_SetStreamFinishedCallback (stream : *PaStream, PaStreamFinishedCallback *streamFinishedCallback) -> PaError;
    pub fn Pa_StartStream(stream : *mut C_PaStream) -> PaError;
    pub fn Pa_StopStream(stream : *mut C_PaStream) -> PaError;
    pub fn Pa_AbortStream(stream : *mut C_PaStream) -> PaError;
    pub fn Pa_IsStreamStopped(stream : *mut C_PaStream) -> PaError;
    pub fn Pa_IsStreamActive(stream : *mut C_PaStream) -> i32;
    pub fn Pa_GetStreamInfo(stream : *mut C_PaStream) -> *const PaStreamInfo;
    pub fn Pa_GetStreamTime(stream : *mut C_PaStream) -> PaTime;
    pub fn Pa_GetStreamCpuLoad(stream : *mut C_PaStream) -> c_double;
    pub fn Pa_ReadStream(stream : *mut C_PaStream, buffer : *mut c_void, frames : u32) -> PaError;
    pub fn Pa_WriteStream(stream : *mut C_PaStream, buffer : *mut c_void, frames : u32) -> PaError;
    pub fn Pa_GetStreamReadAvailable(stream : *mut C_PaStream) -> i64;
    pub fn Pa_GetStreamWriteAvailable(stream : *mut C_PaStream) -> i64;

    /*
    * PortAudio Specific ASIO
    */
    pub fn PaAsio_GetAvailableBufferSizes(device : PaDeviceIndex, minBufferSizeFrames : *mut i32, maxBufferSizeFrames : *mut i32, preferredBufferSizeFrames : *mut i32, granularity : *mut i32) -> PaError;
    pub fn PaAsio_GetInputChannelName(device : PaDeviceIndex, channelIndex : i32, channelName : *mut *const c_char) -> PaError;
    pub fn PaAsio_GetOutputChannelName(device : PaDeviceIndex, channelIndex : i32, channelName : *mut *const c_char) -> PaError;
    pub fn PaAsio_SetStreamSampleRate(stream : *mut C_PaStream, sampleRate : c_double) -> PaError;


    /*
    * PortAudio Specific MAC_CORE
    */
    pub fn PaMacCore_GetStreamInputDevice(s : *mut C_PaStream) -> PaDeviceIndex;
    pub fn PaMacCore_GetStreamOutputDevice(s : *mut C_PaStream) -> PaDeviceIndex;
    // pub fn PaMacCore_GetChannelName (int device, int channelIndex, bool intput) -> *c_char
    pub fn PaMacCore_GetBufferSizeRange(device : PaDeviceIndex, minBufferSizeFrames : *mut u32, maxBufferSizeFrames : *mut u32) -> PaError;
    //pub fn PaMacCore_SetupStreamInfo(PaMacCoreStreamInfo *data, unsigned long flags) -> ();
    //pub fn PaMacCore_SetupChannelMap(PaMacCoreStreamInfo *data, const SInt32 *const channelMap, unsigned long channelMapSize) -> ();
}
