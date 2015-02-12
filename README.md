rust-portaudio [![Build Status](https://travis-ci.org/jeremyletang/rust-portaudio.png?branch=master)](https://travis-ci.org/jeremyletang/rust-portaudio)
==============

__Portaudio__ bindings for Rust

PortAudio is a free, cross-platform, open-source, audio I/O library.
These are the bindings and wrappers for Rust.

PortAudio website : http://portaudio.com

Rust-PortAudio use the same license than PortAudio : the MIT license.

Rust-PortAudio is heavily in development, so there is many bugs.

Only the blocking API work for the moment.


# Installation

__rust-portaudio__ will try to detect portaudio on your system and, failing that (or if given the `PORTAUDIO_ONLY_STATIC` environment variable on the build process), will download and build portaudio statically.

__rust-portaudio__ is built using cargo, so just type `cargo build` at the root of the __rust-portaudio__ repository.

You can build the tests and examples with `cargo test`, and the documentation with `cargo doc`.
