//!
//! A module for implementing the Portaudio Error type and
//! implementing the std Error trait.
//!

enum_from_primitive!{
/// Error codes returned by PortAudio functions.
// FIXME enum_from_primitive does not work with the documentation
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Error {
    NoError = 0,
    NotInitialized = -10000,
    UnanticipatedHostError,
    InvalidChannelCount,
    InvalidSampleRate,
    InvalidDevice,
    InvalidFlag,
    SampleFormatNotSupported,
    BadIODeviceCombination,
    InsufficientMemory,
    BufferTooBig,
    BufferTooSmall,
    NullCallback,
    BadStreamPtr,
    TimedOut,
    InternalError,
    DeviceUnavailable,
    IncompatibleHostApiSpecificStreamInfo,
    StreamIsStopped,
    StreamIsNotStopped,
    InputOverflowed,
    OutputUnderflowed,
    HostApiNotFound,
    InvalidHostApi,
    CanNotReadFromACallbackStream,
    CanNotWriteToACallbackStream,
    CanNotReadFromAnOutputOnlyStream,
    CanNotWriteToAnInputOnlyStream,
    IncompatibleStreamHostApi,
    BadBufferPtr,
}
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

