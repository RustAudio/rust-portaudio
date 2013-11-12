rust-portaudio [![Build Status](https://travis-ci.org/JeremyLetang/rust-portaudio.png?branch=master)](https://travis-ci.org/JeremyLetang/rust-portaudio)
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

__rust-portaudio__ is build with the rustpkg tool :

```Shell
> rustpkg build portaudio
```
