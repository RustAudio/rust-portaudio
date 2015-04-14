rust-portaudio [![Build Status](https://travis-ci.org/jeremyletang/rust-portaudio.png?branch=master)](https://travis-ci.org/jeremyletang/rust-portaudio)
==============

__Portaudio__ bindings for Rust.

PortAudio is a free, cross-platform, open-source, audio I/O library. These are the bindings and wrappers for Rust.

Visit the PortAudio website [here](http://www.portaudio.com/).

Rust-PortAudio use the same license as PortAudio : the MIT license.

Rust-PortAudio is still in development, so there may be bugs - please feel free to add an issue or even better, submit a PR!


# Using rust-portaudio.

To use rust-portaudio in your own project, add it to your Cargo.toml dependencies like so:

```
[dependencies]
portaudio = "*"
```


# Installation

__rust-portaudio__ will try to detect portaudio on your system and, failing that (or if given the `PORTAUDIO_ONLY_STATIC` environment variable on the build process), will download and build portaudio statically.

__rust-portaudio__ is built using cargo, so just type `cargo build` at the root of the __rust-portaudio__ repository.

You can build the tests and examples with `cargo test`, and the documentation with `cargo doc`.

