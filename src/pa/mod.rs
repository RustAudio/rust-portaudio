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
pub use self::stream::{
    Available as StreamAvailable,
    Blocking,
    callback_flags as stream_callback_flags,
    CallbackFlags as StreamCallbackFlags,
    CallbackResult as StreamCallbackResult,
    CallbackTimeInfo as StreamCallbackTimeInfo,
    DuplexSettings as DuplexStreamSettings,
    DuplexCallbackArgs as DuplexStreamCallbackArgs,
    flags as stream_flags,
    Flags as StreamFlags,
    Flow,
    Info as StreamInfo,
    InputSettings as InputStreamSettings,
    InputCallbackArgs as InputStreamCallbackArgs,
    NonBlocking,
    Parameters as StreamParameters,
    Settings as StreamSettings,
    Stream,
};
pub use self::types::{
    DeviceIndex,
    DeviceInfo,
    Frames,
    HostApiIndex,
    HostApiInfo,
    HostApiTypeId,
    HostErrorInfo,
    SampleFormat,
    Time,
    FRAMES_PER_BUFFER_UNSPECIFIED,
};

pub mod error;
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

    /// Retrieve the number of available devices. The number of available devices may be zero.
    ///
    /// **Note:** The only error documented by PortAudio that may occur calling this method is
    /// caused by PortAudio not being initialised, however this should not be possible with our
    /// type-safe **PortAudio** API.
    pub fn device_count(&self) -> DeviceIndex {
        unsafe {
            ffi::Pa_GetDeviceCount().into()
        }
    }

    /// Retrieve the index of the default input device. The result can be used in the
    /// **InSettings** used to open an **In** **Stream**.
    ///
    /// Returns the default output device index for the default host API.
    ///
    /// Returns `None` if no default input device is available or an error was encountered.
    pub fn default_input_device(&self) -> DeviceIndex {
        unsafe {
            ffi::Pa_GetDefaultInputDevice().into()
        }
    }

    /// Retrieve the index of the default output device. The result can be used in the
    /// **OutSettings** used to open an **Out** **Stream**.
    ///
    /// Returns the default input device index for the default host API.
    ///
    /// Returns `None` if no default input device is available or an error was encountered.
    pub fn default_output_device(&self) -> DeviceIndex {
        unsafe {
            ffi::Pa_GetDefaultOutputDevice().into()
        }
    }

    /// Retrieve a **DeviceInfo** structure containing information about the specified device.
    ///
    /// Returns Ok(DeviceInfo) if successful.
    ///
    /// Returns Err(Error::InvalidDevice) if the device parameter is out of range.
    ///
    /// # Arguments
    ///
    /// - device - A valid device index in the range 0 to (port_audio.device_count()-1).
    pub fn device_info(&self, device: DeviceIndex) -> Result<DeviceInfo, Error> {
        let c_info = unsafe { ffi::Pa_GetDeviceInfo(device.into()) };
        if c_info.is_null() {
            Err(Error::InvalidDevice)
        }
        else {
            Ok(DeviceInfo::wrap(c_info))
        }
    }

    /// Retrieve the number of available host APIs.
    ///
    /// Even if a host API is available it may have no devices available.
    ///
    /// Return a non-negative value indicating the number of available host APIs or an `Error` if
    /// an error is encountered.
    ///
    /// TODO: Determine exactly what errors might occur (PA docs aren't clear on this).
    pub fn host_api_count(&self) -> Result<HostApiIndex, Error> {
        unsafe {
            result_from_host_api_index(ffi::Pa_GetHostApiCount())
        }
    }

    /// Retrieve the index of the default host API.
    ///
    /// The default host API will be the lowest common denominator host API on the current platform
    /// and is unlikely to provide the best performance.
    ///
    /// Return a non-negative value ranging from 0 to (get_host_api_count()-1) indicating the
    /// default host API index or an `Error` if an error is encountered.
    ///
    /// TODO: Determine exactly what errors might occur (PA docs aren't clear on this).
    pub fn default_host_api(&self) -> Result<HostApiIndex, Error> {
        unsafe {
            result_from_host_api_index(ffi::Pa_GetDefaultHostApi())
        }
    }

    /// Retrieve a pointer to a structure containing information about a specific host Api.
    ///
    /// # Arguments
    ///
    /// - host_api - A valid host API index ranging from 0 to (Pa_GetHostApiCount()-1)
    ///
    /// Return `Some(PaHostApiInfo)` describing a specific host API. If the `host_api` parameter is
    /// out of range or an error is encountered, the function returns `None`.
    pub fn host_api_info(&self, host_api: HostApiIndex) -> Option<HostApiInfo> {
        let c_host_info = unsafe { ffi::Pa_GetHostApiInfo(host_api as ffi::HostApiIndex) };
        if c_host_info.is_null() {
            None
        } else {
            Some(HostApiInfo::wrap(c_host_info))
        }
    }

    /// Convert a static host API unique identifier, into a runtime host API index.
    ///
    /// # Arguments
    ///
    /// - typde_id - A unique host API identifier belonging to the `PaHostApiTypeId` enumeration.
    ///
    /// Return a valid `HostApiIndex` ranging from 0 to (get_host_api_count()-1) or an `Error` if
    /// an error is encountered.
    ///
    /// TODO: Determine exactly what errors might occur (PA docs aren't clear on this).
    pub fn host_api_type_id_to_host_api_index(type_id: HostApiTypeId) -> Result<HostApiIndex, Error> {
        unsafe {
            result_from_host_api_index(ffi::Pa_HostApiTypeIdToHostApiIndex(type_id as i32))
        }
    }

    /// Convert a host-API-specific device index to standard PortAudio device index.
    ///
    /// This function may be used in conjunction with the `device_count` field of `HostApiInfo` to
    /// enumerate all devices for the specified host API.
    ///
    /// # Arguments
    ///
    /// - `host_api` - A valid host API index ranging from 0 to (get_host_api_count()-1)
    /// - `host_api_device_index` - A valid per-host device index in the range 0 to
    /// (get_host_api_info(host_api).device_count-1)
    ///
    /// Return a non-negative `DeviceIndex` ranging from 0 to (get_device_count()-1)
    /// or an `Error` if an error is encountered.
    ///
    /// TODO: Determine exactly what errors might occur (PA docs aren't clear on this).
    pub fn api_device_index_to_device_index(host_api: HostApiIndex,
                                            host_api_device_index: i32) -> DeviceIndex {
        unsafe {
            ffi::Pa_HostApiDeviceIndexToDeviceIndex(host_api as i32, host_api_device_index).into()
        }
    }

    /// Determine whether it would be possible to open an input-only stream with the specified
    /// parameters.
    ///
    /// The `suggested_latency` field of the `StreamParameters` is ignored.
    ///
    /// Returns `Ok(())` if the format is supported, and an `Err(Error)` indicating why the format
    /// is not supported otherwise.
    pub fn is_input_format_supported<I>(&self, params: StreamParameters<I>, sample_rate: f64)
        -> Result<(), Error>
        where I: Sample,
    {
        is_format_supported(Some(params.into()), None, sample_rate)
    }

    /// Determine whether it would be possible to open an output-only stream with the specified
    /// parameters.
    ///
    /// The `suggested_latency` field of the `StreamParameters` is ignored.
    ///
    /// Returns `Ok(())` if the format is supported, and an `Err(Error)` indicating why the format
    /// is not supported otherwise.
    pub fn is_output_format_supported<O>(&self, params: StreamParameters<O>, sample_rate: f64)
        -> Result<(), Error>
        where O: Sample,
    {
        is_format_supported(None, Some(params.into()), sample_rate)
    }

    /// Determine whether it would be possible to open a duplex stream with the specified
    /// parameters.
    ///
    /// The `suggested_latency` field of the `StreamParameters` is ignored.
    ///
    /// Returns `Ok(())` if the format is supported, and an `Err(Error)` indicating why the format
    /// is not supported otherwise.
    pub fn is_duplex_format_supported<I, O>(&self,
                                            in_params: StreamParameters<I>,
                                            out_params: StreamParameters<O>,
                                            sample_rate: f64)
        -> Result<(), Error>
        where I: Sample,
              O: Sample,
    {
        is_format_supported(Some(in_params.into()), Some(out_params.into()), sample_rate)
    }

    /// Spawn a new blocking [**Stream**](./stream/struct.Stream) with the given settings.
    pub fn open_blocking_stream<'a, S>(&'a self, settings: S)
        -> Result<Stream<'a, Blocking<<S::Flow as Flow>::Buffer>, S::Flow>, Error>
        where S: StreamSettings,
              S::Flow: Flow + 'a,
    {
        Stream::<'a, Blocking<<S::Flow as Flow>::Buffer>, S::Flow>::open(self, settings)
    }

    /// Spawn a new non-blocking [**Stream**](./stream/struct.Stream) with the given settings.
    pub fn open_non_blocking_stream<'a, S, C>(&'a self, settings: S, callback: C)
        -> Result<Stream<'a, NonBlocking, S::Flow>, Error>
        where S: StreamSettings,
              S::Flow: Flow + 'a,
              C: FnMut(<S::Flow as Flow>::CallbackArgs) -> StreamCallbackResult + 'static,
    {
        Stream::<'a, NonBlocking, S::Flow>::open(self, settings, callback)
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
fn is_format_supported(maybe_input_parameters: Option<ffi::C_PaStreamParameters>,
                       maybe_output_parameters: Option<ffi::C_PaStreamParameters>,
                       sample_rate : f64) -> Result<(), Error>
{
    let c_input = maybe_input_parameters.as_ref().map(|input| input as *const _);
    let c_output = maybe_output_parameters.as_ref().map(|output| output as *const _);
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


fn result_from_host_api_index(idx: ffi::HostApiIndex) -> Result<HostApiIndex, Error> {
    match idx {
        idx if idx >= 0 => Ok(idx as u16),
        err => Err(::num::FromPrimitive::from_i32(err).unwrap()),
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
    fn to_sample_format() -> SampleFormat { SampleFormat::F32 }
}

impl private::SamplePrivate for i32 {
    fn to_sample_format() -> SampleFormat { SampleFormat::I32 }
}

impl private::SamplePrivate for i16 {
    fn to_sample_format() -> SampleFormat { SampleFormat::I16 }
}

impl private::SamplePrivate for i8 {
    fn to_sample_format() -> SampleFormat { SampleFormat::I8 }
}

impl private::SamplePrivate for u8 {
    fn to_sample_format() -> SampleFormat { SampleFormat::U8 }
}

/// public trait to constraint pa::Stream for specific types
pub trait Sample: private::SamplePrivate {
    /// Retrieve the SampleFormat variant associated with the type.
    fn sample_format() -> SampleFormat { Self::to_sample_format() }
}

impl Sample for f32 {}
impl Sample for i32 {}
impl Sample for i16 {}
impl Sample for i8 {}
impl Sample for u8 {}
