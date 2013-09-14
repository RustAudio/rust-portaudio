/*!
* The MAC_CORE specific API.
*/

use pa::*;
use types::*;
use ffi;

pub static PaMacCoreChangeDeviceParameters : u32 = 0x01;
pub static PaMacCoreFailIfConversionRequired : u32 = 0x02;
pub static PaMacCoreConversionQualityMin : u32 = 0x0100;
pub static PaMacCoreConversionQualityMedium : u32 = 0x0200;
pub static PaMacCoreConversionQualityLow : u32 = 0x0300;
pub static PaMacCoreConversionQualityHigh : u32 = 0x0400;
pub static PaMacCoreConversionQualityMax : u32 = 0x0000;
pub static PaMacCorePlayNice : u32 = 0x00;
pub static PaMacCorePro : u32 = 0x01;
pub static PaMacCoreMinimizeCPUButPlayNice : u32 = 0x0100;
pub static PaMacCoreMinimizeCPU : u32 = 0x0101;


/// Not implemented
pub struct PaMacCoreStreamInfo {
    size : u32,
    host_api_type : PaHostApiTypeId,
    version : u32,
    flags : u32,
    channel_map : *i32,
    channel_map_size : u32    
}

pub trait MacCore {
    fn get_stream_input_device(&self) -> PaDeviceIndex;
    fn get_stream_output_device(&self) -> PaDeviceIndex; 
}

// #[fixed_stack_segment] #[inline(never)]
// fn get_buffer_size_range(device : PaDeviceIndex) -> Result<(u32, u32), PaError> {
//     let mut min_buffer_size_frames : u32 = 0;
//     let mut max_buffer_size_frames : u32 = 0;
//     let err = unsafe { ffi::PaMacCore_GetBufferSizeRange(device, &min_buffer_size_frames, &max_buffer_size_frames) };
//     match err {
//         PaNoError   => Ok((min_buffer_size_frames, max_buffer_size_frames)),
//         _           => Err(err)
//     }
// }


impl<S> MacCore for PaStream<S> {
    #[fixed_stack_segment] #[inline(never)]
    fn get_stream_input_device(&self) -> PaDeviceIndex {
        unsafe {
            ffi::PaMacCore_GetStreamInputDevice(self.get_c_pa_stream())
        }
    }
    #[fixed_stack_segment] #[inline(never)]
    fn get_stream_output_device(&self) -> PaDeviceIndex {
        unsafe {
            ffi::PaMacCore_GetStreamOutputDevice(self.get_c_pa_stream())
        }
    }
}