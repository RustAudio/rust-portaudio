/*!
* The porspacespacespacele PortAudio API.
*/


use std::str;
use std::libc::{c_double, c_void};
use std::ptr;
use std::libc::{malloc};
use std::vec;

use types::*;
use ffi;
use user_traits::*;

/// Retrieve the release number of the currently running PortAudio build, eg 1900.
#[fixed_stack_segment] #[inline(never)]
pub fn get_version() -> i32 {
    unsafe {
        ffi::Pa_GetVersion()
    }
}

/// Retrieve a textual description of the current PortAudio build, eg "PortAudio V19-devel 13 October 2002".
#[fixed_stack_segment] #[inline(never)]
pub fn get_version_text() -> ~str {
    unsafe { 
        str::raw::from_c_str(ffi::Pa_GetVersionText()) 
    }
}

/**
* Translate the supplied PortAudio error code into a human readable message.
*
* # Arguments 
* * error_code - The error code
*
* Return the error as a string.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_error_text(error_code : PaError) -> ~str {
    unsafe { 
        str::raw::from_c_str(ffi::Pa_GetErrorText(error_code))
    }
}

/**
* Library initialization function - call this before using PortAudio. 
* This function initializes internal data structures and prepares underlying host APIs for use. 
* With the exception of get_version(), get_version_text(), and get_error_text(), 
* this function MUST be called before using any other PortAudio API functions.
*
* Note that if initialize() returns an error code, Pa_Terminate() should NOT be called.
*
* Return PaNoError if successful, otherwise an error code indicating the cause of failure.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn initialize() -> PaError {
    unsafe {
        ffi::Pa_Initialize()
    }
}

/**
* Library termination function - call this when finished using PortAudio. 
* This function deallocates all resources allocated by PortAudio since it was initialized by a call to initialize(). 
* In cases where initialise() has been called multiple times, each call must be matched with a corresponding call to terminate(). 
* The final matching call to terminate() will automatically close any PortAudio streams that are still open.
*
* terminate() MUST be called before exiting a program which uses PortAudio. 
* Failure to do so may result in serious resource leaks, such as audio devices not being available until the next reboot.
*
* Return PaNoError if successful, otherwise an error code indicating the cause of failure.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn terminate() -> PaError {
    unsafe {
        ffi::Pa_Terminate()
    }
}

/**
* Retrieve the number of available host APIs. 
* Even if a host API is available it may have no devices available.
*
* Return a non-negative value indicating the number of available host APIs or,
* a PaErrorCode (which are always negative) if PortAudio is not initialized or an error is encountered.
*/ 
#[fixed_stack_segment] #[inline(never)]
pub fn get_host_api_count() -> PaHostApiIndex {
    unsafe {
        ffi::Pa_GetHostApiCount()
    }
}

/**
* Retrieve the index of the default host API. 
* The default host API will be the lowest common denominator host API 
* on the current platform and is unlikely to provide the best performance.
*
* Return a non-negative value ranging from 0 to (get_host_api_count()-1) 
* indicating the default host API index or, a PaErrorCode (which are always negative) 
* if PortAudio is not initialized or an error is encountered.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_default_host_api() -> PaHostApiIndex {
    unsafe {
        ffi::Pa_GetDefaultHostApi()
    }
}

/**
* Retrieve a pointer to a structure containing information about a specific host Api.
*
* # Arguments
* * host_api - A valid host API index ranging from 0 to (Pa_GetHostApiCount()-1)
*
* Return Some(PaHostApiInfo) describing a specific host API. If the hostApi parameter is out of range or an error is encountered, the function returns None.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_host_api_info(host_api : PaHostApiIndex) -> Option<PaHostApiInfo> {
    let c_host_info = unsafe { ffi::Pa_GetHostApiInfo(host_api) };
    if c_host_info.is_null() {
        None
    }
    else {
        Some(PaHostApiInfo::wrap(c_host_info))
    }
}

/**
* Convert a static host API unique identifier, into a runtime host API index.
*
* # Arguments
* * typde_id - A unique host API identifier belonging to the PaHostApiTypeId enumeration.
* 
* Return a valid PaHostApiIndex ranging from 0 to (get_host_api_count()-1) or, 
* a PaErrorCode (which are always negative) if PortAudio is not initialized or an error is encountered.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn host_api_type_id_to_host_api_index(type_id : PaHostApiTypeId) -> PaHostApiIndex {
    unsafe {
        ffi::Pa_HostApiTypeIdToHostApiIndex(type_id)
    }
}

/**
* Convert a host-API-specific device index to standard PortAudio device index. 
* This function may be used in conjunction with the deviceCount field of PaHostApiInfo 
* to enumerate all devices for the specified host API.
*
* # Arguments
* * host_api - A valid host API index ranging from 0 to (get_host_api_count()-1)
* * host_api_device_index - A valid per-host device index in the range 0 to (get_host_api_info(host_api).device_count-1)
*
* Return a non-negative PaDeviceIndex ranging from 0 to (get_device_count()-1) or, 
* a PaErrorCode (which are always negative) if PortAudio is not initialized or an error is encountered.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn host_api_devide_index_to_device_index(host_api : PaHostApiIndex,
                                                                   host_api_device_index : int)
                                                                   -> PaDeviceIndex {
    unsafe {
        ffi::Pa_HostApiDeviceIndexToDeviceIndex(host_api, host_api_device_index as i32)
    }
}

/**
* Return information about the last host error encountered. 
* The error information returned by get_last_host_error_info() will never be modified asynchronously
* by errors occurring in other PortAudio owned threads (such as the thread that manages the stream callback.)
*
* This function is provided as a last resort, primarily to enhance debugging 
* by providing clients with access to all available error information.
*
* Return a pointer to an immuspacespacespacele structure constraining information about the host error. 
* The values in this structure will only be valid if a PortAudio function has previously returned the PaUnanticipatedHostError error code.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_last_host_error_info() -> PaHostErrorInfo {
    let c_error = unsafe { ffi::Pa_GetLastHostErrorInfo() };
    PaHostErrorInfo::wrap(c_error)
}

/**
* Retrieve the number of available devices. The number of available devices may be zero.
*
* Return A non-negative value indicating the number of available devices or, 
* a PaErrorCode (which are always negative) if PortAudio is not initialized or an error is encountered.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_device_count() -> PaDeviceIndex {
    unsafe {
        ffi::Pa_GetDeviceCount()
    }
}

/**
* Retrieve the index of the default input device. 
* The result can be used in the inputDevice parameter to open_stream().
*
* Return the default input device index for the default host API, 
* or PaNoDevice if no default input device is available or an error was encountered
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_default_input_device() -> PaDeviceIndex {
    unsafe {
        ffi::Pa_GetDefaultInputDevice()
    }
}

/**
* Retrieve the index of the default output device. The result can be
* used in the outputDevice parameter to open_stream().
*
* Return the default output device index for the default host API, 
* or PaNoDevice if no default output device is available or an error was encountered.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_default_output_device() -> PaDeviceIndex {
    unsafe {
        ffi::Pa_GetDefaultOutputDevice()
    }
}

/**
* Retrieve a pointer to a PaDeviceInfo structure containing information about the specified device.
*
* # Arguments
* * device - A valid device index in the range 0 to (Pa_GetDeviceCount()-1)
*
* Return Some(PaDeviceInfo) or, If the device parameter is out of range the function returns None.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_device_info(device : PaDeviceIndex) -> Option<PaDeviceInfo> {
    let c_info = unsafe { ffi::Pa_GetDeviceInfo(device) };
    if c_info.is_null() {
        None
    }
    else {
        Some(PaDeviceInfo::wrap(c_info))
    }
}

/**
* Determine whether it would be possible to open a stream with the specified parameters.
*
* # Arguments
* * input_parameters - A structure that describes the input parameters used to open a stream. 
* The suggestedLatency field is ignored. See PaStreamParameters for a description of these parameters. inputParameters must be None for output-only streams.
* * output_parameters - A structure that describes the output parameters used to open a stream. The suggestedLatency field is ignored. 
* See PaStreamParameters for a description of these parameters. outputParameters must be None for input-only streams.
* * sample_rate - The required sampleRate. For full-duplex streams it is the sample rate for both input and output.
*
* Return 0 if the format is supported, and an error code indicating why the format is not supported otherwise.
* The constant PaFormatIsSupported is provided to compare with the return value for success.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn is_format_supported(input_parameters : &PaStreamParameters,
                                       output_parameters : &PaStreamParameters,
                                       sample_rate : f64)
                                       -> PaError {
    let c_input = input_parameters.unwrap();
    let c_output = output_parameters.unwrap();
    unsafe {
        ffi::Pa_IsFormatSupported(&c_input, &c_output, sample_rate as c_double)
    }
}

/**
* Retrieve the size of a given sample format in bytes.
*
* Return the size in bytes of a single sample in the specified format,
* or PaSampleFormatNotSupported if the format is not supported.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn get_sample_size(format : PaSampleFormat) -> PaError {
    unsafe {
        ffi::Pa_GetSampleSize(format)
    }
}

/**
* Put the caller to sleep for at least 'msec' milliseconds. 
* This function is provided only as a convenience for authors of porspacespacespacele code (such as the tests and examples in the PortAudio distribution.)
*
* The function may sleep longer than requested so don't rely on this for accurate musical timing.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn sleep(m_sec : int) -> () {
    unsafe {
        ffi::Pa_Sleep(m_sec as i32)
    }
}

pub struct WrapObj {
    pa_callback : @PortaudioCallback
}


pub struct PaStream {
    priv c_pa_stream : *C_PaStream,
    priv sample_format : PaSampleFormat,
    priv c_input : Option<C_PaStreamParameters>,
    priv c_output : Option<C_PaStreamParameters>,
    priv unsafe_buffer : *c_void
}

impl PaStream {
    pub fn new(sample_format : PaSampleFormat) -> PaStream {
        PaStream {
            c_pa_stream : ptr::null(),
            sample_format : sample_format,
            c_input : None,
            c_output : None,
            unsafe_buffer : ptr::null()
        }
    }
    
    #[fixed_stack_segment] #[inline(never)]
    fn alloc_buffer(&mut self, sample_format : PaSampleFormat, frames_per_buffer : u32, channels : i32) -> () {
        match sample_format {
            PaFloat32   => self.unsafe_buffer = unsafe { malloc(4 as u64 * frames_per_buffer as u64 * channels as u64) },
            PaInt32     => self.unsafe_buffer = unsafe { malloc(4  as u64 * frames_per_buffer  as u64 * channels as u64 ) },
            PaInt16     => self.unsafe_buffer = unsafe { malloc(2  as u64 * frames_per_buffer  as u64 * channels as u64 ) },
            PaInt8      => self.unsafe_buffer = unsafe { malloc(1  as u64 * frames_per_buffer  as u64 * channels as u64 ) },
            PaUInt8     => self.unsafe_buffer = unsafe { malloc(1  as u64 * frames_per_buffer  as u64 * channels as u64 ) },
            _           => fail!("Format not supported for the moment.")
        }
    } 

    #[fixed_stack_segment] #[inline(never)]
    pub fn open_stream(&mut self,
                       input_parameters : Option<&PaStreamParameters>,
                       output_parameters : Option<&PaStreamParameters>, 
                       sample_rate : f64, 
                       frames_per_buffer : u32, 
                       stream_flags : PaStreamFlags)
                       -> PaError {
        
        if !input_parameters.is_none() {
            self.c_input = Some(input_parameters.unwrap().unwrap());
            self.alloc_buffer(input_parameters.unwrap().sample_format, frames_per_buffer, input_parameters.unwrap().channel_count as i32);
        }
        if !output_parameters.is_none() {
            self.c_output = Some(output_parameters.unwrap().unwrap());
        }


        unsafe {
            if !self.c_input.is_none() && 
               !self.c_output.is_none() {
                ffi::Pa_OpenStream(&self.c_pa_stream, &(self.c_input.unwrap()), &(self.c_output.unwrap()), sample_rate as c_double, frames_per_buffer, stream_flags, None, ptr::null())
            }
            else if self.c_output.is_none() {

                ffi::Pa_OpenStream(&self.c_pa_stream, &(self.c_input.unwrap()), ptr::null(), sample_rate as c_double, frames_per_buffer, stream_flags, None, ptr::null())
            }
            else if self.c_input.is_none() {

                ffi::Pa_OpenStream(&self.c_pa_stream, ptr::null(), &(self.c_input.unwrap()), sample_rate as c_double, frames_per_buffer, stream_flags, None, ptr::null())
            }
            else {
                PaBadStreamPtr
            }
        }
    }

    /// Closes an audio stream. If the audio stream is active it discards any pending buffers as if abort_tream() had been called.
    #[fixed_stack_segment] #[inline(never)]
    pub fn close_stream(&mut self) -> PaError {
        unsafe {
            ffi::Pa_CloseStream(self.c_pa_stream)
        }
    }

    /// Commences audio processing.
    #[fixed_stack_segment] #[inline(never)]
    pub fn start(&mut self) -> PaError {
        unsafe {
            ffi::Pa_StartStream(self.c_pa_stream)
        }
    }

    /// Terminates audio processing. It waits until all pending audio buffers have been played before it returns.
    #[fixed_stack_segment] #[inline(never)]
    pub fn stop(&mut self) -> PaError {
        unsafe {
            ffi::Pa_StopStream(self.c_pa_stream)
        }
    }

    /// Terminates audio processing immediately without waiting for pending buffers to complete.
    #[fixed_stack_segment] #[inline(never)]
    pub fn abort(&mut self) -> PaError {
        unsafe {
            ffi::Pa_AbortStream(self.c_pa_stream)
        }
    }

    /**
    * Determine whether the stream is stopped. 
    * A stream is considered to be stopped prior to a successful call to start_stream and after a successful call to stop_stream or abort_stream. 
    * If a stream callback returns a value other than PaContinue the stream is NOT considered to be stopped.
    *
    * Return one (1) when the stream is stopped, zero (0) when the stream is running or, a PaErrorCode (which are always negative) if PortAudio is not initialized or an error is encountered.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_stopped(&self) -> PaError {
        unsafe {
            ffi::Pa_IsStreamStopped(self.c_pa_stream)
        }
    }

    /**
    * Determine whether the stream is active. A stream is active after a successful call to start_stream(),
    * until it becomes inactive either as a result of a call to stop_stream() or abort_stream(),
    * or as a result of a return value other than paContinue from the stream callback. 
    * In the latter case, the stream is considered inactive after the last buffer has finished playing.
    *
    * Return one (1) when the stream is active (ie playing or recording audio), zero (0) when not playing or, 
    * a PaErrorCode (which are always negative) if PortAudio is not initialized or an error is encountered.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_active(&self) -> PaError {
        unsafe {
            ffi::Pa_IsStreamActive(self.c_pa_stream)
        }
    }

    #[fixed_stack_segment] #[inline(never)]
    pub fn get_stream_time(&self) -> PaTime {
        unsafe {
            ffi::Pa_GetStreamTime(self.c_pa_stream)
        }
    }

    #[fixed_stack_segment] #[inline(never)]
    pub fn get_stream_cpu_load(&self) -> f64 {
        unsafe {
            ffi::Pa_GetStreamCpuLoad(self.c_pa_stream)
        }
    }

    #[fixed_stack_segment] #[inline(never)]
    pub fn get_stream_read_available(&self) -> i64 {
        unsafe {
            ffi::Pa_GetStreamReadAvailable(self.c_pa_stream)
        }
    }

    /**
    * Retrieve the number of frames that can be written to the stream without waiting.
    *
    * Return a non-negative value representing the maximum number of frames that can be written to the stream without blocking or busy waiting or, 
    * a PaErrorCode (which are always negative) if PortAudio is not initialized or an error is encountered.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_stream_write_available(&self) -> i64 {
        unsafe {
            ffi::Pa_GetStreamWriteAvailable(self.c_pa_stream)
        }
    }

    #[fixed_stack_segment] #[inline(never)]
    pub fn read<T>(&self, frames_per_buffer : u32) -> Result<~[T], PaError> {
        let err = 
        unsafe {
            ffi::Pa_ReadStream(self.c_pa_stream, self.unsafe_buffer, frames_per_buffer)
        };
        // Temporary OSX Fixe : Return always PaInputOverflowed
        Ok(unsafe { vec::raw::from_buf_raw::<T>(self.unsafe_buffer as *T, (frames_per_buffer * 2) as uint) })
        // match err {
        //  PaNoError           => Ok(unsafe { vec::raw::from_buf_raw::<T>(self.unsafe_buffer as *T, (frames_per_buffer * 2) as uint) }),
        //  _                   => Err(err)
        // }
    }

    #[fixed_stack_segment] #[inline(never)]
    pub fn write<T>(&self, output_buffer : ~[T], frames_per_buffer : u32) -> PaError {
        unsafe {
            ffi::Pa_WriteStream(self.c_pa_stream, vec::raw::to_ptr::<T>(output_buffer) as *c_void, frames_per_buffer)
        }
    }
}
