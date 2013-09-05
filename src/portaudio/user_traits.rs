/*!
* Traits containing the callback functions
*
*
*/

use types::*;

pub trait PortaudioCallback {
	/**
	*
	* T = type defined by PaSampleFormat
	*/
	fn callback_function(&self, input_buffer : ~[f32], frames_per_buffer : u32) -> (PaStreamCallbackResult, ~[f32]);
}