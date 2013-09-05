/*!
* The ASIO specific API.
*/

use types::*;
use ffi;


#[fixed_stack_segment] #[inline(never)]
pub fn get_available_buffer_sizes(device : PaDeviceIndex) -> Result<(i32, i32, i32, i32), PaError> {
    let min_buffer_size_frames : i32 = 0;
    let max_buffer_size_frames : i32 = 0;
    let preferred_buffer_size_frames : i32 = 0;
    let granularity : i32 = 0;
    let mut error : PaError;
    unsafe {
        error = ffi::PaAsio_GetAvailableBufferSizes(device, &min_buffer_size_frames, &max_buffer_size_frames, &preferred_buffer_size_frames, &granularity);
    }

    match error {
        PaNoError   => Ok((min_buffer_size_frames, max_buffer_size_frames, preferred_buffer_size_frames, granularity)),
        _           => Err(error)
    }
}
