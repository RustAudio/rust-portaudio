#[link(name = "portaudio",
       vers = "0.0.1",
       package_id = "portaudio",
       author = "letang.jeremy@gmail.com",
       uuid = "57224177-873F-43C2-8F44-A41D501C1A63",
       url = "http://https://github.com/JeremyLetang/rust-portaudio")];

#[comment = "Portaudio binding for Rust"];
// #[license = "Zlib/png"];
#[crate_type = "lib"];

#[feature(globs, managed_boxes)];
#[warn(missing_doc)];

extern mod extra;


#[cfg(target_os="macos")]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod c_library {
    #[link_args="-lportaudio"]
    extern {}
}

pub mod types;
pub mod pa;

#[doc(hidden)]
pub mod user_traits;
#[doc(hidden)]
#[cfg(target_os="macos")]
pub mod mac_core;
//pub mod asio;
#[doc(hidden)]
pub mod ffi;
