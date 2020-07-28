rust-portaudio [![Build Status](https://travis-ci.org/RustAudio/rust-portaudio.svg?branch=master)](https://travis-ci.org/RustAudio/rust-portaudio) [![Crates.io](https://img.shields.io/crates/v/portaudio.svg)](https://crates.io/crates/portaudio) [![Crates.io](https://img.shields.io/crates/l/portaudio.svg)](https://github.com/RustAudio/rust-portaudio/blob/master/LICENSE)
==============

[**PortAudio**](http://www.portaudio.com/) bindings and wrappers for Rust.

PortAudio is a free, cross-platform, open-source, audio I/O library.

**rust-portaudio** is still under development, so there may be bugs - please feel free to add an issue or even better, submit a PR!

To use **rust-portaudio** in your own project, add it to your Cargo.toml dependencies like so:

```toml
[dependencies]
portaudio = "X.Y.Z"
```


# Installation

**rust-portaudio** will try to detect portaudio on your system and, failing that (or if given the `PORTAUDIO_ONLY_STATIC` environment variable on the build process), will download and build portaudio statically. If this fails please let us know! In the mean-time, you can manually [download and install PortAudio](http://www.portaudio.com/download.html) yourself.

On Mac OS X, you may need to install manually `portaudio` and `pkg-config` (using [brew](http://brew.sh/), run `brew install portaudio` and `brew install pkg-config`)

**rust-portaudio** is built using cargo, so just type `cargo build` at the root of the **rust-portaudio** repository.

You can build the tests and examples with `cargo test`, and the documentation with `cargo doc`.

# Installation on windows-msvc

On the windows-msvc, **rust-portaudio-sys** uses the Visual Studio solution file to build portaudio.
So, the following programs must be put through the system `PATH`.

- **Git**: getting the source code from repository
- **devenv**: updating the solution file in the repository of portaudio
- **MSBuild**: building PortAudio statically

It is remark that cargo does not read the user-wise path, so you must add these programs to the system path.

