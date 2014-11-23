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

//! Device management

use pa::{
    DeviceIndex,
    DeviceInfo
};
use ffi;

/// Retrieve the number of available devices. The number of available devices may
/// be zero.
///
/// Return A non-negative value indicating the number of available devices or,
/// a PaErrorCode (which are always negative) if PortAudio is not initialized or
/// an error is encountered.
pub fn get_count() -> DeviceIndex {
    unsafe {
        ffi::Pa_GetDeviceCount()
    }
}

/// Retrieve the index of the default input device.
/// The result can be used in the inputDevice parameter to open_stream().
///
/// Return the default input device index for the default host API, or PaNoDevice
/// if no default input device is available or an error was encountered
pub fn get_default_input() -> DeviceIndex {
    unsafe {
        ffi::Pa_GetDefaultInputDevice()
    }
}

/// Retrieve the index of the default output device. The result can be
/// used in the outputDevice parameter to open_stream().
///
/// Return the default output device index for the default host API, or PaNoDevice
/// if no default output device is available or an error was encountered.
pub fn get_default_output() -> DeviceIndex {
    unsafe {
        ffi::Pa_GetDefaultOutputDevice()
    }
}

/// Retrieve a pointer to a PaDeviceInfo structure containing information about
/// the specified device.
///
/// # Arguments
/// * device - A valid device index in the range 0 to (Pa_GetDeviceCount()-1)
///
/// Return Some(PaDeviceInfo) or, If the device parameter is out of range the
/// function returns None.
pub fn get_info(device: DeviceIndex) -> Option<DeviceInfo> {
    let c_info = unsafe { ffi::Pa_GetDeviceInfo(device) };
    if c_info.is_null() {
        None
    }
    else {
        Some(DeviceInfo::wrap(c_info))
    }
}
