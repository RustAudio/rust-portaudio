/*!
* Types used in the PortAudio API
*/

use std::libc::{c_void, c_char, c_double};
use std::{str, ptr};

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

/// A type used to specify one or more sample formats.
pub type PaSampleFormat = u64;
pub static PaFloat32 : PaSampleFormat = 0x00000001;
pub static PaInt32 : PaSampleFormat = 0x00000002;
//pub static PaInt24 : PaSampleFormat = 0x00000004;
pub static PaInt16 : PaSampleFormat = 0x00000008;
pub static PaInt8 : PaSampleFormat = 0x00000010;
pub static PaUInt8 : PaSampleFormat = 0x00000020; 
pub static PaCustomFormat : PaSampleFormat = 0x00010000;
pub static PaNonInterleaved : PaSampleFormat = 0x80000000;

// pub type PaStream = c_void; 

pub type PaStreamFlags = u64;
pub static PaNoFlag : PaStreamFlags = 0;
/// Disable default clipping of out of range samples.
pub static PaClipOff : PaStreamFlags = 0x00000001;
/// Disable default dithering.
pub static PaDitherOff : PaStreamFlags = 0x00000002;
/// Flag requests that where possible a full duplex stream will not discard overflowed input samples without calling the stream callback.
pub static PaNeverDropInput : PaStreamFlags = 0x00000004;
/// Call the stream callback to fill initial output buffers, rather than the default behavior of priming the buffers with zeros (silence)
pub static PaPrimeOutputBuffersUsingStreamCallback : PaStreamFlags = 0x00000008;
/// A mask specifying the platform specific bits.
pub static PaPlatformSpecificFlags : PaStreamFlags = 0xFFFF0000;

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
pub enum PaError { 
    PaNoError = 0,
    PaNotInitialized = -10000,
    PaUnanticipatedHostError,
    PaInvalidChannelCount, 
    PaInvalidSampleRate,
    PaInvalidDevice,
    PaInvalidFlag,
    PaSampleFormatNotSupported, 
    PaBadIODeviceCombination,
    PaInsufficientMemory,
    PaBufferTooBig,
    PaBufferTooSmall, 
    PaNullCallback,
    PaBadStreamPtr,
    PaTimedOut,
    PaInternalError, 
    PaDeviceUnavailable,
    PaIncompatibleHostApiSpecificStreamInfo,
    PaStreamIsStopped,
    PaStreamIsNotStopped, 
    PaInputOverflowed,
    PaOutputUnderflowed,
    PaHostApiNotFound,
    PaInvalidHostApi, 
    PaCanNotReadFromACallbackStream,
    PaCanNotWriteToACallbackStream,
    PaCanNotReadFromAnOutputOnlyStream,
    PaCanNotWriteToAnInputOnlyStream, 
    PaIncompatibleStreamHostApi,
    PaBadBufferPtr 
}

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

/// A structure containing information about a particular host API.
pub struct PaHostApiInfo{
    struct_version : int,
    host_type : PaHostApiTypeId,
    name : ~str,
    device_count : int,
    default_input_device : PaDeviceIndex,
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
                host_type : ((*c_info).host_type),
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
pub struct PaHostErrorInfo {
    error_code : u32,
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
pub struct PaDeviceInfo {
    struct_version : int,
    name : ~str,
    host_api : PaHostApiIndex,
    max_input_channels : int,
    max_output_channels : int, 
    default_low_input_latency : PaTime,
    default_low_output_latency : PaTime,
    default_high_input_latency : PaTime,
    default_high_output_latency : PaTime,
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
pub struct PaStreamParameters {
    device : PaDeviceIndex,
    channel_count : i32,
    sample_format : PaSampleFormat,
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
pub struct PaStreamInfo {
    struct_version : i32,
    input_latency : PaTime,
    output_latency : PaTime,
    sample_rate : f64
}

