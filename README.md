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

You must install on your computer the Portaudio libraries who is used for the binding.

Portaudio is available with package management tools on Linux, or brew on Mac OS.

You can download it directly from the website : [portaudio](http://www.portaudio.com/download.html)

Then clone the repo and build the library with the following command at the root of the __rust-portaudio__ repository.

__rust-portaudio__ is build using make, so just type `make` at the root of the __rust-portaudio__ repository, this command
build __rust-portaudio__, the examples, and the documentation.

You can build them separatly to with the dedicated commands:

```Shell
> make portaudio
> make test
> make doc
```