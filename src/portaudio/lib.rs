#[link(name = "portaudio",
       vers = "0.0.1",
       author = "letang.jeremy@gmail.com",
       uuid = "57224177-873F-43C2-8F44-A41D501C1A63",
       url = "http://https://github.com/JeremyLetang/rust-portaudio")];

#[comment = "Portaudio binding for Rust"];
// #[license = "Zlib/png"];
#[crate_type = "lib"];


extern mod extra;


#[cfg(target_os="macos")]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod c_library {
    #[link_args="-lportaudio"]
    extern {}
}

pub mod types;
pub mod user_traits;
pub mod pa;

#[cfg(target_os="macos")]
pub mod mac_core;

//pub mod asio;

#[doc(hidden)]
pub mod ffi;
