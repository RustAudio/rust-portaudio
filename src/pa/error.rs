//!
//! A module for implementing the Portaudio Error type and
//! implementing the std Error trait.
//!

/// Error codes returned by PortAudio functions.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, FromPrimitive)]
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
    /// The output has overflowed
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
        self.fmt(f)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoError => "No Error",
            Error::NotInitialized => "Portaudio not initialized",
            Error::UnanticipatedHostError => "Unanticipated error from the host",
            Error::InvalidChannelCount => "Invalid channel count",
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
            Error::OutputUnderflowed => "The output stream has overflowed",
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

