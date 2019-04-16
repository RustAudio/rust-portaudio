//!
//! A module for implementing the Portaudio Error type and
//! implementing the std Error trait.
//!

use ffi;

enum_from_primitive! {
/// Error codes returned by PortAudio functions.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Error {
    /// No Error
    NoError =
        ffi::PaErrorCode_paNoError,
    /// No audio devices
    NoDevice =
        ffi::PA_NO_DEVICE,
    /// Portaudio not initialized
    NotInitialized =
        ffi::PaErrorCode_paNotInitialized,
    /// Unanticipated error from the host
    UnanticipatedHostError =
        ffi::PaErrorCode_paUnanticipatedHostError,
    /// Invalid channel count
    InvalidChannelCount =
        ffi::PaErrorCode_paInvalidChannelCount,
    /// Invalid sample rate
    InvalidSampleRate =
        ffi::PaErrorCode_paInvalidSampleRate,
    /// Invalid Device
    InvalidDevice =
        ffi::PaErrorCode_paInvalidDevice,
    /// Invalid Flag
    InvalidFlag =
        ffi::PaErrorCode_paInvalidFlag,
    /// The Sample format is not supported
    SampleFormatNotSupported =
        ffi::PaErrorCode_paSampleFormatNotSupported,
    /// Input device not compatible with output device
    BadIODeviceCombination =
        ffi::PaErrorCode_paBadIODeviceCombination,
    /// Memory insufficient
    InsufficientMemory =
        ffi::PaErrorCode_paInsufficientMemory,
    /// The buffer is too big
    BufferTooBig =
        ffi::PaErrorCode_paBufferTooBig,
    /// The buffer is too small
    BufferTooSmall =
        ffi::PaErrorCode_paBufferTooSmall,
    /// Invalid callback
    NullCallback =
        ffi::PaErrorCode_paNullCallback,
    /// Invalid Stream
    BadStreamPtr =
        ffi::PaErrorCode_paBadStreamPtr,
    /// Time out
    TimedOut =
        ffi::PaErrorCode_paTimedOut,
    /// Portaudio internal error
    InternalError =
        ffi::PaErrorCode_paInternalError,
    /// Device unavailable
    DeviceUnavailable =
        ffi::PaErrorCode_paDeviceUnavailable,
    /// Stream info not compatible with the host
    IncompatibleHostApiSpecificStreamInfo =
        ffi::PaErrorCode_paIncompatibleHostApiSpecificStreamInfo,
    /// The stream is stopped
    StreamIsStopped =
        ffi::PaErrorCode_paStreamIsStopped,
    /// The stream is not stopped
    StreamIsNotStopped =
        ffi::PaErrorCode_paStreamIsNotStopped,
    /// The input stream has overflowed
    InputOverflowed =
        ffi::PaErrorCode_paInputOverflowed,
    /// The output has underflowed
    OutputUnderflowed =
        ffi::PaErrorCode_paOutputUnderflowed,
    /// The host API is not found by Portaudio
    HostApiNotFound =
        ffi::PaErrorCode_paHostApiNotFound,
    /// The host API is invalid
    InvalidHostApi =
        ffi::PaErrorCode_paInvalidHostApi,
    /// Portaudio cannot read from the callback stream
    CanNotReadFromACallbackStream =
        ffi::PaErrorCode_paCanNotReadFromACallbackStream,
    /// Portaudio cannot write to the callback stream
    CanNotWriteToACallbackStream =
        ffi::PaErrorCode_paCanNotWriteToACallbackStream,
    /// Portaudio cannot read from an output only stream
    CanNotReadFromAnOutputOnlyStream =
        ffi::PaErrorCode_paCanNotReadFromAnOutputOnlyStream,
    /// Portaudio cannot write to an input only stream
    CanNotWriteToAnInputOnlyStream =
        ffi::PaErrorCode_paCanNotWriteToAnInputOnlyStream,
    /// The stream is not compatible with the host API
    IncompatibleStreamHostApi =
        ffi::PaErrorCode_paIncompatibleStreamHostApi,
    /// Invalid buffer
    BadBufferPtr =
        ffi::PaErrorCode_paBadBufferPtr,
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
            Error::NoDevice => "No Device",
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
            Error::IncompatibleHostApiSpecificStreamInfo => {
                "Stream info not compatible with the host"
            }
            Error::StreamIsStopped => "The stream is stopped",
            Error::StreamIsNotStopped => "The stream is not stopped",
            Error::InputOverflowed => "The input stream has overflowed",
            Error::OutputUnderflowed => "The output stream has underflowed",
            Error::HostApiNotFound => "The host api is not found by Portaudio",
            Error::InvalidHostApi => "The host API is invalid",
            Error::CanNotReadFromACallbackStream => {
                "Portaudio cannot read from the callback stream"
            }
            Error::CanNotWriteToACallbackStream => "Portaudio cannot write to the callback stream",
            Error::CanNotReadFromAnOutputOnlyStream => {
                "Portaudio cannot read from an output only stream"
            }
            Error::CanNotWriteToAnInputOnlyStream => {
                "Portaudio cannot write to an input only stream"
            }
            Error::IncompatibleStreamHostApi => "The stream is not compatible with the host API",
            Error::BadBufferPtr => "Invalid buffer",
        }
    }
}
