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

//! Host api management

use pa::{
    HostApiIndex,
    HostApiInfo,
    HostApiTypeId,
    DeviceIndex
};
use ffi;

/// Retrieve the number of available host APIs.
/// Even if a host API is available it may have no devices available.
///
/// Return a non-negative value indicating the number of available host APIs or,
/// a PaErrorCode (which are always negative) if PortAudio is not initialized or
/// an error is encountered.
pub fn get_api_count() -> HostApiIndex {
    unsafe {
        ffi::Pa_GetHostApiCount()
    }
}

/// Retrieve the index of the default host API.
/// The default host API will be the lowest common denominator host API
/// on the current platform and is unlikely to provide the best performance.
///
/// Return a non-negative value ranging from 0 to (get_host_api_count()-1)
/// indicating the default host API index or, a PaErrorCode (which are always
/// negative) if PortAudio is not initialized or an error is encountered.
pub fn get_default_api() -> HostApiIndex {
    unsafe {
        ffi::Pa_GetDefaultHostApi()
    }
}

/// Retrieve a pointer to a structure containing information about a specific host
/// Api.
///
/// # Arguments
/// * host_api - A valid host API index ranging from 0 to (Pa_GetHostApiCount()-1)
///
/// Return Some(PaHostApiInfo) describing a specific host API. If the hostApi
/// parameter is out of range or an error is encountered, the function returns None.
pub fn get_api_info(host_api: HostApiIndex) -> Option<HostApiInfo> {
    let c_host_info = unsafe { ffi::Pa_GetHostApiInfo(host_api) };
    if c_host_info.is_null() {
        None
    }
    else {
        Some(HostApiInfo::wrap(c_host_info))
    }
}

/// Convert a static host API unique identifier, into a runtime host API index.
///
/// # Arguments
/// * typde_id - A unique host API identifier belonging to the PaHostApiTypeId
/// enumeration.
///
/// Return a valid HostApiIndex ranging from 0 to (get_host_api_count()-1) or,
/// a PaErrorCode (which are always negative) if PortAudio is not initialized or
/// an error is encountered.
pub fn api_type_id_to_host_api_index(type_id: HostApiTypeId) -> HostApiIndex {
    unsafe {
        ffi::Pa_HostApiTypeIdToHostApiIndex(type_id as i32)
    }
}

/// Convert a host-API-specific device index to standard PortAudio device index.
/// This function may be used in conjunction with the deviceCount field of
/// PaHostApiInfo to enumerate all devices for the specified host API.
///
/// # Arguments
/// * host_api - A valid host API index ranging from 0 to (get_host_api_count()-1)
/// * host_api_device_index - A valid per-host device index in the range 0 to
/// (get_host_api_info(host_api).device_count-1)
///
/// Return a non-negative PaDeviceIndex ranging from 0 to (get_device_count()-1)
/// or, a PaErrorCode (which are always negative) if PortAudio is not initialized
/// or an error is encountered.
pub fn api_device_index_to_device_index(host_api: HostApiIndex,
                                        host_api_device_index: int) -> DeviceIndex {
    unsafe {
        ffi::Pa_HostApiDeviceIndexToDeviceIndex(host_api,
                                                host_api_device_index as i32)
    }
}
