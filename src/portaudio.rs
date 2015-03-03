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

__PortAudio__ bindings for Rust

PortAudio provides a uniform application programming interface (API) across all
supported platforms.  You can think of the PortAudio library as a wrapper that
converts calls to the PortAudio API into calls to platform-specific native audio
APIs. Operating systems often offer more than one native audio API and some APIs
(such as JACK) may be available on multiple target operating systems.
PortAudio supports all the major native audio APIs on each supported platform.

# Installation

rust-portaudio's build script will check to see if you have already installed
PortAudio on your system. If not, it will attempt to automatically download and
install it for you. If this fails, please let us know by posting an issue at [our
github repository] (https://github.com/jeremyletang/rust-portaudio).

If you'd prefer to install it manually, you can download it directly from the website:
[PortAudio](http://www.portaudio.com/download.html)

# Usage

Add rust-portaudio to your project by adding the dependency to your Cargo.toml as follows:

[dependencies]
portaudio = "*"

*/

#![warn(missing_docs)]
#![allow(dead_code)]
#![feature(collections, core, libc, std_misc)]

extern crate libc;

#[cfg(any(target_os="macos", target_os="linux", target_os="win32"))]
mod c_library {
    #[link(name = "portaudio")]
    extern {}
}

pub mod pa;
pub mod ext;
mod ffi;
