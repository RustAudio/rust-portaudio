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

//! The portable PortAudio API.

use ffi;
use libc::c_double;
use std::ptr;

pub use self::error::Error;
pub use self::stream::Stream;
pub use self::types::{
    DeviceIndex,
    DeviceInfo,
    Frames,
    HostApiIndex,
    HostApiInfo,
    HostApiTypeId,
    HostErrorInfo,
    SampleFormat,
    StreamAvailable,
    stream_callback_flags,
    StreamCallbackFlags,
    StreamCallbackTimeInfo,
    StreamCallbackResult,
    stream_flags,
    StreamFlags,
    StreamInfo,
    StreamParameters,
    Time,
    FRAMES_PER_BUFFER_UNSPECIFIED,
    NO_DEVICE,
    USE_HOST_API_SPECIFIC_DEVICE_SPECIFICATION,
};

pub mod error;
pub mod device;
pub mod host;
pub mod stream;
mod types;


/// A type-safe wrapper around the PortAudio API.
///
/// We use a type here instead of pure functions in order to ensure correct intialisation and
/// termination of the underlying PortAudio instance.
#[derive(Debug)]
pub struct PortAudio {
    /// This is solely used for checking whether or not the PortAudio API has already been
    /// terminated manually (via the `PortAudio::terminate` method) when `Drop::drop` is called.
    is_terminated: bool,
}


impl PortAudio {

    /// Construct a **PortAudio** instance.
    ///
    /// This calls PortAudio's `Pa_Initialize` function which initializes internal data structures
    /// and prepares underlying host APIs for use.
    ///
    /// Using the C API, a user would normally have to call `Pa_Terminate` when shutting down
    /// PortAudio, however this **PortAudio** type will automatically take care of cleanup when
    /// `Drop`ped.
    ///
    /// It is safe to simultaneously construct more than one **PortAudio** instance, however this
    /// is rarely necessary.
    pub fn new() -> Result<Self, Error> {
        unsafe {
            match ffi::Pa_Initialize() {
                Error::NoError => Ok(PortAudio { is_terminated: false }),
                err => Err(err),
            }
        }
    }

    /// Takes ownership of `self` and terminates the PortAudio API using `Pa_Terminate`.
    ///
    /// This function deallocates all resources allocated by PortAudio since it was constructed.
    ///
    /// **Calling this method is optional**. It is only necessary if you require handling any
    /// PortAudio termination errors. Otherwise, `Pa_Terminate` will be called and all necessary
    /// cleanup will occur automatically when this **PortAudio** instance is **Drop**ped.
    pub fn terminate(mut self) -> Result<(), Error> {
        self.is_terminated = true;
        terminate()
    }

    /// Retrieve the release number of the currently running PortAudio build.
    pub fn version(&self) -> i32 {
        version()
    }

    /// Retrieve a textual description of the current PortAudio build.
    ///
    /// FIXME: This should return a `&'static str`, not a `String`.
    pub fn version_text(&self) -> String {
        version_text()
    }

    /// Return information about the last host error encountered.
    ///
    /// The error information returned by this method will never be modified asynchronously by
    /// errors occurring in other PortAudio owned threads (such as the thread that manages the
    /// stream callback).
    ///
    /// This method is provided as a last resort, primarily to enhance debugging by providing
    /// clients with access to all available error information.
    ///
    /// Return a pointer to an immutable structure constraining information about the host error.
    /// The values in this structure will only be valid if a PortAudio function or method has
    /// previously returned the UnanticipatedHostError error code.
    pub fn last_host_error_info(&self) -> HostErrorInfo {
        let c_error = unsafe { ffi::Pa_GetLastHostErrorInfo() };
        HostErrorInfo::wrap(c_error)
    }

    /// Determine whether it would be possible to open an input-only stream with the specified
    /// parameters.
    ///
    /// The `suggested_latency` field of the `StreamParameters` is ignored.
    ///
    /// Returns `Ok(())` if the format is supported, and an `Err(Error)` indicating why the format
    /// is not supported otherwise.
    pub fn is_input_format_supported(&self, params: &StreamParameters, sample_rate: f64)
        -> Result<(), Error>
    {
        is_format_supported(Some(params), None, sample_rate)
    }

    /// Determine whether it would be possible to open an output-only stream with the specified
    /// parameters.
    ///
    /// The `suggested_latency` field of the `StreamParameters` is ignored.
    ///
    /// Returns `Ok(())` if the format is supported, and an `Err(Error)` indicating why the format
    /// is not supported otherwise.
    pub fn is_output_format_supported(&self, params: &StreamParameters, sample_rate: f64)
        -> Result<(), Error>
    {
        is_format_supported(None, Some(params), sample_rate)
    }

    /// Determine whether it would be possible to open a duplex stream with the specified
    /// parameters.
    ///
    /// The `suggested_latency` field of the `StreamParameters` is ignored.
    ///
    /// Returns `Ok(())` if the format is supported, and an `Err(Error)` indicating why the format
    /// is not supported otherwise.
    pub fn is_duplex_format_supported(&self,
                                      in_params: &StreamParameters,
                                      out_params: &StreamParameters,
                                      sample_rate: f64)
        -> Result<(), Error>
    {
        is_format_supported(Some(in_params), Some(out_params), sample_rate)
    }

    /// Put the caller to sleep for at least 'msec' milliseconds.
    ///
    /// In the original API this function is provided only as a convenience for authors of portable
    /// code (such as the tests and examples in the PortAudio distribution). This may be removed in
    /// the future in favour of rust's `::std::thread::sleep` function.
    ///
    /// The function may sleep longer than requested so don't rely on this for accurate musical
    /// timing.
    pub fn sleep(&self, m_sec: i32) -> () {
        unsafe {
            ffi::Pa_Sleep(m_sec)
        }
    }

}


impl Drop for PortAudio {
    fn drop(&mut self) {
        if !self.is_terminated {
            terminate().ok();
        }
    }
}


/// Retrieve the release number of the currently running PortAudio build.
pub fn version() -> i32 {
    unsafe {
        ffi::Pa_GetVersion()
    }
}

/// Retrieve a textual description of the current PortAudio build.
///
/// FIXME: This should return a `&'static str`, not a `String`.
pub fn version_text() -> String {
    unsafe {
        ffi::c_str_to_string(&ffi::Pa_GetVersionText())
    }
}


/// This is used by the **PortAudio::terminate** method.
///
/// Library termination function - call this when finished using PortAudio.
///
/// This function deallocates all resources allocated by PortAudio since it was initialized by a
/// call to initialize().
///
/// In cases where initialise() has been called multiple times, each call must be matched with a
/// corresponding call to terminate().
///
/// The final matching call to terminate() will automatically close any PortAudio streams that are
/// still open.
///
/// terminate() MUST be called before exiting a program which uses PortAudio. Failure to do so may
/// result in serious resource leaks, such as audio devices not being available until the next
/// reboot.
///
/// Return NoError if successful, otherwise an error code indicating the cause of failure.
fn terminate() -> Result<(), Error> {
    unsafe {
        match ffi::Pa_Terminate() {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }
}

/// This is used by the **PortAudio::is_*_format_supported** methods.
///
/// Determine whether it would be possible to open a stream with the specified parameters.
///
/// # Arguments
/// * input_parameters - A structure that describes the input parameters used to open a stream.
/// The suggestedLatency field is ignored. See StreamParameters for a description of these
/// parameters. inputParameters must be None for output-only streams.
/// * output_parameters - A structure that describes the output parameters used to open a stream.
/// The suggestedLatency field is ignored. See StreamParameters for a description of these
/// parameters. outputParameters must be None for input-only streams.
/// * sample_rate - The required sampleRate. For full-duplex streams it is the sample rate for both
/// input and output.
///
/// Return Ok(()) if the format is supported, and an Error indicating why the format is not
/// supported otherwise. The constant PaFormatIsSupported is provided to compare with the return
/// value for success.
fn is_format_supported(maybe_input_parameters: Option<&StreamParameters>,
                       maybe_output_parameters: Option<&StreamParameters>,
                       sample_rate : f64) -> Result<(), Error>
{
    let c_input = maybe_input_parameters.map(StreamParameters::unwrap).as_ref()
        .map(|input| input as *const _);
    let c_output = maybe_output_parameters.map(StreamParameters::unwrap).as_ref()
        .map(|output| output as *const _);
    if c_input.is_none() && c_output.is_none() {
        Err(Error::InvalidDevice)
    } else {
        unsafe {
            match ffi::Pa_IsFormatSupported(c_input.unwrap_or(ptr::null()),
                                            c_output.unwrap_or(ptr::null()),
                                            sample_rate as c_double) {
                Error::NoError => Ok(()),
                err => Err(err),
            }
        }
    }
}

/// Retrieve the size of a given sample format in bytes.
///
/// Return the size in bytes of a single sample in the specified format,
/// or SampleFormatNotSupported if the format is not supported.
pub fn get_sample_size(format: SampleFormat) -> Result<u8, Error> {
    let result = unsafe { ffi::Pa_GetSampleSize(format as u64) };
    if result < 0 {
        Err(::num::FromPrimitive::from_i32(result).unwrap())
    } else {
        Ok(result as u8)
    }
}

// /// Library initialization function - call this before using PortAudio.
// /// This function initializes internal data structures and prepares underlying
// /// host APIs for use. With the exception of get_version(), get_version_text(),
// /// and get_error_text(), this function MUST be called before using any other
// /// PortAudio API functions.
// ///
// /// Note that if initialize() returns an error code, Pa_Terminate() should NOT be
// /// called.
// ///
// /// Return NoError if successful, otherwise an error code indicating the cause
// /// of failure.
// fn initialize() -> Result<(), Error> {
//     unsafe {
//         match ffi::Pa_Initialize() {
//             Error::NoError => Ok(()),
//             err => Err(err),
//         }
//     }
// }

// /// Return information about the last host error encountered.
// /// The error information returned by get_last_host_error_info() will never be
// /// modified asynchronously by errors occurring in other PortAudio owned threads
// /// (such as the thread that manages the stream callback.)
// ///
// /// This function is provided as a last resort, primarily to enhance debugging
// /// by providing clients with access to all available error information.
// ///
// /// Return a pointer to an immuspacespacespacele structure constraining
// /// information about the host error. The values in this structure will only be
// /// valid if a PortAudio function has previously returned the
// /// UnanticipatedHostError error code.
// fn get_last_host_error_info() -> HostErrorInfo {
//     let c_error = unsafe { ffi::Pa_GetLastHostErrorInfo() };
//     HostErrorInfo::wrap(c_error)
// }

mod private {

    use num::{FromPrimitive, ToPrimitive};
    use std::ops::{Add, Sub, Mul, Div};
    use super::types::SampleFormat;

    /// internal private trait for Sample format management
    pub trait SamplePrivate: ::std::default::Default + Copy + Clone + ::std::fmt::Debug
                             + ToPrimitive + FromPrimitive + Add + Sub + Mul + Div {
        /// return the size of a sample format
        fn size<S: SamplePrivate>() -> usize {
            ::std::mem::size_of::<S>()
        }
        /// get the sample format
        fn to_sample_format() -> SampleFormat;
    }

}

impl private::SamplePrivate for f32 {
    fn to_sample_format() -> SampleFormat { SampleFormat::Float32 }
}

impl private::SamplePrivate for i32 {
    fn to_sample_format() -> SampleFormat { SampleFormat::Int32 }
}

impl private::SamplePrivate for u8 {
    fn to_sample_format() -> SampleFormat { SampleFormat::UInt8 }
}

impl private::SamplePrivate for i8 {
    fn to_sample_format() -> SampleFormat { SampleFormat::Int8 }
}

/// public trait to constraint pa::Stream for specific types
pub trait Sample: private::SamplePrivate {
    /// Retrieve the SampleFormat variant associated with the type.
    fn sample_format_for<S: Sample>() -> SampleFormat {
        S::to_sample_format()
    }
    /// Retrieve the SampleFormat variant associated with the type.
    fn sample_format(&self) -> SampleFormat { Self::to_sample_format() }
}

impl Sample for f32 {}
impl Sample for i32 {}
impl Sample for i8 {}
impl Sample for u8 {}
