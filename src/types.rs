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

use ffi;
use num::FromPrimitive;
use std::os::raw;

pub use self::sample_format_flags::SampleFormatFlags;


/// The type used to refer to audio devices.
///
/// Values of this type usually range from 0 to (PortAudio::device_count-1).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceIndex(pub u32);

/// The device to be used by some stream.
///
/// This is used as a field within the Settings for a **Stream**.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    /// An index to some Device.
    Index(DeviceIndex),
    /// Indicates that the device(s) to be used are specified in the host api specific stream info
    /// structure.
    UseHostApiSpecificDeviceSpecification,
}


impl From<DeviceIndex> for ffi::PaDeviceIndex {
    fn from(idx: DeviceIndex) -> ffi::PaDeviceIndex {
        let DeviceIndex(idx) = idx;
        idx as ffi::PaDeviceIndex
    }
}

impl From<DeviceIndex> for DeviceKind {
    fn from(idx: DeviceIndex) -> DeviceKind {
        DeviceKind::Index(idx)
    }
}

impl From<DeviceKind> for ffi::PaDeviceIndex {
    fn from(kind: DeviceKind) -> ffi::PaDeviceIndex {
        match kind {
            DeviceKind::Index(idx) => idx.into(),
            DeviceKind::UseHostApiSpecificDeviceSpecification => -2,
        }
    }
}


/// The special value may be used to request that the stream callback will receive an optimal (and
/// possibly varying) number of frames based on host requirements and the requested latency
/// settings.
pub const FRAMES_PER_BUFFER_UNSPECIFIED: u32 = 0;

/// The type used to enumerate to host APIs at runtime.
/// Values of this type range from 0 to (pa::get_host_api_count()-1).
pub type HostApiIndex = ffi::PaHostApiIndex;

/// The type used to represent monotonic time in seconds.
pub type Time = ffi::PaTime;

/// An type alias used to represent a given number of frames.
pub type Frames = i64;

/// A type used to dynamically represent the various standard sample formats (usually) supported by
/// all PortAudio implementations.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    /// Uses -1.0 and +1.0 as the minimum and maximum respectively.
    F32,
    /// 32-bit signed integer sample representation.
    I32,
    /// 24-bit signed integer sample representation.
    ///
    /// TODO: Should work out how to support this properly.
    I24,
    /// 16-bit signed integer sample representation.
    I16,
    /// 8-bit signed integer sample representation.
    I8,
    /// An unsigned 8 bit format where 128 is considered "ground"
    U8,
    /// Some custom sample format.
    ///
    /// TODO: Need to work out how to support this properly. I was unable to find any official
    /// info.
    ///
    /// The following e-mail by Bencina (2004) touches on the topic of custom formats:
    ///
    /// > "It is theoretically possible to pass "custom" data formats to PortAudio using the
    /// paCustomFormat SampleFormat flag. I think the general idea is that when this bit is set,
    /// the low word of the sample format byte is device specific. I know of no implementation that
    /// has ever used this feature so it has not been extensively developed. That said, much of
    /// PortAudio (V19 at least) assumes a frame based sample format, accomodating a block based
    /// format such as mpeg would probably require bypassing some of the internal infrastructure
    /// (such as the block adapter in pa_process). PortAudio has been designed for linear, frame
    /// based i/o, so it would be up to you to propose/suggest ways in which to accomodate your
    /// requirements." - http://music.columbia.edu/pipermail/portaudio/2004-February/003237.html
    Custom,
    /// This variant is used when none of the above variants can be inferred from a given
    /// set of **SampleFormatFlags** via the `SampleFormat::from_flags` function.
    Unknown,
}

impl SampleFormat {

    /// Inspects the given **SampleFormatFlags** for the format.
    ///
    /// Returns `Some(SampleFormat)` if a matching format is found.
    ///
    /// Returns `None` if no matching format is found.
    pub fn from_flags(flags: SampleFormatFlags) -> Self {
        if flags.contains(sample_format_flags::FLOAT_32) {
            SampleFormat::F32
        } else if flags.contains(sample_format_flags::INT_32) {
            SampleFormat::I32
        } else if flags.contains(sample_format_flags::INT_24) {
            SampleFormat::I24
        } else if flags.contains(sample_format_flags::INT_16) {
            SampleFormat::I16
        } else if flags.contains(sample_format_flags::INT_8) {
            SampleFormat::I8
        } else if flags.contains(sample_format_flags::UINT_8) {
            SampleFormat::U8
        } else if flags.contains(sample_format_flags::CUSTOM_FORMAT) {
            SampleFormat::Custom
        } else {
            SampleFormat::Unknown
        }
    }

    /// Converts `self` into the respective **SampleFormatFlags**.
    pub fn flags(self) -> SampleFormatFlags {
        match self {
            SampleFormat::F32 => sample_format_flags::FLOAT_32,
            SampleFormat::I32 => sample_format_flags::INT_32,
            SampleFormat::I24 => sample_format_flags::INT_24,
            SampleFormat::I16 => sample_format_flags::INT_16,
            SampleFormat::I8 => sample_format_flags::INT_8,
            SampleFormat::U8 => sample_format_flags::UINT_8,
            SampleFormat::Custom => sample_format_flags::CUSTOM_FORMAT,
            SampleFormat::Unknown => SampleFormatFlags::empty(),
        }
    }

    /// Returns the size of the **SampleFormat** in bytes.
    ///
    /// Returns `0` if the **SampleFormat** is **Custom** or **Unknown**.
    pub fn size_in_bytes(&self) -> u8 {
        match *self {
            SampleFormat::F32 | SampleFormat::I32 => 4,
            SampleFormat::I24 => 3,
            SampleFormat::I16 => 2,
            SampleFormat::I8 | SampleFormat::U8 => 1,
            SampleFormat::Custom | SampleFormat::Unknown => 0,
        }
    }

}

pub mod sample_format_flags {
    //! A type safe wrapper around PortAudio's `PaSampleFormat` flags.
    use ffi;
    bitflags! {
        /// A type used to specify one or more sample formats. Each value indicates a possible
        /// format for sound data passed to and from the stream callback, Pa_ReadStream and
        /// Pa_WriteStream.
        ///
        /// The standard formats paFloat32, paInt16, paInt32, paInt24, paInt8 and aUInt8 are
        /// usually implemented by all implementations.
        ///
        /// The floating point representation (FLOAT_32) uses +1.0 and -1.0 as the maximum and
        /// minimum respectively.
        ///
        /// UINT_8 is an unsigned 8 bit format where 128 is considered "ground"
        ///
        /// The paNonInterleaved flag indicates that audio data is passed as an array of pointers
        /// to separate buffers, one buffer for each channel. Usually, when this flag is not used,
        /// audio data is passed as a single buffer with all channels interleaved.
        pub flags SampleFormatFlags: u64 {
            /// 32 bits float sample format
            const FLOAT_32 = ffi::PA_FLOAT_32,
            /// 32 bits int sample format
            const INT_32 = ffi::PA_INT_32,
            /// Packed 24 bits int sample format
            const INT_24 = ffi::PA_INT_24,
            /// 16 bits int sample format
            const INT_16 = ffi::PA_INT_16,
            /// 8 bits int sample format
            const INT_8 = ffi::PA_INT_8,
            /// 8 bits unsigned int sample format
            const UINT_8 = ffi::PA_UINT_8,
            /// Custom sample format
            const CUSTOM_FORMAT = ffi::PA_CUSTOM_FORMAT,
            /// Non interleaved sample format
            const NON_INTERLEAVED = ffi::PA_NON_INTERLEAVED,
        }
    }

    impl From<ffi::SampleFormat> for SampleFormatFlags {
        fn from(format: ffi::SampleFormat) -> Self {
            SampleFormatFlags::from_bits(format)
                .unwrap_or_else(|| SampleFormatFlags::empty())
        }
    }

    impl ::std::fmt::Display for SampleFormatFlags {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{:?}", match self.bits() {
                ffi::PA_FLOAT_32 => "FLOAT_32",
                ffi::PA_INT_32 => "INT_32",
                //ffi::PA_INT_24 => "INT_24",
                ffi::PA_INT_16 => "INT_16",
                ffi::PA_INT_8 => "INT_8",
                ffi::PA_UINT_8 => "UINT_8",
                ffi::PA_CUSTOM_FORMAT => "CUSTOM_FORMAT",
                ffi::PA_NON_INTERLEAVED => "NON_INTERLEAVED",
                _   => "<Unknown SampleFormatFlags>",
            })
        }
    }
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

impl HostApiTypeId {
    /// Convert the given ffi::HostApiTypeId to a HostApiTypeId.
    // XXX returning an option it still necessary?
    pub fn from_c_id(c_id: ffi::PaHostApiTypeId) -> Option<Self> {
        use self::ffi::PaHostApiTypeId as C;
        use HostApiTypeId::*;
        let id = match c_id {
            C::paInDevelopment => InDevelopment,
            C::paDirectSound => DirectSound,
            C::paMME => MME,
            C::paASIO => ASIO,
            C::paSoundManager => SoundManager,
            C::paCoreAudio => CoreAudio,
            C::paOSS => OSS,
            C::paALSA => ALSA,
            C::paAL => AL,
            C::paBeOS => BeOS,
            C::paWDMKS => WDMKS,
            C::paJACK => JACK,
            C::paWASAPI => WASAPI,
            C::paAudioScienceHPI => AudioScienceHPI,
        };
        Some(id)
    }
}

/// A structure containing information about a particular host API.
#[derive(Clone, Debug, PartialEq)]
pub struct HostApiInfo<'a> {
    /// The version of the struct
    pub struct_version: i32,
    /// The type of the current host
    pub host_type: HostApiTypeId,
    /// The name of the host
    pub name: &'a str,
    /// The total count of device in the host
    pub device_count: u32,
    /// The index to the default input device or None if no input device is available
    pub default_input_device: Option<DeviceIndex>,
    /// The index to the default output device or None if no output device is available
    pub default_output_device: Option<DeviceIndex>,
}

impl<'a> HostApiInfo<'a> {

    /// Construct the HostApiInfo from the equivalent C struct.
    ///
    /// Returns `None` if:
    /// - either of the given device indices are invalid.
    /// - the device_count is less than `0`.
    /// - a valid `HostApiTypeId` can't be constructed from the given `host_type`.
    pub fn from_c_info(c_info: ffi::PaHostApiInfo) -> Option<HostApiInfo<'a>> {
        let default_input_device = match c_info.defaultInputDevice {
            idx if idx >= 0 => Some(DeviceIndex(idx as u32)),
            ffi::PA_NO_DEVICE => None,
            _ => return None,
        };
        let default_output_device = match c_info.defaultOutputDevice {
            idx if idx >= 0 => Some(DeviceIndex(idx as u32)),
            ffi::PA_NO_DEVICE => None,
            _ => return None,
        };
        let device_count = match c_info.deviceCount {
            n if n >= 0 => n as u32,
            _ => return None,
        };
        let host_type = match HostApiTypeId::from_c_id(c_info.type_) {
            Some(ty) => ty,
            None => return None,
        };
        Some(HostApiInfo {
            struct_version: c_info.structVersion,
            host_type: host_type,
            name: ffi::c_str_to_str(c_info.name)
                .unwrap_or("<Failed to convert str from CStr>"),
            device_count: device_count,
            default_input_device: default_input_device,
            default_output_device: default_output_device,
        })
    }

}

impl<'a> From<HostApiInfo<'a>> for ffi::PaHostApiInfo {
    fn from(info: HostApiInfo<'a>) -> Self {
        let default_input_device = match info.default_input_device {
            Some(i) => i.into(),
            None    => ffi::PA_NO_DEVICE,
        };
        let default_output_device = match info.default_output_device {
            Some(i) => i.into(),
            None    => ffi::PA_NO_DEVICE,
        };
        ffi::PaHostApiInfo {
            structVersion: info.struct_version as raw::c_int,
            type_: FromPrimitive::from_i32(info.host_type as i32).unwrap(),
            name: ffi::str_to_c_str(info.name),
            deviceCount: info.device_count as raw::c_int,
            defaultInputDevice: default_input_device,
            defaultOutputDevice: default_output_device,
        }
    }
}

/// Structure used to return information about a host error condition.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct HostErrorInfo<'a> {
    /// The host API which returned the error code
    pub host_api_type: HostApiTypeId,
    /// The code of the error
    pub code: u32,
    /// The string which explain the error
    pub text: &'a str,
}

impl<'a> HostErrorInfo<'a> {
    /// Construct a HostErrorInfo from the equivalent C struct.
    pub fn from_c_error_info(c_error: ffi::PaHostErrorInfo) -> HostErrorInfo<'a> {
        HostErrorInfo {
            host_api_type: HostApiTypeId::from_c_id(c_error.hostApiType).unwrap(),
            code: c_error.errorCode as u32,
            text: ffi::c_str_to_str(c_error.errorText)
                .unwrap_or("<Failed to convert str from CStr>"),
        }
    }
}

impl<'a> From<HostErrorInfo<'a>> for ffi::PaHostErrorInfo {
    fn from(error: HostErrorInfo<'a>) -> Self {
        ffi::PaHostErrorInfo {
            hostApiType: ::num::FromPrimitive::from_i32(error.host_api_type as i32).unwrap(),
            errorCode: error.code as raw::c_long,
            errorText: ffi::str_to_c_str(error.text)
        }
    }
}

/// A structure providing information and capabilities of PortAudio devices.
///
/// Devices may support input, output or both input and output.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct DeviceInfo<'a> {
    /// The version of the struct
    pub struct_version: i32,
    /// The name of the device
    pub name: &'a str,
    /// Host API identifier
    pub host_api: HostApiIndex,
    /// Maximal number of input channels for this device
    pub max_input_channels: i32,
    /// maximal number of output channel for this device
    pub max_output_channels: i32,
    /// The default low latency for input with this device
    pub default_low_input_latency: Time,
    /// The default low latency for output with this device
    pub default_low_output_latency: Time,
    /// The default high latency for input with this device
    pub default_high_input_latency: Time,
    /// The default high latency for output with this device
    pub default_high_output_latency: Time,
    /// The default sample rate for this device
    pub default_sample_rate: f64
}

impl<'a> DeviceInfo<'a> {

    /// Construct a **DeviceInfo** from the equivalent C struct.
    pub fn from_c_info(c_info: ffi::PaDeviceInfo) -> DeviceInfo<'a> {
        DeviceInfo {
            struct_version: c_info.structVersion,
            name: ffi::c_str_to_str(c_info.name)
                .unwrap_or("<Failed to convert str from CStr>"),
            host_api: c_info.hostApi,
            max_input_channels: c_info.maxInputChannels,
            max_output_channels: c_info.maxOutputChannels,
            default_low_input_latency: c_info.defaultLowInputLatency,
            default_low_output_latency: c_info.defaultLowOutputLatency,
            default_high_input_latency: c_info.defaultHighInputLatency,
            default_high_output_latency: c_info.defaultHighOutputLatency,
            default_sample_rate: c_info.defaultSampleRate
        }
    }

}

impl<'a> From<DeviceInfo<'a>> for ffi::PaDeviceInfo {
    fn from(info: DeviceInfo<'a>) -> Self {
        ffi::PaDeviceInfo {
            structVersion: info.struct_version as raw::c_int,
            name: ffi::str_to_c_str(info.name),
            hostApi: info.host_api as ffi::PaHostApiIndex,
            maxInputChannels: info.max_input_channels as raw::c_int,
            maxOutputChannels: info.max_output_channels as raw::c_int,
            defaultLowInputLatency: info.default_low_input_latency,
            defaultLowOutputLatency: info.default_low_output_latency,
            defaultHighInputLatency: info.default_high_input_latency,
            defaultHighOutputLatency: info.default_high_output_latency,
            defaultSampleRate: info.default_sample_rate
        }
    }
}
