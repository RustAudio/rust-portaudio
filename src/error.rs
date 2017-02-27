//!
//! A module for implementing the Portaudio Error type and
//! implementing the std Error trait.
//!

use ffi::PaErrorCode;

/// Error codes returned by PortAudio functions.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub enum Error {
    /// No Error
    NoError = 0,
    /// Portaudio not initialized
    NotInitialized = -10000,
    /// Unanticipated error from the host
    UnanticipatedHostError,
    /// Invalid channel count
    InvalidChannelCount,
    /// Invalid sample rate
    InvalidSampleRate,
    /// Invalid Device
    InvalidDevice,
    /// Invalid Flag
    InvalidFlag,
    /// The Sample format is not supported
    SampleFormatNotSupported,
    /// Input device not compatible with output device
    BadIODeviceCombination,
    /// Memory insufficient
    InsufficientMemory,
    /// The buffer is too big
    BufferTooBig,
    /// The buffer is too small
    BufferTooSmall,
    /// Invalid callback
    NullCallback,
    /// Invalid Stream
    BadStreamPtr,
    /// Time out
    TimedOut,
    /// Portaudio internal error
    InternalError,
    /// Device unavailable
    DeviceUnavailable,
    /// Stream info not compatible with the host
    IncompatibleHostApiSpecificStreamInfo,
    /// The stream is stopped
    StreamIsStopped,
    /// The stream is not stopped
    StreamIsNotStopped,
    /// The input stream has overflowed
    InputOverflowed,
    /// The output has underflowed
    OutputUnderflowed,
    /// The host API is not found by Portaudio
    HostApiNotFound,
    /// The host API is invalid
    InvalidHostApi,
    /// Portaudio cannot read from the callback stream
    CanNotReadFromACallbackStream,
    /// Portaudio cannot write to the callback stream
    CanNotWriteToACallbackStream,
    /// Portaudio cannot read from an output only stream
    CanNotReadFromAnOutputOnlyStream,
    /// Portaudio cannot write to an input only stream
    CanNotWriteToAnInputOnlyStream,
    /// The stream is not compatible with the host API
    IncompatibleStreamHostApi,
    /// Invalid buffer
    BadBufferPtr,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoError => "No Error",
            Error::NotInitialized => "PortAudio not initialized",
            Error::UnanticipatedHostError => "Unanticipated error from the host",
            Error::InvalidChannelCount => "Invalid number of channels",
            Error::InvalidSampleRate => "Invalid sample rate",
            Error::InvalidDevice => "Invalid device",
            Error::InvalidFlag => "Invalid flag",
            Error::SampleFormatNotSupported => "Sample format is not supported",
            Error::BadIODeviceCombination => "Input device not compatible with output device",
            Error::InsufficientMemory => "Memory insufficient",
            Error::BufferTooBig => "The buffer is too big",
            Error::BufferTooSmall => "The buffer is too small",
            Error::NullCallback => "Invalid callback",
            Error::BadStreamPtr => "Invalid stream",
            Error::TimedOut => "Time out",
            Error::InternalError => "Portaudio internal error",
            Error::DeviceUnavailable => "Device unavailable",
            Error::IncompatibleHostApiSpecificStreamInfo => "Stream info not compatible with the host",
            Error::StreamIsStopped => "The stream is stopped",
            Error::StreamIsNotStopped => "The stream is not stopped",
            Error::InputOverflowed => "The input stream has overflowed",
            Error::OutputUnderflowed => "The output stream has underflowed",
            Error::HostApiNotFound => "The host api is not found by Portaudio",
            Error::InvalidHostApi => "The host API is invalid",
            Error::CanNotReadFromACallbackStream => "Portaudio cannot read from the callback stream",
            Error::CanNotWriteToACallbackStream => "Portaudio cannot write to the callback stream",
            Error::CanNotReadFromAnOutputOnlyStream => "Portaudio cannot read from an output only stream",
            Error::CanNotWriteToAnInputOnlyStream => "Portaudio cannot write to an input only stream",
            Error::IncompatibleStreamHostApi => "The stream is not compatible with the host API",
            Error::BadBufferPtr => "Invalid buffer",
        }
    }
}

impl ::num::FromPrimitive for Error {

    fn from_i64(n: i64) -> Option<Error> {
        let error_code: PaErrorCode = ::num::FromPrimitive::from_i64(n).unwrap();
        Some(Error::from(error_code))
    }

    fn from_u64(n: u64) -> Option<Error> {
        ::num::FromPrimitive::from_i64(n as i64)
    }
    
}

impl From<PaErrorCode> for Error {
    fn from(error: PaErrorCode) -> Error {
        use ffi::PaErrorCode as C;
        use self::Error::*;
        match error {
            C::paNoError => NoError,
            C::paNotInitialized => NotInitialized,
            C::paUnanticipatedHostError => UnanticipatedHostError,
            C::paInvalidChannelCount => InvalidChannelCount,
            C::paInvalidSampleRate => InvalidSampleRate,
            C::paInvalidDevice => InvalidDevice,
            C::paInvalidFlag => InvalidFlag,
            C::paSampleFormatNotSupported => SampleFormatNotSupported,
            C::paBadIODeviceCombination => BadIODeviceCombination,
            C::paInsufficientMemory => InsufficientMemory,
            C::paBufferTooBig => BufferTooBig,
            C::paBufferTooSmall => BufferTooSmall,
            C::paNullCallback => NullCallback,
            C::paBadStreamPtr => BadStreamPtr,
            C::paTimedOut => TimedOut,
            C::paInternalError => InternalError,
            C::paDeviceUnavailable => DeviceUnavailable,
            C::paIncompatibleHostApiSpecificStreamInfo => IncompatibleHostApiSpecificStreamInfo,
            C::paStreamIsStopped => StreamIsStopped,
            C::paStreamIsNotStopped => StreamIsNotStopped,
            C::paInputOverflowed => InputOverflowed,
            C::paOutputUnderflowed => OutputUnderflowed,
            C::paHostApiNotFound => HostApiNotFound,
            C::paInvalidHostApi => InvalidHostApi,
            C::paCanNotReadFromACallbackStream => CanNotReadFromACallbackStream,
            C::paCanNotWriteToACallbackStream => CanNotWriteToACallbackStream,
            C::paCanNotReadFromAnOutputOnlyStream => CanNotReadFromAnOutputOnlyStream,
            C::paCanNotWriteToAnInputOnlyStream => CanNotWriteToAnInputOnlyStream,
            C::paIncompatibleStreamHostApi => IncompatibleStreamHostApi,
            C::paBadBufferPtr => BadBufferPtr,
        }
    }
}
