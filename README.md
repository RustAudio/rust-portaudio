rust-portaudio
==============

[![Crates.io](https://img.shields.io/crates/v/portaudio.svg)](https://crates.io/crates/portaudio)
[![docs.rs](https://docs.rs/portaudio/badge.svg)](https://docs.rs/portaudio)

[![License](https://img.shields.io/crates/l/portaudio.svg)](https://github.com/RustAudio/rust-portaudio/blob/master/LICENSE)
[![Build Status](https://travis-ci.org/RustAudio/rust-portaudio.svg?branch=master)](https://travis-ci.org/RustAudio/rust-portaudio)

[**PortAudio**](http://www.portaudio.com/) bindings and wrappers for Rust.

PortAudio is a free, cross-platform, open-source, audio I/O library.

**rust-portaudio** is in maintenance mode. Consider using
[CPAL](https://github.com/RustAudio/cpal) instead.


# Installation

**rust-portaudio** will try to detect portaudio on your system and, failing that
(or if given the `PORTAUDIO_ONLY_STATIC` environment variable on the build
process), will download and build portaudio statically. If this fails please let
us know! In the mean-time, you can manually [download and install
PortAudio](http://www.portaudio.com/download.html) yourself.

On Mac OS X, you may need to install manually `portaudio` and `pkg-config`
(using [brew](http://brew.sh/), run `brew install portaudio` and `brew install
pkg-config`)

**rust-portaudio** is built using cargo, so just type `cargo build` at the root
of the **rust-portaudio** repository.

You can build the tests and examples with `cargo test`, and the documentation
with `cargo doc`.
