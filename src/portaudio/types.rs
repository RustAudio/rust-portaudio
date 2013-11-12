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
* Types used in the PortAudio API
*/

use std::libc::{c_void, c_char, c_double};
use std::{str, ptr, cast};

#[doc(hidden)]
pub type C_PaStream = c_void;

/// The type used to refer to audio devices. Values of this type usually range from 0 to (pa::get_device_count()-1)
pub type PaDeviceIndex = i32;
/// A special PaDeviceIndex value indicating that no device is available, or should be used.
pub static PaNoDevice : PaDeviceIndex = -1;
/// A special PaDeviceIndex value indicating that the device(s) to be used are specified in the host api specific stream info structure.
pub static PaUseHostApiSpecificDeviceSpecification : PaDeviceIndex = -2;

/// The type used to enumerate to host APIs at runtime. 
/// Values of this type range from 0 to (pa::get_host_api_count()-1).
pub type PaHostApiIndex = i32;

/// The type used to represent monotonic time in seconds.
pub type PaTime = f64;


#[doc(hidden)]
mod ffi {
    
    // Sample format
    pub type PaSampleFormat = u64;
    pub static PaFloat32 :          PaSampleFormat = 0x00000001;
    pub static PaInt32 :            PaSampleFormat = 0x00000002;
    // pub static PaInt24 :          PaSampleFormat = 0x00000004;
    pub static PaInt16 :            PaSampleFormat = 0x00000008;
    pub static PaInt8 :             PaSampleFormat = 0x00000010;
    pub static PaUInt8 :            PaSampleFormat = 0x00000020; 
    pub static PaCustomFormat :     PaSampleFormat = 0x00010000;
    pub static PaNonInterleaved :   PaSampleFormat = 0x80000000;

    // Stream flags
    pub type PaStreamFlags = u64;
    pub static PaNoFlag :                                   PaStreamFlags = 0;
    pub static PaClipOff :                                  PaStreamFlags = 0x00000001;
    pub static PaDitherOff :                                PaStreamFlags = 0x00000002;
    pub static PaNeverDropInput :                           PaStreamFlags = 0x00000004;
    pub static PaPrimeOutputBuffersUsingStreamCallback :    PaStreamFlags = 0x00000008;
    pub static PaPlatformSpecificFlags :                    PaStreamFlags = 0xFFFF0000;

    /// Unchanging unique identifiers for each supported host API
    pub type PaHostApiTypeId = i32;
    pub static PaInDevelopment : PaHostApiTypeId = 0;
    pub static PaDirectSound : PaHostApiTypeId = 1;
    pub static PaMME : PaHostApiTypeId = 2;
    pub static PaASIO : PaHostApiTypeId = 3; 
    pub static PaSoundManager : PaHostApiTypeId = 4;
    pub static PaCoreAudio : PaHostApiTypeId = 5;
    pub static PaOSS : PaHostApiTypeId = 7;
    pub static PaALSA : PaHostApiTypeId = 8; 
    pub static PaAL : PaHostApiTypeId = 9;
    pub static PaBeOS : PaHostApiTypeId = 10;
    pub static PaWDMKS : PaHostApiTypeId = 11;
    pub static PaJACK : PaHostApiTypeId = 12;
    pub static PaWASAPI : PaHostApiTypeId = 13;
    pub static PaAudioScienceHPI : PaHostApiTypeId = 14;
}

/// A type used to specify one or more sample formats.
#[repr(u64)]
#[deriving(Clone, Eq, Ord, ToStr)]
pub enum PaSampleFormat {
    /// 32 bits float sample format
    PaFloat32 =         ffi::PaFloat32,
    /// 32 bits int sample format
    PaInt32 =           ffi::PaInt32,
    /// 16 bits int sample format
    PaInt16 =           ffi::PaInt16,
    /// 8 bits int sample format
    PaInt8 =            ffi::PaInt8,
    /// 8 bits unsigned int sample format
    PaUInt8 =           ffi::PaUInt8,
    /// Custom sample format
    PaCustomFormat =    ffi::PaCustomFormat,
    /// Non interleaved sample format
    PaNonInterleaved =  ffi::PaNonInterleaved
}

/// The flags to pass to a stream
#[repr(u64)]
#[deriving(Clone, Eq, Ord, ToStr)]
pub enum PaStreamFlags {
    /// No flags
    PaNoFlag =                                  ffi::PaNoFlag,
    /// Disable default clipping of out of range samples.
    PaClipOff =                                 ffi::PaClipOff,
    /// Disable default dithering.
    PaDitherOff =                               ffi::PaDitherOff,    
    /// Flag requests that where possible a full duplex stream will not discard overflowed input samples without calling the stream callback.
    PaNeverDropInput =                          ffi::PaNeverDropInput,
    /// Call the stream callback to fill initial output buffers, rather than the default behavior of priming the buffers with zeros (silence)
    PaPrimeOutputBuffersUsingStreamCallback =   ffi::PaPrimeOutputBuffersUsingStreamCallback,
    /// A mask specifying the platform specific bits.
    PaPlatformSpecificFlags =                   ffi::PaPlatformSpecificFlags
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
pub enum PaStreamCallbackResult { 
    PaContinue = 0, 
    PaComplete = 1, 
    PaAbort = 2 
}

/// Error codes returned by PortAudio functions.
#[repr(C)]
#[deriving(Clone, Eq, Ord, ToStr)]
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
#[deriving(Clone, Eq, Ord, ToStr)]
pub enum PaHostApiTypeId {
    /// In development host
    PaInDevelopment =   ffi::PaInDevelopment,
    /// Direct sound
    PaDirectSound =     ffi::PaDirectSound,
    /// MMe API
    PaMME =             ffi::PaMME,
    /// ASIO API
    PaASIO =            ffi::PaASIO,
    /// Sound manager API
    PaSoundManager =    ffi::PaSoundManager,
    /// Core Audio API
    PaCoreAudio =       ffi::PaCoreAudio,
    /// OSS API
    PaOSS =             ffi::PaOSS,
    /// Alsa API
    PaALSA =            ffi::PaALSA,
    /// AL API
    PaAL =              ffi::PaAL,
    /// BeOS API
    PaBeOS =            ffi::PaBeOS,
    /// WDMKS
    PaWDMKS =           ffi::PaWDMKS,
    /// Jack API
    PaJACK =            ffi::PaJACK,
    /// WASAPI
    PaWASAPI =          ffi::PaWASAPI,
    /// Audio Science HPI
    PaAudioScienceHPI = ffi::PaAudioScienceHPI
}

/// A structure containing information about a particular host API.
pub struct PaHostApiInfo{
    /// The version of the struct
    struct_version : int,
    /// The type of the current host
    host_type : PaHostApiTypeId,
    /// The name of the host
    name : ~str,
    /// The total count of device in the host
    device_count : int,
    /// The index to the default input device
    default_input_device : PaDeviceIndex,
    /// The index to the default output device
    default_output_device : PaDeviceIndex
}

#[doc(hidden)]
pub struct C_PaHostApiInfo {
    struct_version : i32,
    host_type : i32,
    name : *c_char,
    device_count : i32,
    default_input_device : i32,
    default_output_device : i32
}

#[doc(hidden)]
impl PaHostApiInfo {
    pub fn wrap(c_info : *C_PaHostApiInfo) -> PaHostApiInfo {
        unsafe {
            PaHostApiInfo {
                struct_version : (*c_info).struct_version as int,
                host_type : cast::transmute(((*c_info).host_type)),
                name : str::raw::from_c_str((*c_info).name),
                device_count : (*c_info).device_count as int,
                default_input_device : (*c_info).default_input_device,
                default_output_device : (*c_info).default_output_device
            }
        }
    }

    pub fn unwrap(&self) -> C_PaHostApiInfo {
        C_PaHostApiInfo {
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
#[deriving(Clone, Eq, Ord, ToStr)]
pub struct PaHostErrorInfo {
    /// The code of the error
    error_code : u32,
    /// The string which explain the error
    error_text : ~str
}

#[doc(hidden)]
pub struct C_PaHostErrorInfo {
    error_code : u32,
    error_text : *c_char
}

#[doc(hidden)]
impl PaHostErrorInfo {
    pub fn wrap(c_error : *C_PaHostErrorInfo) -> PaHostErrorInfo {
        PaHostErrorInfo {
            error_code : unsafe { (*c_error).error_code },
            error_text : unsafe { str::raw::from_c_str((*c_error).error_text) }
        }
    }

    pub fn unwrap(&self) -> C_PaHostErrorInfo {
        C_PaHostErrorInfo {
            error_code : self.error_code,
            error_text : unsafe { self.error_text.to_c_str().unwrap() }
        }
    }
}

/// A structure providing information and capabilities of PortAudio devices. Devices may support input, output or both input and output.
#[deriving(Clone, Eq, Ord, ToStr)]
pub struct PaDeviceInfo {
    /// The version of the struct
    struct_version : int,
    /// The name of the devie
    name : ~str,
    /// Host API identifier
    host_api : PaHostApiIndex,
    /// Maximal number of input channels for this device
    max_input_channels : int,
    /// maximal number of output channel for this device
    max_output_channels : int,
    /// The default low latency for input with this device
    default_low_input_latency : PaTime,
    /// The default low latency for output with this device
    default_low_output_latency : PaTime,
    /// The default high latency for input with this device
    default_high_input_latency : PaTime,
    /// The default high latency for output with this device
    default_high_output_latency : PaTime,
    /// The default sample rate for this device
    default_sample_rate : f64
}

#[doc(hidden)]
pub struct C_PaDeviceInfo {
    struct_version : i32,
    name : *c_char,
    host_api : PaHostApiIndex,
    max_input_channels : i32,
    max_output_channels : i32, 
    default_low_input_latency : PaTime,
    default_low_output_latency : PaTime,
    default_high_input_latency : PaTime,
    default_high_output_latency : PaTime,
    default_sample_rate : c_double
}

#[doc(hidden)]
impl PaDeviceInfo {
    pub fn wrap(c_info : *C_PaDeviceInfo) -> PaDeviceInfo {
        unsafe {
            PaDeviceInfo {
                struct_version : (*c_info).struct_version as int,
                name : str::raw::from_c_str((*c_info).name),
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

    pub fn unwrap(&self) -> C_PaDeviceInfo {
        C_PaDeviceInfo {
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
#[deriving(Clone, Eq, Ord, ToStr)]
pub struct PaStreamParameters {
    /// Index of the device
    device : PaDeviceIndex,
    /// The number of channels for this device
    channel_count : i32,
    /// Sample format of the device
    sample_format : PaSampleFormat,
    /// The suggested latency for this device
    suggested_latency : PaTime, 
}

#[doc(hidden)]
pub struct C_PaStreamParameters {
    device : PaDeviceIndex,
    channel_count : i32,
    sample_format : PaSampleFormat,
    suggested_latency : PaTime, 
    host_api_specific_stream_info : *c_void
}

#[doc(hidden)]
impl PaStreamParameters {
    pub fn wrap(c_parameters : *C_PaStreamParameters) -> PaStreamParameters {
        unsafe {
            PaStreamParameters {
                device : (*c_parameters).device,
                channel_count : (*c_parameters).channel_count,
                sample_format : (*c_parameters).sample_format,
                suggested_latency : (*c_parameters).suggested_latency
            }
        }
    }

    pub fn unwrap(&self) -> C_PaStreamParameters {
        C_PaStreamParameters {
            device : self.device,
            channel_count : self.channel_count as i32,
            sample_format : self.sample_format,
            suggested_latency : self.suggested_latency,
            host_api_specific_stream_info : ptr::null()
        }
    }
}


#[doc(hidden)]
pub struct PaStreamCallbackTimeInfo {
    input_buffer_adc_time : PaTime,
    current_time : PaTime,
    output_buffer_dac_time : PaTime
}

/// A structure containing unchanging information about an open stream.
#[deriving(Clone, Eq, Ord, ToStr)]
pub struct PaStreamInfo {
    /// Struct version
    struct_version : i32,
    /// The input latency for this open stream
    input_latency : PaTime,
    /// The output latency for this open stream
    output_latency : PaTime,
    /// The sample rate for this open stream
    sample_rate : f64
}

