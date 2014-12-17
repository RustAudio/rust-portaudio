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
pub type DeviceIndex = i32;

/// A special DeviceIndex value indicating that no device is available,
/// or should be used.
pub const PA_NO_DEVICE: DeviceIndex = -1;

/// A special DeviceIndex value indicating that the device(s) to be used are
/// specified in the host api specific stream info structure.
pub const PA_USE_HOST_API_SPECIFIC_DEVICE_SPECIFICATION: DeviceIndex = -2;

/// The type used to enumerate to host APIs at runtime.
/// Values of this type range from 0 to (pa::get_host_api_count()-1).
pub type HostApiIndex = i32;

/// The type used to represent monotonic time in seconds.
pub type Time = f64;

/// An type alias used to represent a given number of frames.
pub type Frames = i64;

/// A type used to specify one or more sample formats.
#[repr(u64)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum SampleFormat {
    /// 32 bits float sample format
    Float32 =         ffi::PA_FLOAT_32,
    /// 32 bits int sample format
    Int32 =           ffi::PA_INT_32,
    /// 16 bits int sample format
    Int16 =           ffi::PA_INT_16,
    /// 8 bits int sample format
    Int8 =            ffi::PA_INT_8,
    /// 8 bits unsigned int sample format
    UInt8 =           ffi::PA_UINT_8,
    /// Custom sample format
    CustomFormat =    ffi::PA_CUSTOM_FORMAT,
    /// Non interleaved sample format
    NonInterleaved =  ffi::PA_NON_INTERLEAVED
}

/// The flags to pass to a stream
#[repr(u64)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum StreamFlags {
    /// No flags
    NoFlag =                                  ffi::PA_NO_FLAG,
    /// Disable default clipping of out of range samples.
    ClipOff =                                 ffi::PA_CLIP_OFF,
    /// Disable default dithering.
    DitherOff =                               ffi::PA_DITHER_OFF,
    /// Flag requests that where possible a full duplex stream will not discard overflowed input samples without calling the stream callback.
    NeverDropInput =                          ffi::PA_NEVER_DROP_INPUT,
    /// Call the stream callback to fill initial output buffers, rather than the default behavior of priming the buffers with zeros (silence)
    PrimeOutputBuffersUsingStreamCallback =   ffi::PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK,
    /// A mask specifying the platform specific bits.
    PlatformSpecificFlags =                   ffi::PA_PLATFORM_SPECIFIC_FLAGS
}


#[doc(hidden)]
pub type StreamCallbackFlags = u64;
/*
    pub static InputUnderflow : StreamCallbackFlags = 0x00000001;
    pub static InputOverflow : StreamCallbackFlags = 0x00000002;
    pub static OutputUnderflow : StreamCallbackFlags = 0x00000004;
    pub static OutputOverflow : StreamCallbackFlags = 0x00000008;
    pub static PrimingOutput : StreamCallbackFlags = 0x00000010;
*/

#[doc(hidden)]
pub type CallbackFunction = extern fn(i : f32) -> StreamCallbackResult;
#[doc(hidden)]
#[repr(C)]
pub enum StreamCallbackResult {
    Continue = 0,
    Complete = 1,
    Abort = 2
}

/// Unchanging unique identifiers for each supported host API
#[repr(i32)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum HostApiTypeId {
    /// In development host
    InDevelopment =   ffi::PA_IN_DEVELOPMENT,
    /// Direct sound
    DirectSound =     ffi::PA_DIRECT_SOUND,
    /// MMe API
    MME =             ffi::PA_MME,
    /// ASIO API
    ASIO =            ffi::PA_ASIO,
    /// Sound manager API
    SoundManager =    ffi::PA_SOUND_MANAGER,
    /// Core Audio API
    CoreAudio =       ffi::PA_CORE_AUDIO,
    /// OSS API
    OSS =             ffi::PA_OSS,
    /// Alsa API
    ALSA =            ffi::PA_ALSA,
    /// AL API
    AL =              ffi::PA_AL,
    /// BeOS API
    BeOS =            ffi::PA_BE_OS,
    /// WDMKS
    WDMKS =           ffi::PA_WDMKS,
    /// Jack API
    JACK =            ffi::PA_JACK,
    /// WASAPI
    WASAPI =          ffi::PA_WASAPI,
    /// Audio Science HPI
    AudioScienceHPI = ffi::PA_AUDIO_SCIENCE_HPI
}

/// A structure containing information about a particular host API.
pub struct HostApiInfo{
    /// The version of the struct
    pub struct_version : int,
    /// The type of the current host
    pub host_type : HostApiTypeId,
    /// The name of the host
    pub name : String,
    /// The total count of device in the host
    pub device_count : int,
    /// The index to the default input device
    pub default_input_device : DeviceIndex,
    /// The index to the default output device
    pub default_output_device : DeviceIndex
}

#[doc(hidden)]
impl HostApiInfo {
    pub fn wrap(c_info : *const ffi::C_PaHostApiInfo) -> HostApiInfo {
        unsafe {
            HostApiInfo {
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
            host_type : self.host_type.clone() as i32,
            name : unsafe { self.name.to_c_str().unwrap() },
            device_count : self.device_count as i32,
            default_input_device : self.default_input_device as i32,
            default_output_device : self.default_output_device as i32
        }
    }
}

/// Structure used to return information about a host error condition.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct HostErrorInfo {
    /// The code of the error
    pub error_code : u32,
    /// The string which explain the error
    pub error_text : String
}

#[doc(hidden)]
impl HostErrorInfo {
    pub fn wrap(c_error : *const ffi::C_PaHostErrorInfo) -> HostErrorInfo {
        HostErrorInfo {
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
pub struct DeviceInfo {
    /// The version of the struct
    pub struct_version : int,
    /// The name of the devie
    pub name : String,
    /// Host API identifier
    pub host_api : HostApiIndex,
    /// Maximal number of input channels for this device
    pub max_input_channels : int,
    /// maximal number of output channel for this device
    pub max_output_channels : int,
    /// The default low latency for input with this device
    pub default_low_input_latency : Time,
    /// The default low latency for output with this device
    pub default_low_output_latency : Time,
    /// The default high latency for input with this device
    pub default_high_input_latency : Time,
    /// The default high latency for output with this device
    pub default_high_output_latency : Time,
    /// The default sample rate for this device
    pub default_sample_rate : f64
}

#[doc(hidden)]
impl DeviceInfo {
    pub fn wrap(c_info : *const ffi::C_PaDeviceInfo) -> DeviceInfo {
        unsafe {
            DeviceInfo {
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
pub struct StreamParameters {
    /// Index of the device
    pub device : DeviceIndex,
    /// The number of channels for this device
    pub channel_count : i32,
    /// Sample format of the device
    pub sample_format : SampleFormat,
    /// The suggested latency for this device
    pub suggested_latency : Time,
}

#[doc(hidden)]
impl StreamParameters {
    pub fn wrap(c_parameters : *mut ffi::C_PaStreamParameters) -> StreamParameters {
        unsafe {
            StreamParameters {
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
            sample_format : self.sample_format.clone() as ffi::SampleFormat,
            suggested_latency : self.suggested_latency,
            host_api_specific_stream_info : ptr::null_mut()
        }
    }
}


#[doc(hidden)]
#[repr(C)]
pub struct StreamCallbackTimeInfo {
    pub input_buffer_adc_time : Time,
    pub current_time : Time,
    pub output_buffer_dac_time : Time
}

/// A structure containing unchanging information about an open stream.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
#[repr(C)]
pub struct StreamInfo {
    /// Struct version
    pub struct_version : i32,
    /// The input latency for this open stream
    pub input_latency : Time,
    /// The output latency for this open stream
    pub output_latency : Time,
    /// The sample rate for this open stream
    pub sample_rate : f64
}

