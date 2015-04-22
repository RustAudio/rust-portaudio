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

use std::ptr;
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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
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
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum StreamFlags {
    /// No flags
    NoFlag =                                  ffi::PA_NO_FLAG,
    /// Disable default clipping of out of range samples.
    ClipOff =                                 ffi::PA_CLIP_OFF,
    /// Disable default dithering.
    DitherOff =                               ffi::PA_DITHER_OFF,
    /// Flag requests that where possible a full duplex stream will not discard overflowed input
    /// samples without calling the stream callback.
    NeverDropInput =                          ffi::PA_NEVER_DROP_INPUT,
    /// Call the stream callback to fill initial output buffers, rather than the default behavior
    /// of priming the buffers with zeros (silence)
    PrimeOutputBuffersUsingStreamCallback =   ffi::PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK,
    /// A mask specifying the platform specific bits.
    PlatformSpecificFlags =                   ffi::PA_PLATFORM_SPECIFIC_FLAGS
}

/// The flags returned after writing to a stream
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WriteFlags {
    /// The output stream has underflowed.
    OutputUnderflowed,
    /// The input stream has overflowed.
    InputOverflowed
}

/// Describes stream availability and the number for frames available for reading/writing if there
/// is any.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StreamAvailable {
    /// The number of frames available for reading.
    Frames(i64),
    /// The input stream has overflowed.
    InputOverflowed,
    /// The output stream has underflowed.
    OutputUnderflowed,
}

/// A rust enum representation of the C_PaStreamCallbackFlag
#[repr(u64)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum StreamCallbackFlags {
    /// In a stream opened with paFramesPerBufferUnspecified, indicates that input data is all
    /// silence (zeros) because no real data is available. In a stream opened without
    /// `FramesPerBufferUnspecified`, it indicates that one or more zero samples have been
    /// inserted into the input buffer to compensate for an input underflow.
    InputUnderflow  = ffi::INPUT_UNDERFLOW,
    /// In a stream opened with paFramesPerBufferUnspecified, indicates that data prior to the
    /// first sample of the input buffer was discarded due to an overflow, possibly because the
    /// stream callback is using too much CPU time. Otherwise indicates that data prior to one or
    /// more samples in the input buffer was discarded.
    InputOverflow   = ffi::INPUT_OVERFLOW,
    /// Indicates that output data (or a gap) was inserted, possibly because the stream callback
    /// is using too much CPU time.
    OutputUnderflow = ffi::OUTPUT_UNDERFLOW,
    /// Indicates that output data will be discarded because no room is available.
    OutputOverflow  = ffi::OUTPUT_OVERFLOW,
    /// Some of all of the output data will be used to prime the stream, input data may be zero.
    PrimingOutput   = ffi::PRIMING_OUTPUT,
}

impl StreamCallbackFlags {
    /// Convert an ffi::StreamCallbackFlags to Option<StreamCallbackFlags>.
    pub fn from_u64(n: u64) -> Option<StreamCallbackFlags> {
        match n {
            ffi::PA_NO_FLAG       => None,
            ffi::INPUT_UNDERFLOW  => Some(StreamCallbackFlags::InputUnderflow),
            ffi::INPUT_OVERFLOW   => Some(StreamCallbackFlags::InputOverflow),
            ffi::OUTPUT_UNDERFLOW => Some(StreamCallbackFlags::OutputUnderflow),
            ffi::OUTPUT_OVERFLOW  => Some(StreamCallbackFlags::OutputOverflow),
            ffi::PRIMING_OUTPUT   => Some(StreamCallbackFlags::PrimingOutput),
            _ => {
                println!("Unknown StreamCallbackFlags received: {:?}", n);
                None
            },
        }
    }
}

/// User defined callback function.
pub type StreamCallbackFn<I, O> =
    Box<FnMut(&[I], &mut[O], u32, &StreamCallbackTimeInfo, Option<StreamCallbackFlags>)
            -> StreamCallbackResult>;

#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(C)]
pub enum StreamCallbackResult {
    Continue = 0,
    Complete = 1,
    Abort = 2
}

/// Unchanging unique identifiers for each supported host API
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
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
    pub struct_version : i32,
    /// The type of the current host
    pub host_type : HostApiTypeId,
    /// The name of the host
    pub name : String,
    /// The total count of device in the host
    pub device_count : i32,
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
                struct_version : (*c_info).struct_version,
                host_type : transmute(((*c_info).host_type)),
                name : ffi::c_str_to_string(&(*c_info).name),
                device_count : (*c_info).device_count,
                default_input_device : (*c_info).default_input_device,
                default_output_device : (*c_info).default_output_device
            }
        }
    }

    pub fn unwrap(&self) -> ffi::C_PaHostApiInfo {
        ffi::C_PaHostApiInfo {
            struct_version : self.struct_version as i32,
            host_type : self.host_type as i32,
            name : ffi::string_to_c_str(&self.name),
            device_count : self.device_count as i32,
            default_input_device : self.default_input_device as i32,
            default_output_device : self.default_output_device as i32
        }
    }
}

/// Structure used to return information about a host error condition.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
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
            error_text : unsafe { ffi::c_str_to_string(&(*c_error).error_text) }
        }
    }

    pub fn unwrap(&self) -> ffi::C_PaHostErrorInfo {
        ffi::C_PaHostErrorInfo {
            error_code : self.error_code,
            error_text : ffi::string_to_c_str(&self.error_text)
        }
    }
}

/// A structure providing information and capabilities of PortAudio devices.
/// Devices may support input, output or both input and output.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct DeviceInfo {
    /// The version of the struct
    pub struct_version : i32,
    /// The name of the devie
    pub name : String,
    /// Host API identifier
    pub host_api : HostApiIndex,
    /// Maximal number of input channels for this device
    pub max_input_channels : i32,
    /// maximal number of output channel for this device
    pub max_output_channels : i32,
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
                struct_version : (*c_info).struct_version,
                name : ffi::c_str_to_string(&(*c_info).name),
                host_api : (*c_info).host_api,
                max_input_channels : (*c_info).max_input_channels,
                max_output_channels : (*c_info).max_output_channels,
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
            name : ffi::string_to_c_str(&self.name),
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
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
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
            sample_format : self.sample_format as ffi::SampleFormat,
            suggested_latency : self.suggested_latency,
            host_api_specific_stream_info : ptr::null_mut()
        }
    }
}


#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct StreamCallbackTimeInfo {
    pub input_buffer_adc_time : Time,
    pub current_time : Time,
    pub output_buffer_dac_time : Time
}

/// A structure containing unchanging information about an open stream.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
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
