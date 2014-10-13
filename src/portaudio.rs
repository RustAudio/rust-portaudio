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

/*!
# Rust-PortAudio

__Portaudio__ bindings for Rust

PortAudio provides a uniform application programming interface (API) across all
supported platforms.  You can think of the PortAudio library as a wrapper that
converts calls to the PortAudio API into calls to platform-specific native audio
APIs. Operating systems often offer more than one native audio API and some APIs
(such as JACK) may be available on multiple target operating systems.
PortAudio supports all the major native audio APIs on each supported platform.

# Installation

You must install on your computer the Portaudio libraries who is used for
the binding.

Portaudio is available with package management tools on Linux, or brew on Mac OS.

You can download it directly from the website :
[portaudio](http://www.portaudio.com/download.html)

Then clone the repo and build the library with the following command at the root
of the __rust-portaudio__ repository.

__rust-portaudio__ is build with the rustpkg tool :

```Shell
> rustpkg build portaudio
```

*/

#![crate_name = "portaudio"]

#![comment = "Portaudio binding for Rust"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(globs)]
#![warn(missing_doc)]
#![allow(dead_code)]

extern crate libc;

#[cfg(any(target_os="macos", target_os="linux", target_os="win32"))]
mod c_library {
    #[link(name = "portaudio")]
    extern {}
}

pub mod types;
pub mod pa;

//#[doc(hidden)]
//pub mod user_traits;
//#[doc(hidden)]
//#[cfg(target_os="macos")]
//pub mod mac_core;
//pub mod asio;
#[doc(hidden)]
mod ffi;
