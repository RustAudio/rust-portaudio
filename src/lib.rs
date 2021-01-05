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

//! # rust-portaudio
//!
//! __PortAudio__ bindings for Rust
//!
//! PortAudio provides a uniform application programming interface (API) across all
//! supported platforms.  You can think of the PortAudio library as a wrapper that
//! converts calls to the PortAudio API into calls to platform-specific native audio
//! APIs. Operating systems often offer more than one native audio API and some APIs
//! (such as JACK) may be available on multiple target operating systems.
//! PortAudio supports all the major native audio APIs on each supported platform.
//!
//! # Installation
//!
//! rust-portaudio's build script will check to see if you have already installed
//! PortAudio on your system. If not, it will attempt to automatically download and
//! install it for you. If this fails, please let us know by posting an issue at [our
//! github repository] (https://github.com/jeremyletang/rust-portaudio).
//!
//! If you'd prefer to install it manually, you can download it directly from the website:
//! [PortAudio](http://www.portaudio.com/download.html).
//!
//! # Usage
//!
//! Add rust-portaudio to your project by adding the dependency to your Cargo.toml as follows:
//!
//! ```toml
//! [dependencies]
//! portaudio = "*"
//! ```

#![warn(missing_docs)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate num;
extern crate portaudio_sys as ffi;

use num::FromPrimitive;
use std::option::Option;
use std::os::raw;

pub use error::Error;
pub use ffi::{
    PaStreamCallbackResult as StreamCallbackResult, PA_ABORT as Abort, PA_COMPLETE as Complete,
    PA_CONTINUE as Continue,
};
pub use stream::{
    callback_flags as stream_callback_flags, flags as stream_flags, Available as StreamAvailable,
    Blocking, CallbackFlags as StreamCallbackFlags, CallbackTimeInfo as StreamCallbackTimeInfo,
    Duplex, DuplexCallbackArgs as DuplexStreamCallbackArgs, DuplexSettings as DuplexStreamSettings,
    Flags as StreamFlags, Flow, Info as StreamInfo, Input,
    InputCallbackArgs as InputStreamCallbackArgs, InputSettings as InputStreamSettings,
    NonBlocking, Output, OutputCallbackArgs as OutputStreamCallbackArgs,
    OutputSettings as OutputStreamSettings, Parameters as StreamParameters,
    Settings as StreamSettings, Stream,
};
pub use types::{
    DeviceIndex, DeviceInfo, Frames, HostApiIndex, HostApiInfo, HostApiTypeId, HostErrorInfo,
    SampleFormat, Time, FRAMES_PER_BUFFER_UNSPECIFIED,
};

use std::ptr;

#[macro_use]
mod enum_primitive;
pub mod error;
pub mod ext;
pub mod stream;
mod types;

/// A type-safe wrapper around the PortAudio API.
///
/// We use a type here instead of pure functions in order to ensure correct intialisation and
/// termination of the underlying PortAudio instance.
#[derive(Debug)]
pub struct PortAudio {
    /// The lifetime of the `PortAudio` API.
    ///
    /// The lifetime is shared between `PortAudio` and all its spawned `Stream`s.
    life: std::sync::Arc<Life>,
}

/// The lifetime of the `PortAudio` instance.
///
/// This type is shared between `PortAudio` and its child `Stream`s.
#[derive(Debug)]
pub struct Life {
    /// This is solely used for checking whether or not the PortAudio API has already been
    /// terminated manually (via the `PortAudio::terminate` method) when `Drop::drop` is called.
    is_terminated: std::sync::Mutex<bool>,
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
            let error = FromPrimitive::from_i32(ffi::Pa_Initialize()).unwrap();
            match error {
                Error::NoError => {
                    let life = std::sync::Arc::new(Life {
                        is_terminated: std::sync::Mutex::new(false),
                    });
                    Ok(PortAudio { life: life })
                }
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
    pub fn terminate(self) -> Result<(), Error> {
        *self.life.is_terminated.lock().unwrap() = true;
        terminate()
    }

    /// Retrieve the release number of the currently running PortAudio build.
    pub fn version(&self) -> i32 {
        version()
    }

    /// Retrieve a textual description of the current PortAudio build.
    ///
    /// Returns a `Utf8Error` if the C `*const char` can't be converted into a &'static str.
    pub fn version_text(&self) -> Result<&'static str, ::std::str::Utf8Error> {
        version_text()
    }

    /// Produces an iterator yielding the **DeviceIndex** for each device along with their
    /// respective **DeviceInfo**s.
    pub fn devices(&self) -> Result<Devices, Error> {
        Ok(Devices {
            total: self.device_count()?,
            next: 0,
            port_audio: self,
        })
    }

    /// Retrieve the number of available devices.
    ///
    /// The number of available devices may be zero.
    ///
    /// Returns an `Error` if PortAudio encounters some error.
    ///
    /// **NOTE:** The only error documented by PortAudio that may occur calling this method is
    /// caused by PortAudio not being initialised, however this should not be possible with our
    /// type-safe **PortAudio** API.
    pub fn device_count(&self) -> Result<u32, Error> {
        match unsafe { ffi::Pa_GetDeviceCount() } {
            n if n >= 0 => Ok(n as u32),
            // NOTE: The docs for this error (NO_DEVICE) specify that this simply indicates that
            // there are no devices available or that no available devices should be used. Thus, we
            // will simply translate this to a count of `0`.
            -1 => Ok(0),
            err => Err(::num::FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Retrieve the index of the default input device. The result can be used in the
    /// **InSettings** used to open an **In** **Stream**.
    ///
    /// Returns the default output device index for the default host API.
    ///
    /// Returns `Error` if no default input device is available or an error was encountered.
    ///
    /// **TODO:** Investigate exactly what errors may occur as the PA docs aren't clear on this.
    pub fn default_input_device(&self) -> Result<DeviceIndex, Error> {
        match unsafe { ffi::Pa_GetDefaultInputDevice() } {
            idx if idx >= 0 => Ok(DeviceIndex(idx as u32)),
            err => Err(::num::FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Retrieve the index of the default output device. The result can be used in the
    /// **OutSettings** used to open an **Out** **Stream**.
    ///
    /// Returns the default input device index for the default host API.
    ///
    /// Returns `Error` if no default input device is available or an error was encountered.
    ///
    /// **TODO:** Investigate exactly what errors may occur as the PA docs aren't clear on this.
    pub fn default_output_device(&self) -> Result<DeviceIndex, Error> {
        match unsafe { ffi::Pa_GetDefaultOutputDevice() } {
            idx if idx >= 0 => Ok(DeviceIndex(idx as u32)),
            err => Err(::num::FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Retrieve a **DeviceInfo** structure containing information about the specified device.
    ///
    /// Returns `Ok(DeviceInfo)` if successful.
    ///
    /// Returns `Err(Error::InvalidDevice)` if the device parameter is out of range.
    ///
    /// # Arguments
    ///
    /// - device - A valid device index in the range 0 to (port_audio.device_count()-1).
    pub fn device_info(&self, device: DeviceIndex) -> Result<DeviceInfo, Error> {
        let c_info = unsafe { ffi::Pa_GetDeviceInfo(device.into()) };
        if c_info.is_null() {
            Err(Error::InvalidDevice)
        } else {
            Ok(DeviceInfo::from_c_info(unsafe { *c_info }))
        }
    }

    /// Produces an iterator yielding the **HostApiIndex** of each available API along with their
    /// respective **HostApiInfo**s.
    pub fn host_apis(&self) -> HostApis {
        HostApis {
            total: self.host_api_count().unwrap_or(0),
            next: 0,
            port_audio: self,
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
        unsafe { result_from_host_api_index(ffi::Pa_GetHostApiCount()) }
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
        unsafe { result_from_host_api_index(ffi::Pa_GetDefaultHostApi()) }
    }

    /// Retrieve a pointer to a structure containing information about a specific host Api.
    ///
    /// # Arguments
    ///
    /// - host_api - A valid host API index ranging from 0 to (Pa_GetHostApiCount()-1)
    ///
    /// Return `Some(PaHostApiInfo)` describing a specific host API.
    ///
    /// Returns `None` if the `host_api` parameter is out of range or an error is encountered.
    pub fn host_api_info<'a>(&'a self, host_api: HostApiIndex) -> Option<HostApiInfo<'a>> {
        let c_host_info = unsafe { ffi::Pa_GetHostApiInfo(host_api as HostApiIndex) };
        if c_host_info.is_null() {
            None
        } else {
            HostApiInfo::from_c_info(unsafe { *c_host_info })
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
    pub fn host_api_type_id_to_host_api_index(
        &self,
        type_id: HostApiTypeId,
    ) -> Result<HostApiIndex, Error> {
        let id = FromPrimitive::from_i32(type_id as i32).unwrap();
        unsafe { result_from_host_api_index(ffi::Pa_HostApiTypeIdToHostApiIndex(id)) }
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
    pub fn api_device_index_to_device_index(
        &self,
        host_api: HostApiIndex,
        host_api_device_index: i32,
    ) -> Result<DeviceIndex, Error> {
        let result =
            unsafe { ffi::Pa_HostApiDeviceIndexToDeviceIndex(host_api, host_api_device_index) };
        match result {
            idx if idx >= 0 => Ok(DeviceIndex(idx as u32)),
            err => Err(::num::FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Determine whether it would be possible to open an input-only stream with the specified
    /// parameters.
    ///
    /// The `suggested_latency` field of the `StreamParameters` is ignored.
    ///
    /// Returns `Ok(())` if the format is supported, and an `Err(Error)` indicating why the format
    /// is not supported otherwise.
    pub fn is_input_format_supported<I>(
        &self,
        params: StreamParameters<I>,
        sample_rate: f64,
    ) -> Result<(), Error>
    where
        I: Sample,
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
    pub fn is_output_format_supported<O>(
        &self,
        params: StreamParameters<O>,
        sample_rate: f64,
    ) -> Result<(), Error>
    where
        O: Sample,
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
    pub fn is_duplex_format_supported<I, O>(
        &self,
        in_params: StreamParameters<I>,
        out_params: StreamParameters<O>,
        sample_rate: f64,
    ) -> Result<(), Error>
    where
        I: Sample,
        O: Sample,
    {
        is_format_supported(Some(in_params.into()), Some(out_params.into()), sample_rate)
    }

    /// Open a new blocking [**Stream**](./stream/struct.Stream.html) with the given settings.
    ///
    /// The **Stream** will be opened in **Blocking** "read/write" mode.
    ///
    /// In **Blocking** mode, the client can receive sample data using **Stream::read** (for
    /// **Input** and **Duplex** streams only) and write sample data using **Stream::write** (for
    /// **Output** and **Duplex** streams only). The number of samples that may be read or written
    /// without blocking is returned by **Stream::read_available** and **Stream::write_available**
    /// respectively.
    ///
    /// The returned **Stream** is inactive (stopped).
    pub fn open_blocking_stream<S>(
        &self,
        settings: S,
    ) -> Result<Stream<Blocking<<S::Flow as Flow>::Buffer>, S::Flow>, Error>
    where
        S: StreamSettings,
        S::Flow: Flow,
    {
        Stream::<Blocking<<S::Flow as Flow>::Buffer>, S::Flow>::open(self.life.clone(), settings)
    }

    /// Open a new non-blocking [**Stream**](./stream/struct.Stream.html) with the given settings.
    ///
    /// When a non-blocking stream is running, PortAudio calls the given `callback` periodically.
    /// The callback function is responsible for processing buffers of audio samples passed via the
    /// input and/or output parameters (depending on the **Stream**'s **Flow**).
    ///
    /// The `callback` runs at very high or real-time priority. It is required to consistently meet
    /// its time deadlines. Do **not** allocate memory, access the file system, call library
    /// functions or call other functions from the stream callback that may block or take an
    /// unpredictable amount of time to complete.
    ///
    /// In order for a stream to maintain glitch-free operation the `callback` must consume and
    /// return audio data faster than it is recorded and/or played. PortAudio anticipates that each
    /// callback invocation may execute for a duration approaching the duration of `frames` audio
    /// frames at the stream sample rate. It is reasonable to expect to be able to utilise 70% or
    /// more of the available CPU time in the PortAudio `callback`. However, due to buffer size
    /// adaption and other factors, not all host APIs are able to guarantee audio stability under
    /// heavy CPU load with arbitrary fixed callback buffer sizes. When high callback CPU
    /// utilisation is required the most robust behaviour can be achieved by using an unspecified
    /// `frames_per_buffer` in the given settings (i.e. setting it to `0`).
    ///
    /// The `callback` should return one of the variants of the
    /// [**StreamCallbackResult**](./stream/enum.CallbackResult.html) enum. To ensure that the
    /// callback continues to be called, it should return **Continue**. Either **Complete** or
    /// **Abort** can be returned to finish stream processing, after either of these values is
    /// returned the callback will not be called again. If **Abort** is returned the stream will
    /// finish as soon as possible. If **Complete** is returned, the stream will continue until all
    /// buffers generated by the callback have been played. This may be useful in applications such
    /// as soundfile players where a specific duration of output is required. However, it is not
    /// necessary to utilize this mechanism as **Stream::stop/abort/close** can also be used to
    /// stop the **Stream**. **Output** stream `callback`s must always fill the entire buffer
    /// irrespective of its return value.
    ///
    /// The returned **Stream** is inactive (stopped).
    pub fn open_non_blocking_stream<S, C>(
        &self,
        settings: S,
        callback: C,
    ) -> Result<Stream<NonBlocking, S::Flow>, Error>
    where
        S: StreamSettings,
        S::Flow: Flow,
        C: FnMut(<S::Flow as Flow>::CallbackArgs) -> ffi::PaStreamCallbackResult + 'static,
    {
        Stream::<NonBlocking, S::Flow>::open(self.life.clone(), settings, callback)
    }

    /// Produce the default **StreamParameters** for an **Input** **Stream**.
    ///
    /// The device used will be the default input device for the default Host API.
    ///
    /// The produced **Parameters** will assume interleaved buffered audio data.
    pub fn default_input_stream_params<I>(
        &self,
        channels: i32,
    ) -> Result<StreamParameters<I>, Error> {
        const INTERLEAVED: bool = true;
        let device = self.default_input_device()?;
        let latency = self.device_info(device)?.default_low_input_latency;
        Ok(StreamParameters::new(
            device,
            channels,
            INTERLEAVED,
            latency,
        ))
    }

    /// Produce the default **StreamParameters** for an **Output** **Stream**.
    ///
    /// The device used will be the default output device for the default Host API.
    ///
    /// The produced **Parameters** will assume interleaved buffered audio data.
    pub fn default_output_stream_params<O>(
        &self,
        channels: i32,
    ) -> Result<StreamParameters<O>, Error> {
        const INTERLEAVED: bool = true;
        let device = self.default_output_device()?;
        let latency = self.device_info(device)?.default_low_output_latency;
        Ok(StreamParameters::new(
            device,
            channels,
            INTERLEAVED,
            latency,
        ))
    }

    /// Produce the default **InputStreamSettings** with the given number of channels, sample_rate
    /// and frames per buffer.
    ///
    /// The device used will be the default input device for the default Host API.
    ///
    /// The produced settings will assume interleaved buffered audio data.
    pub fn default_input_stream_settings<I>(
        &self,
        channels: i32,
        sample_rate: f64,
        frames_per_buffer: u32,
    ) -> Result<InputStreamSettings<I>, Error> {
        let params = self.default_input_stream_params(channels)?;
        Ok(InputStreamSettings::new(
            params,
            sample_rate,
            frames_per_buffer,
        ))
    }

    /// Produce the default **OutputStreamSettings** with the given number of channels, sample_rate
    /// and frames per buffer.
    ///
    /// The device used will be the default output device for the default Host API.
    ///
    /// The produced settings will assume interleaved buffered audio data.
    pub fn default_output_stream_settings<O>(
        &self,
        channels: i32,
        sample_rate: f64,
        frames_per_buffer: u32,
    ) -> Result<OutputStreamSettings<O>, Error> {
        let params = self.default_output_stream_params(channels)?;
        Ok(OutputStreamSettings::new(
            params,
            sample_rate,
            frames_per_buffer,
        ))
    }

    /// Produce the default **DuplexStreamSettings** with the given number of channels, sample_rate
    /// and frames per buffer.
    ///
    /// The devices used will be the default input and output devices for the default Host API.
    ///
    /// The produced settings will assume interleaved buffered audio data.
    pub fn default_duplex_stream_settings<I, O>(
        &self,
        in_channels: i32,
        out_channels: i32,
        sample_rate: f64,
        frames_per_buffer: u32,
    ) -> Result<DuplexStreamSettings<I, O>, Error> {
        let in_params = self.default_input_stream_params(in_channels)?;
        let out_params = self.default_output_stream_params(out_channels)?;
        Ok(DuplexStreamSettings::new(
            in_params,
            out_params,
            sample_rate,
            frames_per_buffer,
        ))
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
        unsafe { ffi::Pa_Sleep(m_sec as raw::c_long) }
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
    pub fn last_host_error_info<'a>(&'a self) -> HostErrorInfo<'a> {
        let c_error = unsafe { ffi::Pa_GetLastHostErrorInfo() };
        HostErrorInfo::from_c_error_info(unsafe { *c_error })
    }
}

impl Drop for Life {
    fn drop(&mut self) {
        if !*self.is_terminated.lock().unwrap() {
            terminate().ok();
        }
    }
}

/// Retrieve the release number of the currently running PortAudio build.
pub fn version() -> i32 {
    unsafe { ffi::Pa_GetVersion() }
}

/// Retrieve a textual description of the current PortAudio build.
pub fn version_text() -> Result<&'static str, ::std::str::Utf8Error> {
    unsafe { ffi::c_str_to_str(ffi::Pa_GetVersionText()) }
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
        let error = FromPrimitive::from_i32(ffi::Pa_Terminate()).unwrap();
        match error {
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
fn is_format_supported(
    maybe_input_parameters: Option<ffi::PaStreamParameters>,
    maybe_output_parameters: Option<ffi::PaStreamParameters>,
    sample_rate: f64,
) -> Result<(), Error> {
    let c_input = maybe_input_parameters
        .as_ref()
        .map(|input| input as *const _);
    let c_output = maybe_output_parameters
        .as_ref()
        .map(|output| output as *const _);
    if c_input.is_none() && c_output.is_none() {
        Err(Error::InvalidDevice)
    } else {
        unsafe {
            let error_code = ffi::Pa_IsFormatSupported(
                c_input.unwrap_or(ptr::null()),
                c_output.unwrap_or(ptr::null()),
                sample_rate as raw::c_double,
            );
            let error = FromPrimitive::from_i32(error_code).unwrap();
            match error {
                Error::NoError => Ok(()),
                err => Err(err),
            }
        }
    }
}

/// An iterator yielding the **DeviceIndex** for each available device along with their respective
/// **DeviceInfo**s.
pub struct Devices<'a> {
    total: u32,
    next: u32,
    port_audio: &'a PortAudio,
}

/// An iterator yielding the **HostApiIndex** for each available API along with their respective
/// **HostApiInfo**s.
#[derive(Clone, Debug)]
pub struct HostApis<'a> {
    total: HostApiIndex,
    next: HostApiIndex,
    port_audio: &'a PortAudio,
}

impl<'a> Iterator for Devices<'a> {
    type Item = Result<(DeviceIndex, DeviceInfo<'a>), Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.total {
            let idx = DeviceIndex(self.next);
            self.next += 1;
            return Some(self.port_audio.device_info(idx).map(|info| (idx, info)));
        }
        None
    }
}

impl<'a> Iterator for HostApis<'a> {
    type Item = (HostApiIndex, HostApiInfo<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.total {
            let idx = self.next;
            self.next += 1;
            if let Some(info) = self.port_audio.host_api_info(idx) {
                return Some((idx, info));
            }
        }
        None
    }
}

fn result_from_host_api_index(idx: ffi::PaHostApiIndex) -> Result<HostApiIndex, Error> {
    match idx {
        idx if idx >= 0 => Ok(idx),
        err => Err(::num::FromPrimitive::from_i32(err).unwrap()),
    }
}

/// Retrieve the size of a given sample format in bytes.
///
/// Return the size in bytes of a single sample in the specified format,
/// or SampleFormatNotSupported if the format is not supported.
pub fn get_sample_size(format: SampleFormat) -> Result<u8, Error> {
    let result = unsafe { ffi::Pa_GetSampleSize(format as ffi::PaSampleFormat) };
    if result < 0 {
        Err(::num::FromPrimitive::from_i32(result).unwrap())
    } else {
        Ok(result as u8)
    }
}

mod private {
    use super::types::SampleFormat;
    use num::{FromPrimitive, ToPrimitive};
    use std::ops::{Add, Div, Mul, Sub};

    /// internal private trait for Sample format management
    pub trait SamplePrivate:
        ::std::default::Default
        + Copy
        + Clone
        + ::std::fmt::Debug
        + ToPrimitive
        + FromPrimitive
        + Add
        + Sub
        + Mul
        + Div
    {
        /// return the size of a sample format
        fn size<S: SamplePrivate>() -> usize {
            ::std::mem::size_of::<S>()
        }
        /// get the sample format
        fn to_sample_format() -> SampleFormat;
    }
}

impl private::SamplePrivate for f32 {
    fn to_sample_format() -> SampleFormat {
        SampleFormat::F32
    }
}

impl private::SamplePrivate for i32 {
    fn to_sample_format() -> SampleFormat {
        SampleFormat::I32
    }
}

impl private::SamplePrivate for i16 {
    fn to_sample_format() -> SampleFormat {
        SampleFormat::I16
    }
}

impl private::SamplePrivate for i8 {
    fn to_sample_format() -> SampleFormat {
        SampleFormat::I8
    }
}

impl private::SamplePrivate for u8 {
    fn to_sample_format() -> SampleFormat {
        SampleFormat::U8
    }
}

/// public trait to constraint pa::Stream for specific types
pub trait Sample: private::SamplePrivate {
    /// Retrieve the SampleFormat variant associated with the type.
    fn sample_format() -> SampleFormat {
        Self::to_sample_format()
    }
}

impl Sample for f32 {}
impl Sample for i32 {}
impl Sample for i16 {}
impl Sample for i8 {}
impl Sample for u8 {}
