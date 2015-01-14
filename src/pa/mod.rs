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

use std::{ptr, mem};
use std::mem::{transmute};
use std::num::{FromPrimitive};
use std::vec::{Vec};
use libc::{c_double, c_void, malloc};
use libc::types::os::arch::c95::size_t;

use ffi;
pub use self::error::Error;
pub use self::types::{
    HostApiInfo,
    DeviceInfo,
    SampleFormat,
    StreamFlags,
    StreamParameters,
    HostErrorInfo,
    Time,
    Frames,
    StreamInfo,
    CallbackFunction,
    DeviceIndex,
    HostApiIndex,
    StreamCallbackFlags,
    StreamCallbackTimeInfo,
    StreamCallbackResult,
    PA_NO_DEVICE,
    HostApiTypeId,
    PA_USE_HOST_API_SPECIFIC_DEVICE_SPECIFICATION,
};

pub mod error;
mod types;
pub mod device;
pub mod host;

/// Retrieve the release number of the currently running PortAudio build.
pub fn get_version() -> i32 {
    unsafe {
        ffi::Pa_GetVersion()
    }
}

/// Retrieve a textual description of the current PortAudio build.
pub fn get_version_text() -> String {
    unsafe {
        ffi::c_str_to_string(&ffi::Pa_GetVersionText())
    }
}

/// Translate the supplied PortAudio error code into a human readable message.
///
/// # Arguments
/// * error_code - The error code
///
/// Return the error as a string.
pub fn get_error_text(error_code: Error) -> String {
    unsafe {
        ffi::c_str_to_string(&ffi::Pa_GetErrorText(error_code))
    }
}

/// Library initialization function - call this before using PortAudio.
/// This function initializes internal data structures and prepares underlying
/// host APIs for use. With the exception of get_version(), get_version_text(),
/// and get_error_text(), this function MUST be called before using any other
/// PortAudio API functions.
///
/// Note that if initialize() returns an error code, Pa_Terminate() should NOT be
/// called.
///
/// Return NoError if successful, otherwise an error code indicating the cause
/// of failure.
pub fn initialize() -> Result<(), Error> {
    unsafe {
        match ffi::Pa_Initialize() {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }
}

/// Library termination function - call this when finished using PortAudio.
/// This function deallocates all resources allocated by PortAudio since it was
/// initialized by a call to initialize(). In cases where initialise() has been
/// called multiple times, each call must be matched with a corresponding call to
/// terminate(). The final matching call to terminate() will automatically close
/// any PortAudio streams that are still open.
///
/// terminate() MUST be called before exiting a program which uses PortAudio.
/// Failure to do so may result in serious resource leaks, such as audio devices
/// not being available until the next reboot.
///
/// Return NoError if successful, otherwise an error code indicating the cause
/// of failure.
pub fn terminate() -> Result<(), Error> {
    unsafe {
        match ffi::Pa_Terminate() {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }
}

/// Return information about the last host error encountered.
/// The error information returned by get_last_host_error_info() will never be
/// modified asynchronously by errors occurring in other PortAudio owned threads
/// (such as the thread that manages the stream callback.)
///
/// This function is provided as a last resort, primarily to enhance debugging
/// by providing clients with access to all available error information.
///
/// Return a pointer to an immuspacespacespacele structure constraining
/// information about the host error. The values in this structure will only be
/// valid if a PortAudio function has previously returned the
/// UnanticipatedHostError error code.
pub fn get_last_host_error_info() -> HostErrorInfo {
    let c_error = unsafe { ffi::Pa_GetLastHostErrorInfo() };
    HostErrorInfo::wrap(c_error)
}

/// Determine whether it would be possible to open a stream with the specified
/// parameters.
///
/// # Arguments
/// * input_parameters - A structure that describes the input parameters used to
/// open a stream.
/// The suggestedLatency field is ignored. See StreamParameters for a
/// description of these parameters. inputParameters must be None for output-only
/// streams.
/// * output_parameters - A structure that describes the output parameters used to
/// open a stream. The suggestedLatency field is ignored. See StreamParameters
/// for a description of these parameters. outputParameters must be None for
/// input-only streams.
/// * sample_rate - The required sampleRate. For full-duplex streams it is the
/// sample rate for both input and output.
///
/// Return 0 if the format is supported, and an error code indicating why the
/// format is not supported otherwise. The constant PaFormatIsSupported is
/// provided to compare with the return value for success.
pub fn is_format_supported(input_parameters: &StreamParameters,
                           output_parameters: &StreamParameters,
                           sample_rate : f64) -> Result<(), Error> {
    let c_input = input_parameters.unwrap();
    let c_output = output_parameters.unwrap();
    match unsafe {
        ffi::Pa_IsFormatSupported(&c_input, &c_output, sample_rate as c_double)
    } {
        Error::NoError => Ok(()),
        err => Err(err),
    }
}

/// Retrieve the size of a given sample format in bytes.
///
/// Return the size in bytes of a single sample in the specified format,
/// or SampleFormatNotSupported if the format is not supported.
pub fn get_sample_size(format: SampleFormat) -> Result<(), Error> {
    match unsafe {
        ffi::Pa_GetSampleSize(format as u64)
    } {
        Error::NoError => Ok(()),
        err => Err(err),
    }
}

/// Put the caller to sleep for at least 'msec' milliseconds.
/// This function is provided only as a convenience for authors of portable code
/// (such as the tests and examples in the PortAudio distribution.)
///
/// The function may sleep longer than requested so don't rely on this for
/// accurate musical timing.
pub fn sleep(m_sec : i32) -> () {
    unsafe {
        ffi::Pa_Sleep(m_sec)
    }
}

mod private {

    use std::num::{FromPrimitive, ToPrimitive};
    use std::ops::{Add, Sub, Mul, Div};
    use super::types::SampleFormat;

    /// internal private trait for Sample format management
    pub trait SamplePrivate: ::std::default::Default + Copy + Clone + ::std::fmt::Show
                             + ToPrimitive + FromPrimitive + Add + Sub + Mul + Div {
        /// return the size of a sample format
        fn size<S: SamplePrivate>() -> usize {
            ::std::mem::size_of::<S>()
        }
        /// get the sample format
        fn to_sample_format(&self) -> SampleFormat;
    }

}

impl private::SamplePrivate for f32 {
    fn to_sample_format(&self) -> SampleFormat { SampleFormat::Float32 }
}

impl private::SamplePrivate for i32 {
    fn to_sample_format(&self) -> SampleFormat { SampleFormat::Int32 }
}

impl private::SamplePrivate for u8 {
    fn to_sample_format(&self) -> SampleFormat { SampleFormat::UInt8 }
}

impl private::SamplePrivate for i8 {
    fn to_sample_format(&self) -> SampleFormat { SampleFormat::Int8 }
}

/// public trait to constraint pa::Stream for specific types
pub trait Sample: private::SamplePrivate {
    /// Retrieve the SampleFormat variant associated with the type.
    fn sample_format_for<S: Sample>() -> SampleFormat {
        let s: S = ::std::default::Default::default();
        s.sample_format()
    }
    /// Retrieve the SampleFormat variant associated with the type.
    fn sample_format(&self) -> SampleFormat { self.to_sample_format() }
}

impl Sample for f32 {}
impl Sample for i32 {}
impl Sample for i8 {}
impl Sample for u8 {}

/// Representation of an audio stream, where the format of the stream is defined
/// by the S parameter.
pub struct Stream<I: Sample, O: Sample> {
    c_pa_stream : *mut ffi::C_PaStream,
    c_input : Option<ffi::C_PaStreamParameters>,
    c_output : Option<ffi::C_PaStreamParameters>,
    unsafe_buffer : *mut c_void,
    callback_function : Option<CallbackFunction>,
    num_input_channels : i32
}

impl<I: Sample, O: Sample> Stream<I, O> {
    /// Constructor for Stream.
    ///
    /// Return a new Stream.
    pub fn new() -> Stream<I, O> {
        Stream {
            c_pa_stream : ptr::null_mut(),
            c_input : None,
            c_output : None,
            unsafe_buffer : ptr::null_mut(),
            callback_function : None,
            num_input_channels : 0
        }
    }

    /// Opens a stream for either input, output or both.
    ///
    /// # Arguments
    /// * input_parameters - A structure that describes the input parameters used
    /// by the opened stream.
    /// * output_parameters - A structure that describes the output parameters
    /// used by the opened stream.
    /// * sample_rate - The desired sample_rate. For full-duplex streams it is the
    /// sample rate for both input and output
    /// * frames_per_buffer - The number of frames passed to the stream callback
    /// function.
    /// * stream_flags -Flags which modify the behavior of the streaming process.
    /// This parameter may contain a combination of flags ORed together. Some
    /// flags may only be relevant to certain buffer formats.
    ///
    /// Upon success returns NoError and the stream is inactive (stopped).
    /// If fails, a non-zero error code is returned.
    pub fn open(&mut self,
                input_parameters: Option<&StreamParameters>,
                output_parameters: Option<&StreamParameters>,
                sample_rate: f64,
                frames_per_buffer: u32,
                stream_flags: StreamFlags) -> Result<(), Error> {
        if input_parameters.is_some() {
            self.c_input = Some(input_parameters.unwrap().unwrap());
            self.num_input_channels = input_parameters.unwrap().channel_count;
            self.unsafe_buffer = unsafe {
                malloc(mem::size_of::<I>() as size_t *
                       frames_per_buffer as size_t *
                       input_parameters.unwrap().channel_count as size_t) as *mut c_void};
        }
        if output_parameters.is_some() {
            self.c_output = Some(output_parameters.unwrap().unwrap());
        }

        unsafe {
            if self.c_input.is_some() &&
               self.c_output.is_some() {
                let err = ffi::Pa_OpenStream(&mut self.c_pa_stream,
                                             &(self.c_input.unwrap()),
                                             &(self.c_output.unwrap()),
                                             sample_rate as c_double,
                                             frames_per_buffer,
                                             stream_flags as u64,
                                             None,
                                             ptr::null_mut());
                match err {
                    Error::NoError => Ok(()),
                    _ => Err(err),
                }
            }
            else if self.c_input.is_some() {
                let err = ffi::Pa_OpenStream(&mut self.c_pa_stream,
                                             &(self.c_input.unwrap()),
                                             ptr::null(),
                                             sample_rate as c_double,
                                             frames_per_buffer,
                                             stream_flags as u64,
                                             None,
                                             ptr::null_mut());
                match err {
                    Error::NoError => Ok(()),
                    _ => Err(err),
                }
            }
            else if self.c_output.is_some() {
                let err = ffi::Pa_OpenStream(&mut self.c_pa_stream,
                                             ptr::null(),
                                             &(self.c_output.unwrap()),
                                             sample_rate as c_double,
                                             frames_per_buffer,
                                             stream_flags as u64,
                                             None,
                                             ptr::null_mut());
                match err {
                    Error::NoError => Ok(()),
                    _ => Err(err),
                }
            }
            else {
                Err(Error::BadStreamPtr)
            }
        }
    }

    /// A simplified version of open() that opens the default input and/or output
    /// devices.
    ///
    /// # Arguments
    /// * sample_rate - The desired sample_rate. For full-duplex streams it is
    /// the sample rate for both input and output
    /// * frames_per_buffer - The number of frames passed to the stream callback
    /// function
    /// * num_input_channels - The number of channels of sound that will be
    /// supplied to the stream callback or returned by Pa_ReadStream. It can range
    /// from 1 to the value of maxInputChannels in the DeviceInfo record for the
    /// default input device. If 0 the stream is opened as an output-only stream.
    /// * num_output_channels - The number of channels of sound to be delivered to
    /// the stream callback or passed to Pa_WriteStream. It can range from 1 to
    /// the value of maxOutputChannels in the DeviceInfo record for the default
    /// output device. If 0 the stream is opened as an output-only stream.
    /// * sample_format - The sample_format for the input and output buffers.
    ///
    /// Upon success returns NoError and the stream is inactive (stopped).
    /// If fails, a non-zero error code is returned.
    pub fn open_default(&mut self,
                        sample_rate: f64,
                        frames_per_buffer: u32,
                        num_input_channels: i32,
                        num_output_channels: i32,
                        sample_format: SampleFormat) -> Result<(), Error> {

        if num_input_channels > 0 {
            self.c_input = None;
            self.num_input_channels = num_input_channels;
            self.unsafe_buffer = unsafe {
                malloc(mem::size_of::<I>() as size_t *
                       frames_per_buffer as size_t *
                       num_input_channels as size_t) as *mut c_void };
        }
        match unsafe {
           ffi::Pa_OpenDefaultStream(&mut self.c_pa_stream,
                                     num_input_channels,
                                     num_output_channels,
                                     sample_format as u64,
                                     sample_rate as c_double,
                                     frames_per_buffer,
                                     None,
                                     ptr::null_mut())
        } {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Closes an audio stream. If the audio stream is active it discards any
    /// pending buffers as if abort_tream() had been called.
    pub fn close(&mut self) -> Result<(), Error> {
        match unsafe {
            ffi::Pa_CloseStream(self.c_pa_stream)
        } {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Commences audio processing.
    pub fn start(&mut self) -> Result<(), Error> {
        match unsafe {
            ffi::Pa_StartStream(self.c_pa_stream)
        } {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Terminates audio processing. It waits until all pending audio buffers
    /// have been played before it returns.
    pub fn stop(&mut self) -> Result<(), Error> {
        match unsafe {
            ffi::Pa_StopStream(self.c_pa_stream)
        } {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Terminates audio processing immediately without waiting for pending
    /// buffers to complete.
    pub fn abort(&mut self) -> Result<(), Error> {
        match unsafe {
            ffi::Pa_AbortStream(self.c_pa_stream)
        } {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Determine whether the stream is stopped.
    /// A stream is considered to be stopped prior to a successful call to
    /// start_stream and after a successful call to stop_stream or abort_stream.
    /// If a stream callback returns a value other than Continue the stream is
    /// NOT considered to be stopped.
    ///
    /// Return one (1) when the stream is stopped, zero (0) when the stream is
    /// running or, a ErrorCode (which are always negative) if PortAudio is not
    /// initialized or an error is encountered.
    pub fn is_stopped(&self) -> Result<(), Error> {
        match unsafe {
            ffi::Pa_IsStreamStopped(self.c_pa_stream)
        } {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Determine whether the stream is active. A stream is active after a
    /// successful call to start_stream(), until it becomes inactive either as a
    /// result of a call to stop_stream() or abort_stream(), or as a result of a
    /// return value other than paContinue from the stream callback. In the latter
    /// case, the stream is considered inactive after the last buffer has finished
    /// playing.
    ///
    /// Return Ok(true) when the stream is active (ie playing or recording audio),
    /// Ok(false) when not playing or, a Err(Error) if PortAudio is not
    /// initialized or an error is encountered.
    pub fn is_active(&self) -> Result<bool, Error> {
        let err = unsafe { ffi::Pa_IsStreamActive(self.c_pa_stream) };
        match err {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(unsafe { transmute::<i32, Error>(err) })
        }
    }

    /// Returns the current time in seconds for a stream according to the same
    /// clock used to generate callback StreamCallbackTimeInfo timestamps.
    /// The time values are monotonically increasing and have unspecified origin.
    ///
    /// get_stream_time returns valid time values for the entire life of the
    /// stream, from when the stream is opened until it is closed.
    /// Starting and stopping the stream does not affect the passage of time
    /// returned by Pa_GetStreamTime.
    ///
    /// Return the stream's current time in seconds, or 0 if an error occurred.
    pub fn get_stream_time(&self) -> Time {
        unsafe {
            ffi::Pa_GetStreamTime(self.c_pa_stream)
        }
    }

    /// Retrieve CPU usage information for the specified stream.
    ///
    /// The "CPU Load" is a fraction of total CPU time consumed by a callback
    /// stream's audio processing routines including, but not limited to the
    /// client supplied stream callback. This function does not work with blocking
    /// read/write streams.
    pub fn get_stream_cpu_load(&self) -> f64 {
        unsafe {
            ffi::Pa_GetStreamCpuLoad(self.c_pa_stream)
        }
    }

    /// Retrieve the number of frames that can be read from the stream without
    /// waiting.
    ///
    /// Returns a non-negative value representing the maximum number of frames
    /// that can be read from the stream without blocking or busy waiting or,
    /// a ErrorCode (which are always negative) if PortAudio is not initialized
    /// or an error is encountered.
    pub fn get_stream_read_available(&self) -> Result<Option<Frames>, Error> {
        match unsafe { ffi::Pa_GetStreamReadAvailable(self.c_pa_stream) } {
            0          => Ok(None),
            n if n > 0 => Ok(Some(n)),
            n          => Err(FromPrimitive::from_i64(n).expect("Undefined error code.")),
        }
    }

    /// Retrieve the number of frames that can be written to the stream without
    /// waiting.
    ///
    /// Return a non-negative value representing the maximum number of frames that
    /// can be written to the stream without blocking or busy waiting or,
    /// a ErrorCode (which are always negative) if PortAudio is not initialized
    /// or an error is encountered.
    pub fn get_stream_write_available(&self) -> Result<Option<Frames>, Error> {
        match unsafe { ffi::Pa_GetStreamWriteAvailable(self.c_pa_stream) } {
            0          => Ok(None),
            n if n > 0 => Ok(Some(n)),
            n          => Err(FromPrimitive::from_i64(n).expect("Undefined error code.")),
        }
    }

    #[doc(hidden)]
    // Temporary OSX Fixe : Return always PaInputOverflowed
    #[cfg(target_os="macos")]
    pub fn read(&self, frames_per_buffer: u32) -> Result<Vec<I>, Error> {
        unsafe {
            ffi::Pa_ReadStream(self.c_pa_stream,
                               self.unsafe_buffer,
                               frames_per_buffer)
        };
        Ok(unsafe {
            Vec::from_raw_buf(self.unsafe_buffer as *const I,
                              (frames_per_buffer as usize) * (self.num_input_channels as usize)) })
    }

    /// Read samples from an input stream.
    /// The function doesn't return until the entire buffer has been filled - this
    /// may involve waiting for the operating system to supply the data.
    ///
    /// # Arguments
    /// * frames_per_buffer - The number of frames in the buffer.
    ///
    /// Return Ok(~[S]), a buffer containing the sample of the format S.
    /// If fail return a Error code.
    #[cfg(any(target_os="win32", target_os="linux"))]
    pub fn read(&self, frames_per_buffer: u32) -> Result<Vec<I>, Error> {
        let err = unsafe {
            ffi::Pa_ReadStream(self.c_pa_stream, self.unsafe_buffer, frames_per_buffer)
        };
        match err {
            Error::NoError  => Ok(unsafe {
                Vec::from_raw_buf(self.unsafe_buffer as *const I,
                        (frames_per_buffer * self.num_input_channels as u32) as uint) }),
            _               => Err(err)
        }
    }

    /// Write samples to an output stream.
    /// This function doesn't return until the entire buffer has been consumed
    /// - this may involve waiting for the operating system to consume the data.
    ///
    /// # Arguments
    /// * output_buffer - The buffer contains samples in the format specified by S.
    /// * frames_per_buffer - The number of frames in the buffer.
    ///
    /// Return NoError on success, or a Error code if fail.
    pub fn write<T> (&self, output_buffer: T, frames_per_buffer : u32) -> Result<(), Error>
    where T: AsSlice<O> {
        match unsafe {
            ffi::Pa_WriteStream(self.c_pa_stream,
                                output_buffer.as_slice().as_ptr() as *mut c_void,
                                frames_per_buffer)
        } {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Retrieve a StreamInfo structure containing information about the
    /// specified stream.
    pub fn get_stream_info(&self) -> StreamInfo {
        unsafe {
            *ffi::Pa_GetStreamInfo(self.c_pa_stream)
        }
    }

    #[doc(hidden)]
    pub fn get_c_pa_stream(&self) -> *mut ffi::C_PaStream {
        self.c_pa_stream
    }
}
