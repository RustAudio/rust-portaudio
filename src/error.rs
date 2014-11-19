//! 
//! A module for implementing the Portaudio Error type and
//! implementing the std Error trait.
//!


/// Error codes returned by PortAudio functions.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
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
    BadBufferPtr
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            NoError => "No Error",
            NotInitialized => "Portaudio not initialized",
            UnanticipatedHostError => "Unanticipated error from the host",
            InvalidChannelCount => "Invalid channel count",
            InvalidSampleRate => "Invalid sample rate",
            InvalidDevice => "Invalid device",
            InvalidFlag => "Invalid flag",
            SampleFormatNotSupported => "Sample format is not supported",
            BadIODeviceCombination => "Input device not compatible with output device",
            InsufficientMemory => "Memory insufficient",
            BufferTooBig => "The buffer is too big",
            BufferTooSmall => "The buffer is too small",
            NullCallback => "Invalid callback",
            BadStreamPtr => "Invalid stream",
            TimedOut => "Time out",
            InternalError => "Portaudio internal error",
            DeviceUnavailable => "Device unavailable",
            IncompatibleHostApiSpecificStreamInfo => "Stream info not compatible with the host",
            StreamIsStopped => "The stream is stopped",
            StreamIsNotStopped => "The stream is not stopped",
            InputOverflowed => "The input stream has overflowed",
            OutputUnderflowed => "The output stream has overflowed",
            HostApiNotFound => "The host api is not found by Portaudio",
            InvalidHostApi => "The host API is invalid",
            CanNotReadFromACallbackStream => "Portaudio cannot read from the callback stream",
            CanNotWriteToACallbackStream => "Portaudio cannot write to the callback stream",
            CanNotReadFromAnOutputOnlyStream => "Portaudio cannot read from an output only stream",
            CanNotWriteToAnInputOnlyStream => "Portaudio cannot write to an input only stream",
            IncompatibleStreamHostApi => "The stream is not compatible with the host API",
            BadBufferPtr => "Invalid buffer",
        }
    }
}

