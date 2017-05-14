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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

extern crate pkg_config;

use std::path::Path;
use std::env;
use std::fmt::Display;

#[cfg(all(unix, not(target_os = "linux")))]
use unix_platform as platform;

fn main() {
    if env::var("PORTAUDIO_ONLY_STATIC").is_err() {
        // If pkg-config finds a library on the system, we are done
        if pkg_config::Config::new().atleast_version("19").find("portaudio-2.0").is_ok() {
            return;
        }
    }

    build();
}

fn build() {
    // retrieve cargo deps out dir
    let out_dir_str = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    let static_lib = out_dir.join("lib/libportaudio.a");
    if let Err(_) = ::std::fs::metadata(static_lib) {
        platform::download();
        platform::build(out_dir);
    }

    platform::print_libs(out_dir);
}

// Similar to unwrap, but panics on just the error value
#[allow(dead_code)]
fn err_to_panic<T, E: Display>(result: Result<T, E>) -> T {
    match result {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    }
}

#[allow(dead_code)]
mod unix_platform {
    use std::process::Command;
    use std::path::Path;

    use std::env;

    use super::err_to_panic;

    pub const PORTAUDIO_URL: &'static str = "http://www.portaudio.com/archives/pa_stable_v19_20140130.tgz";
    pub const PORTAUDIO_TAR: &'static str = "pa_stable_v19_20140130.tgz";
    pub const PORTAUDIO_FOLDER: &'static str = "portaudio";

    pub fn download() {
        err_to_panic(Command::new("curl").arg(PORTAUDIO_URL).arg("-O").output());
    }

    pub fn build(out_dir: &Path) {
        // untar portaudio sources
        err_to_panic(Command::new("tar").arg("xvf").arg(PORTAUDIO_TAR).output());

        // change dir to the portaudio folder
        err_to_panic(env::set_current_dir(PORTAUDIO_FOLDER));

        // run portaudio autoconf
        err_to_panic(Command::new("./configure")
            .args(&["--disable-shared", "--enable-static"]) // Only build static lib
            .args(&["--prefix", out_dir.to_str().unwrap()]) // Install on the outdir
            .arg("--with-pic") // Build position-independent code (required by Rust)
            .output());

        // then make
        err_to_panic(Command::new("make").output());

        // "install" on the outdir
        err_to_panic(Command::new("make").arg("install").output());

        // return to rust-portaudio root
        err_to_panic(env::set_current_dir(".."));

        // cleaning portaudio sources
        err_to_panic(Command::new("rm").arg("-rf")
            .args(&[PORTAUDIO_TAR, PORTAUDIO_FOLDER]).output());
    }

    pub fn print_libs(out_dir: &Path) {
        let out_str = out_dir.to_str().unwrap();
        println!("cargo:rustc-flags=-L native={}/lib -l static=portaudio", out_str);
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use pkg_config;
    use std::process::Command;
    use super::unix_platform;
    use std::path::Path;

    use super::err_to_panic;

    pub fn download() {
        err_to_panic(Command::new("wget").arg(unix_platform::PORTAUDIO_URL).output());
    }

    pub fn build(out_dir: &Path) {
        unix_platform::build(out_dir);
    }

    pub fn print_libs(out_dir: &Path) {
        let portaudio_pc_file = out_dir.join("lib/pkgconfig/portaudio-2.0.pc");
        let portaudio_pc_file = portaudio_pc_file.to_str().unwrap();

        err_to_panic(pkg_config::Config::new().statik(true).find(portaudio_pc_file));
    }
}

#[cfg(windows)]
mod platform {
    use std;
    use std::path::Path;
    use std::process::Command;

    extern crate cmake;

    pub fn download() {
        let mut command = Command::new("cmake");

        command.arg("-P");
        command.arg("download.cmake");

        match command.status() {
            Ok(status) =>
                if !status.success() {
                    panic!("Failed to execute command: {:?}", command)
                },
            Err(error) =>
                panic!("Failed to execute command: {:?}\n{}", command, error)
        }
    }

    pub fn build(out_dir: &Path) {
        let source_path = out_dir.join("portaudio");

        // Note: the 'PA_WDMKS_NO_KSGUID_LIB' preprocessor definition is a
        // workaround for an issue which is fixed in the newer versions. See
        // https://app.assembla.com/spaces/portaudio/subversion/commits/1944
        cmake::Config::new(source_path)
            .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_DEBUG", out_dir)
            .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELEASE", out_dir)
            .cflag("-DPA_WDMKS_NO_KSGUID_LIB")
            .out_dir(out_dir)
            .build_target("portaudio_static")
            .build();

        std::fs::rename(
            out_dir.join(platform_specific_library_name()),
            out_dir.join("portaudio.lib")).unwrap();
    }

    pub fn print_libs(out_dir: &Path) {
        println!(
            "cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
    }

    #[cfg(target_arch = "x86")]
    fn platform_specific_library_name() -> &'static str {
        "portaudio_static_x86.lib"
    }

    #[cfg(target_arch = "x86_64")]
    fn platform_specific_library_name() -> &'static str {
        "portaudio_static_x64.lib"
    }
}
