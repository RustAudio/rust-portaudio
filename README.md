rust-portaudio [![Build Status](https://travis-ci.org/jeremyletang/rust-portaudio.png?branch=master)](https://travis-ci.org/jeremyletang/rust-portaudio) [![Crates.io](https://img.shields.io/crates/v/portaudio.svg)](https://crates.io/crates/portaudio) [![Crates.io](https://img.shields.io/crates/l/portaudio.svg)](https://github.com/jeremyletang/rust-portaudio/blob/master/LICENSE)
==============

[**PortAudio**](http://www.portaudio.com/) bindings for Rust.

PortAudio is a free, cross-platform, open-source, audio I/O library. These are the bindings and wrappers for Rust.

**rust-portaudio** is still under development, so there may be bugs - please feel free to add an issue or even better, submit a PR!

To use **rust-portaudio** in your own project, add it to your Cargo.toml dependencies like so:

```toml
[dependencies]
portaudio = "X.Y.Z"
```


# Installation

**rust-portaudio** will try to detect portaudio on your system and, failing that (or if given the `PORTAUDIO_ONLY_STATIC` environment variable on the build process), will download and build portaudio statically. If this fails please let us know! In the mean-time, you can manually [download and install PortAudio](http://www.portaudio.com/download.html) yourself.

**rust-portaudio** is built using cargo, so just type `cargo build` at the root of the **rust-portaudio** repository.

You can build the tests and examples with `cargo test`, and the documentation with `cargo doc`.

