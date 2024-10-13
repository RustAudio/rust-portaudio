//! This module aims to provide a user-friendly rust-esque wrapper around the portaudio Stream
//! types.
//!
//! The primary type of interest is [**Stream**](./struct.Stream).

use ffi;
use libc;
use num::FromPrimitive;
use std::os::raw;
use std::{self, ptr};

use super::error::Error;
use super::types::{DeviceIndex, DeviceKind, SampleFormat, SampleFormatFlags, Time};
use super::Sample;

pub use self::callback_flags::CallbackFlags;
pub use self::flags::Flags;

/// There are two **Mode**s with which a **Stream** can be set: [**Blocking**](./struct.Blocking)
/// and [**NonBlocking**](./struct.NonBlocking).
pub trait Mode {}

/// Types used to open a **Stream** via the
/// [**PortAudio::open_blocking_stream**](../struct.PortAudio.html#method.open_blocking_stream) and
/// [**PortAudio::open_non_blocking_stream**](../struct.PortAudio.html#method.open_blocking_stream)
/// methods.
pub trait Settings {
    /// The **Flow** of the **Stream** (**Input**, **Output** or **Duplex**).
    type Flow;
    /// Construct the **Stream**'s **Flow** alongside the rest of its settings.
    fn into_flow_and_settings(self) -> (Self::Flow, f64, u32, Flags);
}

/// There are three possible **Flow**s available for a **Stream**: [**Input**](./struct.Input),
/// [**Out**](./struct.Output) and [**Duplex**](./struct.Duplex).
pub trait Flow {
    /// The type of buffer(s) necessary for transferring audio in a Blocking stream.
    type Buffer;
    /// The arguments passed to the non-blocking stream callback.
    type CallbackArgs;
    /// Timing information for the buffer passed to the stream callback.
    type CallbackTimeInfo;
    /// Construct a new **Self::Buffer**.
    fn new_buffer(&self, frames_per_buffer: u32) -> Self::Buffer;
    /// Necessary for dynamically acquiring bi-directional params for Pa_OpenStream.
    fn params_both_directions(
        &self,
    ) -> (
        Option<ffi::PaStreamParameters>,
        Option<ffi::PaStreamParameters>,
    );
    /// Constructs the **Flow**'s associated **CallbackArgs** from the non-blocking C API stream
    /// parameters.
    fn new_callback_args(
        input: *const raw::c_void,
        output: *mut raw::c_void,
        frame_count: raw::c_ulong,
        time_info: *const ffi::PaStreamCallbackTimeInfo,
        flags: ffi::PaStreamCallbackFlags,
        in_channels: i32,
        out_channels: i32,
    ) -> Self::CallbackArgs;
}

/// **Streams** that can be read by the user.
pub trait Reader: Flow {
    /// The sample format for the readable buffer.
    type Sample;
    /// Borrow the readable **Buffer**.
    fn readable_buffer(blocking: &Blocking<Self::Buffer>) -> &Buffer;
    /// The number of channels in the readable **Buffer**.
    fn channel_count(&self) -> i32;
}

/// **Streams** that can be written to by the user for output to some DAC.
pub trait Writer: Flow {
    /// The sample format for the writable buffer.
    type Sample;
    /// Mutably borrow the the writable **Buffer**.
    fn writable_buffer(blocking: &mut Blocking<Self::Buffer>) -> &mut Buffer;
    /// The number of channels in the writable **Buffer**.
    fn channel_count(&self) -> i32;
}

/// An alias for the boxed Callback function type.
type CallbackFn = dyn FnMut(
    *const raw::c_void,
    *mut raw::c_void,
    raw::c_ulong,
    *const ffi::PaStreamCallbackTimeInfo,
    ffi::PaStreamCallbackFlags,
) -> ffi::PaStreamCallbackResult;

/// A wrapper around a user-given **CallbackFn** that can be sent to PortAudio.
struct CallbackFnWrapper {
    f: Box<CallbackFn>,
}

/// Timing information for the buffer passed to the input stream callback.
///
/// Time values are expressed in seconds and are synchronised with the time base used by
/// `Stream::time` method for the associated stream.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InputCallbackTimeInfo {
    /// The time when the stream callback was invoked.
    pub current: Time,
    /// The time when the first sample of the input buffer was captured at the ADC input.
    pub buffer_adc: Time,
}

/// Timing information for the buffer passed to the output stream callback.
///
/// Time values are expressed in seconds and are synchronised with the time base used by
/// `Stream::time` method for the associated stream.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OutputCallbackTimeInfo {
    /// The time when the stream callback was invoked.
    pub current: Time,
    /// The time when the first sample of the output buffer will output the DAC.
    pub buffer_dac: Time,
}

/// Timing information for the buffers passed to the stream callback.
///
/// Time values are expressed in seconds and are synchronised with the time base used by
/// `Stream::time` method for the associated stream.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DuplexCallbackTimeInfo {
    /// The time when the stream callback was invoked.
    pub current: Time,
    /// The time when the first sample of the input buffer was captured at the ADC input.
    pub in_buffer_adc: Time,
    /// The time when the first sample of the output buffer will output the DAC.
    pub out_buffer_dac: Time,
}

/// Arguments given to a **NonBlocking** **Input** **Stream**'s **CallbackFn**.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InputCallbackArgs<'a, I: 'a> {
    /// The buffer of interleaved samples read from the **Input** **Stream**'s ADC.
    pub buffer: &'a [I],
    /// The number of frames of audio data stored within the `buffer`.
    pub frames: usize,
    /// Flags indicating the current state of the stream and whether or not any special edge cases
    /// have occurred.
    pub flags: CallbackFlags,
    /// Timing information relevant to the callback.
    pub time: InputCallbackTimeInfo,
}

/// Arguments given to a **NonBlocking** **Input** **Stream**'s **CallbackFn**.
#[derive(Debug, PartialEq)]
pub struct OutputCallbackArgs<'a, O: 'a> {
    /// The **Output** **Stream**'s buffer, to which we will write our interleaved audio data.
    pub buffer: &'a mut [O],
    /// The number of frames of audio data stored within the `buffer`.
    pub frames: usize,
    /// Flags indicating the current state of the stream and whether or not any special edge cases
    /// have occurred.
    pub flags: CallbackFlags,
    /// Timing information relevant to the callback.
    pub time: OutputCallbackTimeInfo,
}

/// Arguments given to a **NonBlocking** **Input** **Stream**'s **CallbackFn**.
#[derive(Debug, PartialEq)]
pub struct DuplexCallbackArgs<'a, I: 'a, O: 'a> {
    /// The buffer of interleaved samples read from the **Stream**'s ADC.
    pub in_buffer: &'a [I],
    /// The **Stream**'s output buffer, to which we will write interleaved audio data.
    pub out_buffer: &'a mut [O],
    /// The number of frames of audio data stored within the `buffer`.
    pub frames: usize,
    /// Flags indicating the current state of the stream and whether or not any special edge cases
    /// have occurred.
    pub flags: CallbackFlags,
    /// Timing information relevant to the callback.
    pub time: DuplexCallbackTimeInfo,
}

/// A **Stream** **Mode** representing a blocking stream.
///
/// Unlike the **NonBlocking** stream, PortAudio requires that we manually manage the audio data
/// buffer for the **Blocking** stream.
pub struct Blocking<B> {
    buffer: B,
}

/// A **Stream** **Mode** representing a non-blocking stream.
pub struct NonBlocking {
    callback: Box<CallbackFnWrapper>,
}

/// A type-safe PortAudio PaStream wrapper.
///
/// **F** is the stream's directional [**Flow**][1]:
///
/// - [**Input**][2] - Receives data from an input device's ADC.
/// - [**Output**][3] - Sends data to an output device's DAC.
/// - [**Duplex**][4] - Receives and Sends data on two devices synchronously.
///
/// A **Stream** of a particular [**Flow**][1] type can be opened by passing the **Flow**'s
/// associated **Settings** type to either of the [**PortAudio::open_blocking_stream**][12] or
/// [**PortAudio::open_non_blocking_stream**][13] methods.
///
/// - [**InputSettings**][14] -> [**Input**][2]
/// - [**OutputSettings**][15] -> [**Output**][3]
/// - [**DuplexSettings**][16] -> [**Duplex**][4]
///
/// **M** is the stream's [**Mode**][5]:
///
/// - [**Blocking**][6] - The stream will be run on the caller's thread. For [**Blocking**][6]
/// streams, a user can read from [**Input**][2] and [**Duplex**][4] streams using the
/// [**Stream::read_available**][8] and [**Stream::read**][9] methods and write to [**Output**][3]
/// and [**Duplex**][4] streams using the [**Stream::write_available**][10] and
/// [**Stream::write**][11] methods. A [**Blocking**][6] **Stream** can be opened using the
/// [**PortAudio::open_blocking_stream][12]** method.
/// - [**NonBlocking**][7] - The stream will be run on a separate thread. [**NonBlocking][7]
/// streams are read and written to via the callback arguments that are associated with the
/// **Stream**'s [**Flow**][1] type:
///     - **Input** -> [**InputCallbackArgs**](./struct.InputCallbackArgs.html)
///     - **Output** -> [**OutputCallbackArgs**](./struct.OutputCallbackArgs.html)
///     - **Duplex** -> [**DuplexCallbackArgs**](./struct.DuplexCallbackArgs.html)
/// A [**NonBlocking**][7] **Stream** can be opened using the
/// [**PortAudio::open_non_blocking_stream][13]** method.
///
/// A **Stream** may only live as long as the **PortAudio** instance from which it was spawned and
/// no longer.
///
/// The original PortAudio documentation for the **PaStream** type can be found [here][17].
///
/// [1]: ./trait.Flow.html
/// [2]: ./struct.Input.html
/// [3]: ./struct.Output.html
/// [4]: ./struct.Duplex.html
/// [5]: ./trait.Mode.html
/// [6]: ./struct.Blocking.html
/// [7]: ./struct.NonBlocking.html
/// [8]: ./struct.Stream.html#method.read_available
/// [9]: ./struct.Stream.html#method.read
/// [10]: ./struct.Stream.html#method.write_available
/// [11]: ./struct.Stream.html#method.write
/// [12]: ../struct.PortAudio.html#method.open_blocking_stream
/// [13]: ../struct.PortAudio.html#method.open_non_blocking_stream
/// [14]: ./struct.InputSettings.html
/// [15]: ./struct.OutputSettings.html
/// [16]: ./struct.DuplexSettings.html
/// [17]: http://portaudio.com/docs/v19-doxydocs/portaudio_8h.html#a19874734f89958fccf86785490d53b4c
#[allow(dead_code)]
pub struct Stream<M, F> {
    pa_stream: *mut ffi::PaStream,
    mode: M,
    flow: F,
    port_audio_life: std::sync::Arc<super::Life>,
}

/// Parameters for one direction (input or output) of a stream.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Parameters<S> {
    /// Index of the device to be used, or a variant indicating to use the host-specific API.
    pub device: DeviceKind,
    /// The number of channels for this device
    pub channel_count: i32,
    /// The suggested latency for this device
    pub suggested_latency: Time,
    /// Indicates the format of the audio buffer.
    ///
    /// If `true`, audio data is passed as a single buffer with all channels interleaved.
    ///
    /// If `false`, audio data is passed as an array of pointers to separate buffers, one buffer
    /// for each channel.
    pub is_interleaved: bool,
    /// Sample format of the audio data provided to/by the device.
    sample_format: std::marker::PhantomData<S>,
}

/// Settings used to construct an **Input** **Stream**.
#[derive(Copy, Clone, Debug)]
pub struct InputSettings<I> {
    /// The set of Parameters necessary for constructing the **Stream**.
    pub params: Parameters<I>,
    /// The number of audio frames read per second.
    pub sample_rate: f64,
    /// The number of audio frames that are read per buffer.
    pub frames_per_buffer: u32,
    /// Any special **Stream** behaviour we require given as a set of flags.
    pub flags: Flags,
}

/// Settings used to construct an **Out** **Stream**.
#[derive(Copy, Clone, Debug)]
pub struct OutputSettings<O> {
    /// The set of Parameters necessary for constructing the **Stream**.
    pub params: Parameters<O>,
    /// The number of audio frames written per second.
    pub sample_rate: f64,
    /// The number of audio frames requested per buffer.
    pub frames_per_buffer: u32,
    /// Any special **Stream** behaviour we require given as a set of flags.
    pub flags: Flags,
}

/// Settings used to construct a **Duplex** **Stream**.
#[derive(Copy, Clone, Debug)]
pub struct DuplexSettings<I, O> {
    /// The set of Parameters necessary for constructing the input **Stream**.
    pub in_params: Parameters<I>,
    /// The set of Parameters necessary for constructing the output **Stream**.
    pub out_params: Parameters<O>,
    /// The number of audio frames written per second.
    pub sample_rate: f64,
    /// The number of audio frames requested per buffer.
    pub frames_per_buffer: u32,
    /// Any special **Stream** behaviour we require given as a set of flags.
    pub flags: Flags,
}

/// A type of **Flow** that describes an input-only **Stream**.
pub struct Input<I> {
    params: Parameters<I>,
}

/// A type of **Flow** that describes an output-only **Stream**.
pub struct Output<O> {
    params: Parameters<O>,
}

/// A type of **Flow** that describes a bi-directional (input *and* output) **Stream**.
pub struct Duplex<I, O> {
    in_params: Parameters<I>,
    out_params: Parameters<O>,
}

unsafe impl Send for NonBlocking {}
unsafe impl<M, F> Send for Stream<M, F>
where
    M: Send,
    F: Send,
{
}

impl<S> Parameters<S> {
    /// Construct a new **Parameters**.
    pub fn new(
        device: DeviceIndex,
        channel_count: i32,
        is_interleaved: bool,
        suggested_latency: Time,
    ) -> Self {
        Self::new_internal(
            device.into(),
            channel_count,
            is_interleaved,
            suggested_latency,
        )
    }

    /// The same as **Parameters::new**, but the device(s) to be used are specified in the host
    /// api specific stream info structure.
    ///
    /// **NOTE:** This has not yet been tested.
    pub fn host_api_specific_device(
        channel_count: i32,
        is_interleaved: bool,
        suggested_latency: Time,
    ) -> Self {
        let kind = DeviceKind::UseHostApiSpecificDeviceSpecification;
        Self::new_internal(kind, channel_count, is_interleaved, suggested_latency)
    }

    fn new_internal(
        device_kind: DeviceKind,
        channel_count: i32,
        is_interleaved: bool,
        suggested_latency: Time,
    ) -> Self {
        Parameters {
            device: device_kind,
            channel_count: channel_count,
            is_interleaved: is_interleaved,
            suggested_latency: suggested_latency,
            sample_format: std::marker::PhantomData,
        }
    }
}

/// Simplify implementation of one-way-Stream Settings types.
macro_rules! impl_half_duplex_settings {
    ($name:ident) => {
        impl<S> $name<S> {
            /// Construct the settings from the given `params`, `sample_rate` and
            /// `frames_per_buffer` with an empty set of **StreamFlags**.
            pub fn new(params: Parameters<S>, sample_rate: f64, frames_per_buffer: u32) -> Self {
                Self::with_flags(params, sample_rate, frames_per_buffer, Flags::empty())
            }

            /// Construct the settings with the given **Parameters**, `sample_rate`,
            /// `frames_per_buffer` and **StreamFlags**.
            pub fn with_flags(
                params: Parameters<S>,
                sample_rate: f64,
                frames_per_buffer: u32,
                flags: Flags,
            ) -> Self {
                $name {
                    params: params,
                    sample_rate: sample_rate,
                    frames_per_buffer: frames_per_buffer,
                    flags: flags,
                }
            }
        }
    };
}

impl_half_duplex_settings!(OutputSettings);
impl_half_duplex_settings!(InputSettings);

impl<I, O> DuplexSettings<I, O> {
    /// Construct the settings from the given `params`, `sample_rate` and
    /// `frames_per_buffer` with an empty set of **StreamFlags**.
    pub fn new(
        in_params: Parameters<I>,
        out_params: Parameters<O>,
        sample_rate: f64,
        frames_per_buffer: u32,
    ) -> Self {
        Self::with_flags(
            in_params,
            out_params,
            sample_rate,
            frames_per_buffer,
            Flags::empty(),
        )
    }

    /// Construct the settings with the given **Parameters**, `sample_rate`,
    /// `frames_per_buffer` and **StreamFlags**.
    pub fn with_flags(
        in_params: Parameters<I>,
        out_params: Parameters<O>,
        sample_rate: f64,
        frames_per_buffer: u32,
        flags: Flags,
    ) -> Self {
        DuplexSettings {
            in_params: in_params,
            out_params: out_params,
            sample_rate: sample_rate,
            frames_per_buffer: frames_per_buffer,
            flags: flags,
        }
    }
}

impl<I> Flow for Input<I>
where
    I: Sample + 'static,
{
    type Buffer = Buffer;
    type CallbackArgs = InputCallbackArgs<'static, I>;
    type CallbackTimeInfo = InputCallbackTimeInfo;

    fn new_buffer(&self, frames_per_buffer: u32) -> Self::Buffer {
        let channel_count = self.params.channel_count;
        Buffer::new::<I>(frames_per_buffer, channel_count)
    }

    fn params_both_directions(
        &self,
    ) -> (
        Option<ffi::PaStreamParameters>,
        Option<ffi::PaStreamParameters>,
    ) {
        (Some(self.params.into()), None)
    }

    fn new_callback_args(
        input: *const raw::c_void,
        _output: *mut raw::c_void,
        frame_count: raw::c_ulong,
        time_info: *const ffi::PaStreamCallbackTimeInfo,
        flags: ffi::PaStreamCallbackFlags,
        in_channels: i32,
        _out_channels: i32,
    ) -> Self::CallbackArgs {
        let flags = CallbackFlags::from_bits(flags).unwrap_or_else(|| CallbackFlags::empty());
        let time = unsafe {
            InputCallbackTimeInfo {
                current: (*time_info).currentTime,
                buffer_adc: (*time_info).inputBufferAdcTime,
            }
        };
        // TODO: At the moment, we assume the buffer is interleaved. We need to check whether or
        // not buffer is interleaved here. This should probably an extra type parameter (along-side
        // the Sample type param).
        let buffer: &[I] = {
            let buffer_len = in_channels as usize * frame_count as usize;
            let buffer_ptr = input as *const I;
            unsafe { std::slice::from_raw_parts(buffer_ptr, buffer_len) }
        };
        InputCallbackArgs {
            buffer: buffer,
            frames: frame_count as usize,
            flags: flags,
            time: time,
        }
    }
}

impl<O> Flow for Output<O>
where
    O: Sample + 'static,
{
    type Buffer = Buffer;
    type CallbackArgs = OutputCallbackArgs<'static, O>;
    type CallbackTimeInfo = OutputCallbackTimeInfo;

    fn params_both_directions(
        &self,
    ) -> (
        Option<ffi::PaStreamParameters>,
        Option<ffi::PaStreamParameters>,
    ) {
        (None, Some(self.params.into()))
    }

    fn new_buffer(&self, frames_per_buffer: u32) -> Self::Buffer {
        let channel_count = self.params.channel_count;
        Buffer::new::<O>(frames_per_buffer, channel_count)
    }

    fn new_callback_args(
        _input: *const raw::c_void,
        output: *mut raw::c_void,
        frame_count: raw::c_ulong,
        time_info: *const ffi::PaStreamCallbackTimeInfo,
        flags: ffi::PaStreamCallbackFlags,
        _in_channels: i32,
        out_channels: i32,
    ) -> Self::CallbackArgs {
        let flags = CallbackFlags::from_bits(flags).unwrap_or_else(|| CallbackFlags::empty());
        let time = unsafe {
            OutputCallbackTimeInfo {
                current: (*time_info).currentTime,
                buffer_dac: (*time_info).outputBufferDacTime,
            }
        };
        // TODO: At the moment, we assume the buffer is interleaved. We need to check whether or
        // not buffer is interleaved here. This should probably an extra type parameter (along-side
        // the Sample type param).
        let buffer: &mut [O] = {
            let buffer_len = out_channels as usize * frame_count as usize;
            let buffer_ptr = output as *mut O;
            unsafe { std::slice::from_raw_parts_mut(buffer_ptr, buffer_len) }
        };
        OutputCallbackArgs {
            buffer: buffer,
            frames: frame_count as usize,
            flags: flags,
            time: time,
        }
    }
}

impl<I, O> Flow for Duplex<I, O>
where
    I: Sample + 'static,
    O: Sample + 'static,
{
    type Buffer = (Buffer, Buffer);
    type CallbackArgs = DuplexCallbackArgs<'static, I, O>;
    type CallbackTimeInfo = DuplexCallbackTimeInfo;

    fn params_both_directions(
        &self,
    ) -> (
        Option<ffi::PaStreamParameters>,
        Option<ffi::PaStreamParameters>,
    ) {
        (Some(self.in_params.into()), Some(self.out_params.into()))
    }

    fn new_buffer(&self, frames_per_buffer: u32) -> Self::Buffer {
        let in_channel_count = self.in_params.channel_count;
        let in_buffer = Buffer::new::<I>(frames_per_buffer, in_channel_count);
        let out_channel_count = self.out_params.channel_count;
        let out_buffer = Buffer::new::<O>(frames_per_buffer, out_channel_count);
        (in_buffer, out_buffer)
    }

    fn new_callback_args(
        input: *const raw::c_void,
        output: *mut raw::c_void,
        frame_count: raw::c_ulong,
        time_info: *const ffi::PaStreamCallbackTimeInfo,
        flags: ffi::PaStreamCallbackFlags,
        in_channels: i32,
        out_channels: i32,
    ) -> Self::CallbackArgs {
        let flags = CallbackFlags::from_bits(flags).unwrap_or_else(|| CallbackFlags::empty());
        let time = unsafe {
            DuplexCallbackTimeInfo {
                current: (*time_info).currentTime,
                in_buffer_adc: (*time_info).inputBufferAdcTime,
                out_buffer_dac: (*time_info).outputBufferDacTime,
            }
        };
        // TODO: At the moment, we assume these buffers are interleaved. We need to check whether
        // or not buffer is interleaved here. This should probably an extra type parameter
        // (along-side the Sample type param).
        let in_buffer: &[I] = {
            let buffer_len = in_channels as usize * frame_count as usize;
            let buffer_ptr = input as *const I;
            unsafe { std::slice::from_raw_parts(buffer_ptr, buffer_len) }
        };
        let out_buffer: &mut [O] = {
            let buffer_len = out_channels as usize * frame_count as usize;
            let buffer_ptr = output as *mut O;
            unsafe { std::slice::from_raw_parts_mut(buffer_ptr, buffer_len) }
        };
        DuplexCallbackArgs {
            in_buffer: in_buffer,
            out_buffer: out_buffer,
            frames: frame_count as usize,
            flags: flags,
            time: time,
        }
    }
}

impl<I> Reader for Input<I>
where
    I: Sample + 'static,
{
    type Sample = I;
    fn readable_buffer(blocking: &Blocking<<Input<I> as Flow>::Buffer>) -> &Buffer {
        &blocking.buffer
    }
    fn channel_count(&self) -> i32 {
        self.params.channel_count
    }
}

impl<I, O> Reader for Duplex<I, O>
where
    I: Sample + 'static,
    O: Sample + 'static,
{
    type Sample = I;
    fn readable_buffer(blocking: &Blocking<<Duplex<I, O> as Flow>::Buffer>) -> &Buffer {
        &blocking.buffer.0
    }
    fn channel_count(&self) -> i32 {
        self.in_params.channel_count
    }
}

impl<O> Writer for Output<O>
where
    O: Sample + 'static,
{
    type Sample = O;
    fn writable_buffer(blocking: &mut Blocking<<Output<O> as Flow>::Buffer>) -> &mut Buffer {
        &mut blocking.buffer
    }
    fn channel_count(&self) -> i32 {
        self.params.channel_count
    }
}

impl<I, O> Writer for Duplex<I, O>
where
    I: Sample + 'static,
    O: Sample + 'static,
{
    type Sample = O;
    fn writable_buffer(blocking: &mut Blocking<<Duplex<I, O> as Flow>::Buffer>) -> &mut Buffer {
        &mut blocking.buffer.1
    }
    fn channel_count(&self) -> i32 {
        self.out_params.channel_count
    }
}

/// The buffer used to transfer audio data between the input and output streams.
pub struct Buffer {
    data: *mut libc::c_void,
}

pub mod flags {
    //! A type safe wrapper around PortAudio's stream flags.
    use ffi;
    bitflags! {
        /// Flags used to control the behaviour of a stream. They are passed as parameters to
        /// Stream::open or Stream::open_default. Multiple flags may be used together.
        ///
        /// See the [bitflags repo](https://github.com/rust-lang/bitflags/blob/master/src/lib.rs)
        /// for examples of composing flags together.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Flags: ::std::os::raw::c_ulong {
            /// No flags.
            const NO_FLAG =                                       ffi::PA_NO_FLAG;
            /// Disable default clipping of out of range samples.
            const CLIP_OFF =                                      ffi::PA_CLIP_OFF;
            /// Disable default dithering.
            const DITHER_OFF =                                    ffi::PA_DITHER_OFF;
            /// Flag requests that where possible a full duplex stream will not discard overflowed
            /// input samples without calling the stream callback.
            const NEVER_DROP_INPUT =                              ffi::PA_NEVER_DROP_INPUT;
            /// Call the stream callback to fill initial output buffers, rather than the default
            /// behavior of priming the buffers with zeros (silence)
            const PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK = ffi::PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK;
            /// A mask specifying the platform specific bits.
            const PA_PLATFORM_SPECIFIC_FLAGS =                    ffi::PA_PLATFORM_SPECIFIC_FLAGS;
        }
    }

    impl ::std::fmt::Display for Flags {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(
                f,
                "{:?}",
                match self.bits() {
                    ffi::PA_NO_FLAG => "NO_FLAG",
                    ffi::PA_CLIP_OFF => "CLIP_OFF",
                    ffi::PA_DITHER_OFF => "DITHER_OFF",
                    ffi::PA_NEVER_DROP_INPUT => "NEVER_DROP_INPUT",
                    ffi::PA_PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK => {
                        "PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK"
                    }
                    ffi::PA_PLATFORM_SPECIFIC_FLAGS => "PLATFORM_SPECIFIC_FLAGS",
                    _ => "<Unknown StreamFlags>",
                }
            )
        }
    }
}

/// Describes stream availability and the number for frames available for reading/writing if there
/// is any.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Available {
    /// The number of frames available for reading.
    Frames(::std::os::raw::c_long),
    /// The input stream has overflowed.
    InputOverflowed,
    /// The output stream has underflowed.
    OutputUnderflowed,
}

pub mod callback_flags {
    //! A type safe wrapper around PortAudio's stream callback flags.
    use ffi;
    bitflags! {
        /// Flag bit constants for the status flags passed to the stream's callback function.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct CallbackFlags:  ::std::os::raw::c_ulong {
            /// No flags.
            const NO_FLAG          = ffi::PA_NO_FLAG;
            /// In a stream opened with paFramesPerBufferUnspecified, indicates that input data is
            /// all silence (zeros) because no real data is available. In a stream opened without
            /// `FramesPerBufferUnspecified`, it indicates that one or more zero samples have been
            /// inserted into the input buffer to compensate for an input underflow.
            const INPUT_UNDERFLOW  = ffi::INPUT_UNDERFLOW;
            /// In a stream opened with paFramesPerBufferUnspecified, indicates that data prior to
            /// the first sample of the input buffer was discarded due to an overflow, possibly
            /// because the stream callback is using too much CPU time. Otherwise indicates that
            /// data prior to one or more samples in the input buffer was discarded.
            const INPUT_OVERFLOW   = ffi::INPUT_OVERFLOW;
            /// Indicates that output data (or a gap) was inserted, possibly because the stream
            /// callback is using too much CPU time.
            const OUTPUT_UNDERFLOW = ffi::OUTPUT_UNDERFLOW;
            /// Indicates that output data will be discarded because no room is available.
            const OUTPUT_OVERFLOW  = ffi::OUTPUT_OVERFLOW;
            /// Some of all of the output data will be used to prime the stream, input data may be
            /// zero.
            const PRIMING_OUTPUT   = ffi::PRIMING_OUTPUT;
        }
    }

    impl ::std::fmt::Display for CallbackFlags {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(
                f,
                "{:?}",
                match self.bits() {
                    ffi::PA_NO_FLAG => "NO_FLAG",
                    ffi::INPUT_UNDERFLOW => "INPUT_UNDERFLOW",
                    ffi::INPUT_OVERFLOW => "INPUT_OVERFLOW",
                    ffi::OUTPUT_UNDERFLOW => "OUTPUT_UNDERFLOW",
                    ffi::OUTPUT_OVERFLOW => "OUTPUT_OVERFLOW",
                    ffi::PRIMING_OUTPUT => "PRIMING_INPUT",
                    _ => "<Unknown StreamCallbackFlags>",
                }
            )
        }
    }
}

/// Timing information for the buffers passed to the stream callback.
///
/// Time values are expressed in seconds and are synchronised with the time base used by
/// `Stream::time` method for the associated stream.
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CallbackTimeInfo {
    /// The time when the first sample of the input buffer was captured by the
    pub input_buffer_adc_time: Time,
    /// The time when the tream callback was invoked.
    pub current_time: Time,
    pub output_buffer_dac_time: Time,
}

/// A structure containing unchanging information about an open stream.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct Info {
    /// Struct version
    pub struct_version: i32,
    /// The input latency for this open stream
    pub input_latency: Time,
    /// The output latency for this open stream
    pub output_latency: Time,
    /// The sample rate for this open stream
    pub sample_rate: f64,
}

impl From<ffi::PaStreamInfo> for Info {
    fn from(info: ffi::PaStreamInfo) -> Info {
        Info {
            struct_version: info.structVersion,
            input_latency: info.inputLatency,
            output_latency: info.outputLatency,
            sample_rate: info.sampleRate,
        }
    }
}

impl<B> Mode for Blocking<B> {}
impl Mode for NonBlocking {}

impl<S: Sample> Parameters<S> {
    /// Converts the given `C_PaStreamParameters` into their respective **Parameters**.
    ///
    /// Returns `None` if the `sample_format` differs to that of the **S** **Sample** parameter.
    ///
    /// Returns `None` if the `device` index is neither a valid index or a
    /// `UseHostApiSpecificDeviceSpecification` flag.
    pub fn from_c_params(c_params: ffi::PaStreamParameters) -> Option<Self> {
        let sample_format_flags: SampleFormatFlags = c_params.sampleFormat.into();
        let is_interleaved = !sample_format_flags.contains(SampleFormatFlags::NON_INTERLEAVED);
        let c_sample_format = SampleFormat::from_flags(c_params.sampleFormat.into());
        if S::sample_format() != c_sample_format {
            return None;
        }
        let device = match c_params.device {
            n if n >= 0 => DeviceIndex(n as u32).into(),
            -1 => DeviceKind::UseHostApiSpecificDeviceSpecification,
            _ => return None,
        };
        Some(Parameters {
            device: device,
            channel_count: c_params.channelCount,
            suggested_latency: c_params.suggestedLatency,
            is_interleaved: is_interleaved,
            sample_format: std::marker::PhantomData,
        })
    }
}

impl<S: Sample> From<Parameters<S>> for ffi::PaStreamParameters {
    /// Converts the **Parameters** into its matching `C_PaStreamParameters`.
    fn from(params: Parameters<S>) -> Self {
        let Parameters {
            device,
            channel_count,
            suggested_latency,
            is_interleaved,
            ..
        } = params;
        let sample_format = S::sample_format();
        let mut sample_format_flags = sample_format.flags();
        if !is_interleaved {
            sample_format_flags.insert(SampleFormatFlags::NON_INTERLEAVED);
        }
        ffi::PaStreamParameters {
            device: device.into(),
            channelCount: channel_count as raw::c_int,
            sampleFormat: sample_format_flags.bits(),
            suggestedLatency: suggested_latency,
            hostApiSpecificStreamInfo: ptr::null_mut(),
        }
    }
}

impl<I> Settings for InputSettings<I> {
    type Flow = Input<I>;
    fn into_flow_and_settings(self) -> (Self::Flow, f64, u32, Flags) {
        let InputSettings {
            params,
            sample_rate,
            frames_per_buffer,
            flags,
        } = self;
        let flow = Input { params: params };
        (flow, sample_rate, frames_per_buffer, flags)
    }
}

impl<O> Settings for OutputSettings<O> {
    type Flow = Output<O>;
    fn into_flow_and_settings(self) -> (Self::Flow, f64, u32, Flags) {
        let OutputSettings {
            params,
            sample_rate,
            frames_per_buffer,
            flags,
        } = self;
        let flow = Output { params: params };
        (flow, sample_rate, frames_per_buffer, flags)
    }
}

impl<I, O> Settings for DuplexSettings<I, O> {
    type Flow = Duplex<I, O>;
    fn into_flow_and_settings(self) -> (Self::Flow, f64, u32, Flags) {
        let DuplexSettings {
            in_params,
            out_params,
            sample_rate,
            frames_per_buffer,
            flags,
        } = self;
        let flow = Duplex {
            in_params: in_params,
            out_params: out_params,
        };
        (flow, sample_rate, frames_per_buffer, flags)
    }
}

impl Buffer {
    /// Construct a new **Buffer** for transferring audio on a stream with the given format.
    fn new<S>(frames_per_buffer: u32, channel_count: i32) -> Buffer {
        let sample_format_bytes = ::std::mem::size_of::<S>() as libc::size_t;
        let n_frames = frames_per_buffer as libc::size_t;
        let n_channels = channel_count as libc::size_t;
        let malloc_size = sample_format_bytes * n_frames * n_channels;
        Buffer {
            data: unsafe { libc::malloc(malloc_size) as *mut libc::c_void },
        }
    }

    /// Convert the **Buffer**'s data field into a slice with the given format.
    unsafe fn slice<'a, S>(&'a self, frames: u32, channels: i32) -> &'a [S] {
        let len = (frames * channels as u32) as usize;
        // TODO: At the moment, we assume this buffer is interleaved. We need to check whether
        // or not buffer is interleaved here. This should probably an extra type parameter
        // (along-side the Sample type param).
        std::slice::from_raw_parts(self.data as *const S, len)
    }

    /// Convert the **Buffer**'s data field into a mutable slice with the given format.
    unsafe fn slice_mut<'a, S>(&'a mut self, frames: u32, channels: i32) -> &'a mut [S] {
        let len = (frames * channels as u32) as usize;
        // TODO: At the moment, we assume this buffer is interleaved. We need to check whether
        // or not buffer is interleaved here. This should probably an extra type parameter
        // (along-side the Sample type param).
        std::slice::from_raw_parts_mut(self.data as *mut S, len)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { libc::free(self.data) }
    }
}

fn open_blocking_stream(
    in_params: Option<ffi::PaStreamParameters>,
    out_params: Option<ffi::PaStreamParameters>,
    sample_rate: f64,
    frames_per_buffer: u32,
    flags: Flags,
) -> Result<*mut raw::c_void, Error> {
    // The pointer to which PortAudio will attach the stream.
    let mut c_stream_ptr: *mut raw::c_void = ptr::null_mut();
    let in_c_params = in_params.map(|p| p.into());
    let out_c_params = out_params.map(|p| p.into());
    let in_c_params_ptr = in_c_params
        .as_ref()
        .map(|p| p as *const _)
        .unwrap_or(ptr::null());
    let out_c_params_ptr = out_c_params
        .as_ref()
        .map(|p| p as *const _)
        .unwrap_or(ptr::null());
    let c_flags = flags.bits();

    // open the PortAudio stream.
    unsafe {
        let error_code = ffi::Pa_OpenStream(
            &mut c_stream_ptr,
            in_c_params_ptr,
            out_c_params_ptr,
            sample_rate,
            frames_per_buffer as raw::c_ulong,
            c_flags,
            None,
            ptr::null_mut(),
        );
        let error = FromPrimitive::from_i32(error_code).unwrap();
        match error {
            Error::NoError => Ok(c_stream_ptr),
            err => Err(err),
        }
    }
}

fn open_non_blocking_stream(
    in_params: Option<ffi::PaStreamParameters>,
    out_params: Option<ffi::PaStreamParameters>,
    sample_rate: f64,
    frames_per_buffer: u32,
    flags: Flags,
    callback: &mut CallbackFnWrapper,
) -> Result<*mut raw::c_void, Error> {
    // The pointer to which PortAudio will attach the stream.
    let mut c_stream_ptr: *mut raw::c_void = ptr::null_mut();
    let in_c_params = in_params.map(|p| p.into());
    let out_c_params = out_params.map(|p| p.into());
    let in_c_params_ptr = in_c_params
        .as_ref()
        .map(|p| p as *const _)
        .unwrap_or(ptr::null());
    let out_c_params_ptr = out_c_params
        .as_ref()
        .map(|p| p as *const _)
        .unwrap_or(ptr::null());
    let c_flags = flags.bits();

    // Here we create an alias to the `Box` ptr held by the `CallbackFnWrapper`. We do this in
    // order to pass the pointer to the Pa_OpenStream function so that we may use it later as
    // `user_data` within the `stream_callback_proc`. The reason we don't pass ownership entirely
    // is so that we can still automatically clean up the data when the **Stream** (owner of the
    // `CallbackFnWrapper`) falls out of scope.
    // We know that this is safe because:
    // 1. We never call the aliased function ourselves.
    // 2. We always stop the stream and in turn stop the PortAudio lib from accessing the function
    //    before dropping it.
    // 3. The aliased function is a private member and can't be accessed outside this module.
    let user_data = {
        let callback_fn_ptr = callback as *mut CallbackFnWrapper;
        callback_fn_ptr as *mut raw::c_void
    };

    // open the PortAudio stream.
    unsafe {
        let error_code = ffi::Pa_OpenStream(
            &mut c_stream_ptr,
            in_c_params_ptr,
            out_c_params_ptr,
            sample_rate,
            frames_per_buffer as raw::c_ulong,
            c_flags,
            Some(stream_callback_proc),
            user_data,
        );
        let error = FromPrimitive::from_i32(error_code).unwrap();
        match error {
            Error::NoError => Ok(c_stream_ptr),
            err => Err(err),
        }
    }
}

impl<M, F> Stream<M, F> {
    fn new_unopened(mode: M, flow: F, life: std::sync::Arc<super::Life>) -> Self {
        Stream {
            pa_stream: ptr::null_mut(),
            mode: mode,
            flow: flow,
            port_audio_life: life,
        }
    }

    /// Closes an audio stream.
    ///
    /// If the audio stream is active it discards any pending buffers as if Stream::abort had been
    /// called.
    pub fn close(&mut self) -> Result<(), Error> {
        let error_code = unsafe { ffi::Pa_CloseStream(self.pa_stream) };
        let error = FromPrimitive::from_i32(error_code).unwrap();
        match error {
            Error::NoError => Ok(()),
            err => Err(err),
        }
    }

    /// Commences audio processing.
    pub fn start(&mut self) -> Result<(), Error> {
        let error_code = unsafe { ffi::Pa_StartStream(self.pa_stream) };
        let error = FromPrimitive::from_i32(error_code).unwrap();
        match error {
            0 => Ok(()),
            err => Err(FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Terminates audio processing.
    ///
    /// It waits until all pending audio buffers have been played before it returns.
    pub fn stop(&mut self) -> Result<(), Error> {
        let error_code = unsafe { ffi::Pa_StopStream(self.pa_stream) };
        let error = FromPrimitive::from_i32(error_code).unwrap();
        match error {
            0 => Ok(()),
            err => Err(FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Terminates audio processing immediately without waiting for pending buffers to complete.
    pub fn abort(&mut self) -> Result<(), Error> {
        let error_code = unsafe { ffi::Pa_AbortStream(self.pa_stream) };
        let error = FromPrimitive::from_i32(error_code).unwrap();
        match error {
            0 => Ok(()),
            err => Err(FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Determine whether the stream is stopped.
    ///
    /// A stream is considered to be stopped prior to a successful call to start_stream and after a
    /// successful call to stop_stream or abort_stream.
    ///
    /// If a stream callback returns a value other than Continue the stream is NOT considered to be
    /// stopped.
    ///
    /// Return `true` when the stream is stopped.
    ///
    /// Returns `false` when the stream is running.
    ///
    /// Returnes `Error` if an error is encountered.
    ///
    /// TODO: Clarify what errors can actually an occur.
    pub fn is_stopped(&self) -> Result<bool, Error> {
        let error_code = unsafe { ffi::Pa_IsStreamStopped(self.pa_stream) };
        match error_code {
            1 => Ok(true),
            0 => Ok(false),
            err => Err(FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Determine whether the stream is active.
    ///
    /// A stream is active after a successful call to `Stream::start`, until it becomes inactive
    /// either as a result of a call to `Stream::stop` or `Stream::abort`, or as a result of a
    /// return value other than `Continue` from the stream callback. In the latter case, the stream
    /// is considered inactive after the last buffer has finished playing.
    ///
    /// Returns `true` when the stream is active (ie playing or recording audio).
    ///
    /// Returns `false` when not playing.
    ///
    /// Returns an `Error` if an error is encountered.
    ///
    /// TODO: Clarify what errors can actually an occur.
    pub fn is_active(&self) -> Result<bool, Error> {
        let error_code = unsafe { ffi::Pa_IsStreamActive(self.pa_stream) };
        match error_code {
            0 => Ok(false),
            1 => Ok(true),
            err => Err(FromPrimitive::from_i32(err).unwrap()),
        }
    }

    /// Returns the current time in seconds for a stream according to the same clock used to
    /// generate callback CallbackTimeInfo timestamps.
    ///
    /// The time values are monotonically increasing and have unspecified origin.
    ///
    /// This returns valid time values for the entire life of the stream, from when the stream is
    /// opened until it is closed.
    ///
    /// Starting and stopping the stream does not affect the passage of time returned by this
    /// method.
    ///
    /// Returns the stream's current time in seconds, or 0 if an error occurred.
    pub fn time(&self) -> Time {
        unsafe { ffi::Pa_GetStreamTime(self.pa_stream) }
    }

    /// Retrieve a Info structure containing information about the stream.
    pub fn info(&self) -> Info {
        unsafe {
            let info = ffi::Pa_GetStreamInfo(self.pa_stream);
            Info::from(*info)
        }
    }

    /// This function is solely for use within the extension modules for interacting with PortAudio
    /// platform-specific extension APIs.
    pub fn unsafe_pa_stream(&self) -> *mut ffi::PaStream {
        self.pa_stream
    }
}

impl<F> Stream<Blocking<F::Buffer>, F>
where
    F: Flow,
{
    /// Open a new **Blocking** **Stream** with the given **Flow** and settings.
    pub fn open<S>(life: std::sync::Arc<super::Life>, settings: S) -> Result<Self, Error>
    where
        S: Settings<Flow = F>,
    {
        let (flow, sample_rate, frames_per_buffer, flags) = settings.into_flow_and_settings();
        let buffer = flow.new_buffer(frames_per_buffer);
        let blocking = Blocking { buffer: buffer };
        let (in_params, out_params) = flow.params_both_directions();
        let mut stream = Stream::new_unopened(blocking, flow, life);
        open_blocking_stream(in_params, out_params, sample_rate, frames_per_buffer, flags).map(
            |pa_stream| {
                stream.pa_stream = pa_stream;
                stream
            },
        )
    }
}

impl<F> Stream<Blocking<F::Buffer>, F>
where
    F: Flow + Reader,
{
    /// Retrieve the number of frames that can be read from the stream without waiting.
    ///
    /// Returns a Result with either:
    /// - An Ok variant with a `Available` enum describing either:
    ///     - The number of frames available to be read from the stream (without blocking or busy
    ///       waiting) or
    ///     - Flags indicating whether or not there has been input overflow or output underflow.
    /// - An Err variant in the case PortAudio is not initialized or some error is encountered.
    ///
    /// See the blocking.rs example for a usage example.
    pub fn read_available(&self) -> Result<Available, Error> {
        match unsafe { ffi::Pa_GetStreamReadAvailable(self.pa_stream) } {
            n if n >= 0 => Ok(Available::Frames(n)),
            n => match FromPrimitive::from_i64(n as i64) {
                Some(Error::InputOverflowed) => Ok(Available::InputOverflowed),
                Some(Error::OutputUnderflowed) => Ok(Available::OutputUnderflowed),
                Some(err) => Err(err),
                _ => panic!("Undefined error code: {:?}", n),
            },
        }
    }

    /// Read samples from an input stream.
    ///
    /// The function doesn't return until the entire buffer has been filled - this may involve
    /// waiting for the operating system to supply the data.
    ///
    /// # Arguments
    /// * frames - The number of frames in the buffer.
    ///
    /// Returns an interleaved slice containing the read audio data.
    ///
    /// Returns an `Error` if some error occurred.
    ///
    /// TODO: Research and document exactly what errors can occur.
    pub fn read<'b>(&'b self, frames: u32) -> Result<&'b [F::Sample], Error> {
        let buffer = F::readable_buffer(&self.mode);
        let err = unsafe {
            ffi::Pa_ReadStream(
                self.pa_stream,
                buffer.data as *mut raw::c_void,
                frames as raw::c_ulong,
            )
        };
        match err {
            0 => unsafe {
                let channel_count = Reader::channel_count(&self.flow);
                Ok(buffer.slice(frames, channel_count))
            },
            err => Err(FromPrimitive::from_i32(err).unwrap()),
        }
    }
}

impl<F> Stream<Blocking<F::Buffer>, F>
where
    F: Flow + Writer,
{
    /// Retrieve the number of frames that can be written to the stream without waiting.
    ///
    /// Returns a Result with either:
    /// - An Ok variant with a `Available` enum describing either:
    ///     - The number of frames available to be written to the stream (without blocking or busy
    ///       waiting) or
    ///     - Flags indicating whether or not there has been input overflow or output underflow.
    /// - An Err variant in the case PortAudio is not initialized or some error is encountered.
    ///
    /// See the blocking.rs example for a usage example.
    pub fn write_available(&self) -> Result<Available, Error> {
        match unsafe { ffi::Pa_GetStreamWriteAvailable(self.pa_stream) } {
            n if n >= 0 => Ok(Available::Frames(n)),
            n => match FromPrimitive::from_i64(n as i64) {
                Some(Error::InputOverflowed) => Ok(Available::InputOverflowed),
                Some(Error::OutputUnderflowed) => Ok(Available::OutputUnderflowed),
                Some(err) => Err(err),
                _ => panic!("Undefined error code: {:?}", n),
            },
        }
    }

    /// Write samples to an output stream.
    ///
    /// This function doesn't return until the entire buffer has been consumed
    ///
    /// - this may involve waiting for the operating system to consume the data.
    ///
    /// # Arguments
    /// * frames - The number of frames in the buffer.
    /// * write_fn - The buffer contains samples in the format specified by S.
    ///
    /// Returns Ok(()) on success and an Err(Error) variant on failure.
    pub fn write<WF>(&mut self, frames: u32, write_fn: WF) -> Result<(), Error>
    where
        WF: for<'b> FnOnce(&'b mut [F::Sample]),
    {
        let pa_stream = self.pa_stream;
        let channels = Writer::channel_count(&self.flow);
        let out_buffer = F::writable_buffer(&mut self.mode);
        let written_slice = {
            let slice = unsafe { out_buffer.slice_mut(frames, channels) };
            write_fn(slice);
            slice
        };
        let result = unsafe {
            let written_slice_ptr = written_slice.as_ptr() as *mut raw::c_void;
            ffi::Pa_WriteStream(pa_stream, written_slice_ptr, frames as raw::c_ulong)
        };
        match result {
            0 => Ok(()),
            err => Err(FromPrimitive::from_i32(err).unwrap()),
        }
    }
}

impl<F> Stream<NonBlocking, F> {
    /// Open a new **NonBlocking** **Stream** with the given **Flow** and settings.
    pub fn open<S, C>(
        life: std::sync::Arc<super::Life>,
        settings: S,
        mut callback: C,
    ) -> Result<Self, Error>
    where
        S: Settings<Flow = F>,
        F: Flow,
        C: FnMut(F::CallbackArgs) -> ffi::PaStreamCallbackResult + 'static,
    {
        let (flow, sample_rate, frames_per_buffer, flags) = settings.into_flow_and_settings();
        let (in_params, out_params) = flow.params_both_directions();
        let in_channels = in_params.map(|p| p.channelCount).unwrap_or(0);
        let out_channels = out_params.map(|p| p.channelCount).unwrap_or(0);

        let callback_wrapper_fn = move |input: *const raw::c_void,
                                        output: *mut raw::c_void,
                                        frame_count: raw::c_ulong,
                                        time_info: *const ffi::PaStreamCallbackTimeInfo,
                                        flags: ffi::PaStreamCallbackFlags|
              -> ffi::PaStreamCallbackResult {
            let args = F::new_callback_args(
                input,
                output,
                frame_count,
                time_info,
                flags,
                in_channels,
                out_channels,
            );
            callback(args)
        };

        let non_blocking = NonBlocking {
            // Here, we `Box` the wrapper so that we can collect the pointer from the callback.
            //
            // TODO: See if it is possible to pass a ptr to the callback_fn itself instead of
            // requiring the wrapper at all. It seems like DST will be a problem here though.
            callback: Box::new(CallbackFnWrapper {
                // Here we `Box` the callback fn as we can't handle generic types in the c callback
                // function.
                f: Box::new(callback_wrapper_fn),
            }),
        };

        let mut stream = Stream::new_unopened(non_blocking, flow, life);
        open_non_blocking_stream(
            in_params,
            out_params,
            sample_rate,
            frames_per_buffer,
            flags,
            &mut stream.mode.callback,
        )
        .map(|pa_stream| {
            stream.pa_stream = pa_stream;
            stream
        })
    }

    /// Retrieve CPU usage information for the specified stream.
    ///
    /// The "CPU Load" is a fraction of total CPU time consumed by a callback stream's audio
    /// processing routines including, but not limited to the client supplied stream callback.
    pub fn cpu_load(&self) -> f64 {
        unsafe { ffi::Pa_GetStreamCpuLoad(self.pa_stream) }
    }
}

impl<M, F> Drop for Stream<M, F> {
    fn drop(&mut self) {
        self.stop().ok();
        self.close().ok();
    }
}

/// A callback procedure to be used by portaudio in the case that a user_callback has been given
/// upon opening the stream (`Stream::open`).
extern "C" fn stream_callback_proc(
    input: *const raw::c_void,
    output: *mut raw::c_void,
    frame_count: raw::c_ulong,
    time_info: *const ffi::PaStreamCallbackTimeInfo,
    flags: ffi::PaStreamCallbackFlags,
    user_callback_ptr: *mut raw::c_void,
) -> ffi::PaStreamCallbackResult {
    let callback = user_callback_ptr as *mut CallbackFnWrapper;
    unsafe { ((*callback).f)(input, output, frame_count, time_info, flags) }
}
