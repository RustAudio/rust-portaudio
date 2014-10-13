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

//! Types used in the PortAudio API

#![allow(dead_code)]

use std::{string, ptr};
use std::mem::{transmute};

use ffi;

/// The type used to refer to audio devices. Values of this type usually range
/// from 0 to (pa::get_device_count()-1)
pub type PaDeviceIndex = i32;
/// A special PaDeviceIndex value indicating that no device is available,
/// or should be used.
pub const PA_NO_DEVICE: PaDeviceIndex = -1;
/// A special PaDeviceIndex value indicating that the device(s) to be used are
/// specified in the host api specific stream info structure.
pub const PA_USE_HOST_API_SPECIFIC_DEVICE_SPECIFICATION: PaDeviceIndex = -2;

/// The type used to enumerate to host APIs at runtime.
/// Values of this type range from 0 to (pa::get_host_api_count()-1).
pub type PaHostApiIndex = i32;

/// The type used to represent monotonic time in seconds.
pub type PaTime = f64;

/// A type used to specify one or more sample formats.
#[repr(u64)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PaSampleFormat {
    /// 32 bits float sample format
    PaFloat32 =         ffi::PA_FLOAT_32,
    /// 32 bits int sample format
    PaInt32 =           ffi::PA_INT_32,
    /// 16 bits int sample format
    PaInt16 =           ffi::PA_INT_16,
    /// 8 bits int sample format
    PaInt8 =            ffi::PA_INT_8,
    /// 8 bits unsigned int sample format
    PaUInt8 =           ffi::PA_UINT_8,
    /// Custom sample format
    PaCustomFormat =    ffi::PA_CUSTOM_FORMAT,
    /// Non interleaved sample format
    PaNonInterleaved =  ffi::PA_NON_INTERLEAVED
}

/// The flags to pass to a stream
#[repr(u64)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PaStreamFlags {
    /// No flags
    PaNoFlag =                                  ffi::PA_NO_FLAG,
    /// Disable default clipping of out of range samples.
    PaClipOff =                                 ffi::PA_CLIP_OFF,
    /// Disable default dithering.
    PaDitherOff =                               ffi::PA_DITHER_OFF,
    /// Flag requests that where possible a full duplex stream will not discard overflowed input samples without calling the stream callback.
    PaNeverDropInput =                          ffi::PA_NEVER_DROP_INPUT,
    /// Call the stream callback to fill initial output buffers, rather than the default behavior of priming the buffers with zeros (silence)
    PaPrimeOutputBuffersUsingStreamCallback =   ffi::PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK,
    /// A mask specifying the platform specific bits.
    PaPlatformSpecificFlags =                   ffi::PA_PLATFORM_SPECIFIC_FLAGS
}


#[doc(hidden)]
pub type PaStreamCallbackFlags = u64;
/*
    pub static PaInputUnderflow : PaStreamCallbackFlags = 0x00000001;
    pub static PaInputOverflow : PaStreamCallbackFlags = 0x00000002;
    pub static PaOutputUnderflow : PaStreamCallbackFlags = 0x00000004;
    pub static PaOutputOverflow : PaStreamCallbackFlags = 0x00000008;
    pub static PaPrimingOutput : PaStreamCallbackFlags = 0x00000010;
*/

#[doc(hidden)]
pub type PaCallbackFunction = extern fn(i : f32) -> PaStreamCallbackResult;
#[doc(hidden)]
#[repr(C)]
pub enum PaStreamCallbackResult {
    PaContinue = 0,
    PaComplete = 1,
    PaAbort = 2
}

/// Error codes returned by PortAudio functions.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PaError {
    /// No Error
    PaNoError = 0,
    /// Portaudio not initialized
    PaNotInitialized = -10000,
    /// Unanticipated error from the host
    PaUnanticipatedHostError,
    /// Invalid channel count
    PaInvalidChannelCount,
    /// Invalid sample rate
    PaInvalidSampleRate,
    /// Invalid Device
    PaInvalidDevice,
    /// Invalid Flag
    PaInvalidFlag,
    /// The Sample format is not supported
    PaSampleFormatNotSupported,
    /// Input device not compatible with output device
    PaBadIODeviceCombination,
    /// Memory insufficient
    PaInsufficientMemory,
    /// The buffer is too big
    PaBufferTooBig,
    /// The buffer is too small
    PaBufferTooSmall,
    /// Invalid callback
    PaNullCallback,
    /// Invalid Stream
    PaBadStreamPtr,
    /// Time out
    PaTimedOut,
    /// Portaudio internal error
    PaInternalError,
    /// Device unavailable
    PaDeviceUnavailable,
    /// Stream info not compatible with the host
    PaIncompatibleHostApiSpecificStreamInfo,
    /// The stream is stopped
    PaStreamIsStopped,
    /// The stream is not stopped
    PaStreamIsNotStopped,
    /// The input stream has overflowed
    PaInputOverflowed,
    /// The output has overflowed
    PaOutputUnderflowed,
    /// The host API is not found by Portaudio
    PaHostApiNotFound,
    /// The host API is invalid
    PaInvalidHostApi,
    /// Portaudio cannot read from the callback stream
    PaCanNotReadFromACallbackStream,
    /// Portaudio cannot wrtie to the callback stream
    PaCanNotWriteToACallbackStream,
    /// Portaudio cannot read from an output only stream
    PaCanNotReadFromAnOutputOnlyStream,
    /// Portaudio cannot write to an input only stream
    PaCanNotWriteToAnInputOnlyStream,
    /// The stream is not compatible with the host API
    PaIncompatibleStreamHostApi,
    /// Invalid buffer
    PaBadBufferPtr
}

/// Unchanging unique identifiers for each supported host API
#[repr(i32)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PaHostApiTypeId {
    /// In development host
    PaInDevelopment =   ffi::PA_IN_DEVELOPMENT,
    /// Direct sound
    PaDirectSound =     ffi::PA_DIRECT_SOUND,
    /// MMe API
    PaMME =             ffi::PA_MME,
    /// ASIO API
    PaASIO =            ffi::PA_ASIO,
    /// Sound manager API
    PaSoundManager =    ffi::PA_SOUND_MANAGER,
    /// Core Audio API
    PaCoreAudio =       ffi::PA_CORE_AUDIO,
    /// OSS API
    PaOSS =             ffi::PA_OSS,
    /// Alsa API
    PaALSA =            ffi::PA_ALSA,
    /// AL API
    PaAL =              ffi::PA_AL,
    /// BeOS API
    PaBeOS =            ffi::PA_BE_OS,
    /// WDMKS
    PaWDMKS =           ffi::PA_WDMKS,
    /// Jack API
    PaJACK =            ffi::PA_JACK,
    /// WASAPI
    PaWASAPI =          ffi::PA_WASAPI,
    /// Audio Science HPI
    PaAudioScienceHPI = ffi::PA_AUDIO_SCIENCE_HPI
}

/// A structure containing information about a particular host API.
pub struct PaHostApiInfo{
    /// The version of the struct
    pub struct_version : int,
    /// The type of the current host
    pub host_type : PaHostApiTypeId,
    /// The name of the host
    pub name : String,
    /// The total count of device in the host
    pub device_count : int,
    /// The index to the default input device
    pub default_input_device : PaDeviceIndex,
    /// The index to the default output device
    pub default_output_device : PaDeviceIndex
}

#[doc(hidden)]
impl PaHostApiInfo {
    pub fn wrap(c_info : *const ffi::C_PaHostApiInfo) -> PaHostApiInfo {
        unsafe {
            PaHostApiInfo {
                struct_version : (*c_info).struct_version as int,
                host_type : transmute(((*c_info).host_type)),
                name : string::raw::from_buf((*c_info).name as *const u8),
                device_count : (*c_info).device_count as int,
                default_input_device : (*c_info).default_input_device,
                default_output_device : (*c_info).default_output_device
            }
        }
    }

    pub fn unwrap(&self) -> ffi::C_PaHostApiInfo {
        ffi::C_PaHostApiInfo {
            struct_version : self.struct_version as i32,
            host_type : self.host_type as i32,
            name : unsafe { self.name.to_c_str().unwrap() },
            device_count : self.device_count as i32,
            default_input_device : self.default_input_device as i32,
            default_output_device : self.default_output_device as i32
        }
    }
}

/// Structure used to return information about a host error condition.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct PaHostErrorInfo {
    /// The code of the error
    pub error_code : u32,
    /// The string which explain the error
    pub error_text : String
}

#[doc(hidden)]
impl PaHostErrorInfo {
    pub fn wrap(c_error : *const ffi::C_PaHostErrorInfo) -> PaHostErrorInfo {
        PaHostErrorInfo {
            error_code : unsafe { (*c_error).error_code },
            error_text : unsafe { string::raw::from_buf((*c_error).error_text as *const u8) }
        }
    }

    pub fn unwrap(&self) -> ffi::C_PaHostErrorInfo {
        ffi::C_PaHostErrorInfo {
            error_code : self.error_code,
            error_text : unsafe { self.error_text.to_c_str().unwrap() }
        }
    }
}

/// A structure providing information and capabilities of PortAudio devices.
/// Devices may support input, output or both input and output.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct PaDeviceInfo {
    /// The version of the struct
    pub struct_version : int,
    /// The name of the devie
    pub name : String,
    /// Host API identifier
    pub host_api : PaHostApiIndex,
    /// Maximal number of input channels for this device
    pub max_input_channels : int,
    /// maximal number of output channel for this device
    pub max_output_channels : int,
    /// The default low latency for input with this device
    pub default_low_input_latency : PaTime,
    /// The default low latency for output with this device
    pub default_low_output_latency : PaTime,
    /// The default high latency for input with this device
    pub default_high_input_latency : PaTime,
    /// The default high latency for output with this device
    pub default_high_output_latency : PaTime,
    /// The default sample rate for this device
    pub default_sample_rate : f64
}

#[doc(hidden)]
impl PaDeviceInfo {
    pub fn wrap(c_info : *const ffi::C_PaDeviceInfo) -> PaDeviceInfo {
        unsafe {
            PaDeviceInfo {
                struct_version : (*c_info).struct_version as int,
                name : string::raw::from_buf((*c_info).name as *const u8),
                host_api : (*c_info).host_api,
                max_input_channels : (*c_info).max_input_channels as int,
                max_output_channels : (*c_info).max_output_channels as int,
                default_low_input_latency : (*c_info).default_low_input_latency,
                default_low_output_latency : (*c_info).default_low_output_latency,
                default_high_input_latency : (*c_info).default_high_input_latency,
                default_high_output_latency : (*c_info).default_high_output_latency,
                default_sample_rate : (*c_info).default_sample_rate
            }
        }
    }

    pub fn unwrap(&self) -> ffi::C_PaDeviceInfo {
        ffi::C_PaDeviceInfo {
            struct_version : self.struct_version as i32,
            name : unsafe { self.name.to_c_str().unwrap() },
            host_api : self.host_api,
            max_input_channels : self.max_input_channels as i32,
            max_output_channels : self.max_output_channels as i32,
            default_low_input_latency : self.default_low_input_latency,
            default_low_output_latency : self.default_low_output_latency,
            default_high_input_latency : self.default_high_input_latency,
            default_high_output_latency : self.default_high_output_latency,
            default_sample_rate : self.default_sample_rate
        }
    }
}

/// Parameters for one direction (input or output) of a stream.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct PaStreamParameters {
    /// Index of the device
    pub device : PaDeviceIndex,
    /// The number of channels for this device
    pub channel_count : i32,
    /// Sample format of the device
    pub sample_format : PaSampleFormat,
    /// The suggested latency for this device
    pub suggested_latency : PaTime,
}

#[doc(hidden)]
impl PaStreamParameters {
    pub fn wrap(c_parameters : *mut ffi::C_PaStreamParameters) -> PaStreamParameters {
        unsafe {
            PaStreamParameters {
                device : (*c_parameters).device,
                channel_count : (*c_parameters).channel_count,
                sample_format : transmute((*c_parameters).sample_format),
                suggested_latency : (*c_parameters).suggested_latency
            }
        }
    }

    pub fn unwrap(&self) -> ffi::C_PaStreamParameters {
        ffi::C_PaStreamParameters {
            device : self.device,
            channel_count : self.channel_count as i32,
            sample_format : self.sample_format as ffi::PaSampleFormat,
            suggested_latency : self.suggested_latency,
            host_api_specific_stream_info : ptr::null_mut()
        }
    }
}


#[doc(hidden)]
#[repr(C)]
pub struct PaStreamCallbackTimeInfo {
    pub input_buffer_adc_time : PaTime,
    pub current_time : PaTime,
    pub output_buffer_dac_time : PaTime
}

/// A structure containing unchanging information about an open stream.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
#[repr(C)]
pub struct PaStreamInfo {
    /// Struct version
    pub struct_version : i32,
    /// The input latency for this open stream
    pub input_latency : PaTime,
    /// The output latency for this open stream
    pub output_latency : PaTime,
    /// The sample rate for this open stream
    pub sample_rate : f64
}

