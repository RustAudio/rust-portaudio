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
pub static PaFloat32: PaSampleFormat = 0x00000001;
pub static PaInt32: PaSampleFormat = 0x00000002;
// pub static PaInt24: PaSampleFormat = 0x00000004;
pub static PaInt16: PaSampleFormat = 0x00000008;
pub static PaInt8: PaSampleFormat = 0x00000010;
pub static PaUInt8: PaSampleFormat = 0x00000020;
pub static PaCustomFormat: PaSampleFormat = 0x00010000;
pub static PaNonInterleaved: PaSampleFormat = 0x80000000;

// Stream flags
pub type PaStreamFlags = u64;
pub static PaNoFlag: PaStreamFlags = 0;
pub static PaClipOff: PaStreamFlags = 0x00000001;
pub static PaDitherOff: PaStreamFlags = 0x00000002;
pub static PaNeverDropInput: PaStreamFlags = 0x00000004;
pub static PaPrimeOutputBuffersUsingStreamCallback: PaStreamFlags = 0x00000008;
pub static PaPlatformSpecificFlags: PaStreamFlags = 0xFFFF0000;

/// Unchanging unique identifiers for each supported host API
pub type PaHostApiTypeId = i32;
pub static PaInDevelopment: PaHostApiTypeId = 0;
pub static PaDirectSound: PaHostApiTypeId = 1;
pub static PaMME: PaHostApiTypeId = 2;
pub static PaASIO: PaHostApiTypeId = 3;
pub static PaSoundManager: PaHostApiTypeId = 4;
pub static PaCoreAudio: PaHostApiTypeId = 5;
pub static PaOSS: PaHostApiTypeId = 7;
pub static PaALSA: PaHostApiTypeId = 8;
pub static PaAL: PaHostApiTypeId = 9;
pub static PaBeOS: PaHostApiTypeId = 10;
pub static PaWDMKS: PaHostApiTypeId = 11;
pub static PaJACK: PaHostApiTypeId = 12;
pub static PaWASAPI: PaHostApiTypeId = 13;
pub static PaAudioScienceHPI: PaHostApiTypeId = 14;

pub type C_PaStream = c_void;

pub struct C_PaStreamParameters {
    pub device : PaDeviceIndex,
    pub channel_count : i32,
    pub sample_format : PaSampleFormat,
    pub suggested_latency : PaTime,
    pub host_api_specific_stream_info : *c_void
}

pub struct C_PaDeviceInfo {
    pub struct_version: i32,
    pub name: *c_char,
    pub host_api: PaHostApiIndex,
    pub max_input_channels: i32,
    pub max_output_channels: i32,
    pub default_low_input_latency: PaTime,
    pub default_low_output_latency: PaTime,
    pub default_high_input_latency: PaTime,
    pub default_high_output_latency: PaTime,
    pub default_sample_rate: c_double
}

pub struct C_PaHostErrorInfo {
    pub error_code: u32,
    pub error_text: *c_char
}

pub struct C_PaHostApiInfo {
    pub struct_version: i32,
    pub host_type: i32,
    pub name: *c_char,
    pub device_count: i32,
    pub default_input_device: i32,
    pub default_output_device: i32
}

extern "C" {

    /// PortAudio portable API

    pub fn Pa_GetVersion() -> i32;
    pub fn Pa_GetVersionText() -> *c_char;
    pub fn Pa_GetErrorText(errorCode : PaError) -> *c_char;
    pub fn Pa_Initialize() -> PaError;
    pub fn Pa_Terminate() -> PaError;
    pub fn Pa_GetHostApiCount() -> PaHostApiIndex;
    pub fn Pa_GetDefaultHostApi() -> PaHostApiIndex;
    pub fn Pa_GetHostApiInfo(hostApi : PaHostApiIndex) -> *C_PaHostApiInfo;
    pub fn Pa_HostApiTypeIdToHostApiIndex(type_id : PaHostApiTypeId) -> PaHostApiIndex;
    pub fn Pa_HostApiDeviceIndexToDeviceIndex(hostApi : PaHostApiIndex, hostApiDeviceIndex : i32) -> PaDeviceIndex;
    pub fn Pa_GetLastHostErrorInfo() -> *C_PaHostErrorInfo;
    pub fn Pa_GetDeviceCount() -> PaDeviceIndex;
    pub fn Pa_GetDefaultInputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDefaultOutputDevice() -> PaDeviceIndex;
    pub fn Pa_GetDeviceInfo(device : PaDeviceIndex) -> *C_PaDeviceInfo;
    pub fn Pa_IsFormatSupported(input_parameters : *C_PaStreamParameters, outputParameters : *C_PaStreamParameters, sampleRate : c_double) -> PaError;
    pub fn Pa_GetSampleSize(format : PaSampleFormat) -> PaError;
    pub fn Pa_Sleep(msec : i32) -> ();
    pub fn Pa_OpenStream(stream : **C_PaStream,
                         inputParameters : *C_PaStreamParameters,
                         outputParameters : *C_PaStreamParameters,
                         sampleRate : c_double,
                         framesPerBuffer : u32,
                         streamFlags : PaStreamFlags,
                         streamCallback : Option<extern "C" fn(*c_void, *c_void, u32, *PaStreamCallbackTimeInfo, PaStreamCallbackFlags, *c_void) -> PaStreamCallbackResult>,
                         userData : *c_void)
                         -> PaError;
    pub fn Pa_OpenDefaultStream(stream : **C_PaStream,
                                numInputChannels : i32,
                                numOutputChannels : i32,
                                sampleFormat : PaSampleFormat,
                                sampleRate : c_double,
                                framesPerBuffer : u32,
                                streamCallback : Option<extern "C" fn(*c_void, *c_void, u32, *PaStreamCallbackTimeInfo, PaStreamCallbackFlags, *c_void) -> PaStreamCallbackResult>,
                                userData : *c_void)
                                -> PaError;
    pub fn Pa_CloseStream(stream : *C_PaStream) -> PaError;
    //pub fn Pa_SetStreamFinishedCallback (stream : *PaStream, PaStreamFinishedCallback *streamFinishedCallback) -> PaError;
    pub fn Pa_StartStream(stream : *C_PaStream) -> PaError;
    pub fn Pa_StopStream(stream : *C_PaStream) -> PaError;
    pub fn Pa_AbortStream(stream : *C_PaStream) -> PaError;
    pub fn Pa_IsStreamStopped(stream : *C_PaStream) -> PaError;
    pub fn Pa_IsStreamActive(stream : *C_PaStream) -> i32;
    pub fn Pa_GetStreamInfo(stream : *C_PaStream) -> *PaStreamInfo;
    pub fn Pa_GetStreamTime(stream : *C_PaStream) -> PaTime;
    pub fn Pa_GetStreamCpuLoad(stream : *C_PaStream) -> c_double;
    pub fn Pa_ReadStream(stream : *C_PaStream, buffer : *c_void, frames : u32) -> PaError;
    pub fn Pa_WriteStream(stream : *C_PaStream, buffer : *c_void, frames : u32) -> PaError;
    pub fn Pa_GetStreamReadAvailable(stream : *C_PaStream) -> i64;
    pub fn Pa_GetStreamWriteAvailable(stream : *C_PaStream) -> i64;

    /*
    * PortAudio Specific ASIO
    */
    pub fn PaAsio_GetAvailableBufferSizes(device : PaDeviceIndex, minBufferSizeFrames : *i32, maxBufferSizeFrames : *i32, preferredBufferSizeFrames : *i32, granularity : *i32) -> PaError;
    pub fn PaAsio_GetInputChannelName(device : PaDeviceIndex, channelIndex : i32, channelName : **c_char) -> PaError;
    pub fn PaAsio_GetOutputChannelName(device : PaDeviceIndex, channelIndex : i32, channelName : **c_char) -> PaError;
    pub fn PaAsio_SetStreamSampleRate(stream : *C_PaStream, sampleRate : c_double) -> PaError;


    /*
    * PortAudio Specific MAC_CORE
    */
    pub fn PaMacCore_GetStreamInputDevice(s : *C_PaStream) -> PaDeviceIndex;
    pub fn PaMacCore_GetStreamOutputDevice(s : *C_PaStream) -> PaDeviceIndex;
    // pub fn PaMacCore_GetChannelName (int device, int channelIndex, bool intput) -> *c_char
    pub fn PaMacCore_GetBufferSizeRange(device : PaDeviceIndex, minBufferSizeFrames : *u32, maxBufferSizeFrames : *u32) -> PaError;
    //pub fn PaMacCore_SetupStreamInfo(PaMacCoreStreamInfo *data, unsigned long flags) -> ();
    //pub fn PaMacCore_SetupChannelMap(PaMacCoreStreamInfo *data, const SInt32 *const channelMap, unsigned long channelMapSize) -> ();
}
