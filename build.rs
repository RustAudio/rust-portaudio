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

#[allow(dead_code)]
mod unix_platform {
    use std::process::Command;
    use std::path::Path;

    use std::env;

    pub const PORTAUDIO_URL: &'static str = "http://www.portaudio.com/archives/pa_stable_v19_20140130.tgz";
    pub const PORTAUDIO_TAR: &'static str = "pa_stable_v19_20140130.tgz";
    pub const PORTAUDIO_FOLDER: &'static str = "portaudio";

    pub fn download() {
        match Command::new("curl").arg(PORTAUDIO_URL).arg("-O").output() {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }
    }

    pub fn build(out_dir: &Path) {
        // untar portaudio sources
        match Command::new("tar").arg("xvf").arg(PORTAUDIO_TAR).output() {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }

        // change dir to the portaudio folder
        match env::set_current_dir(PORTAUDIO_FOLDER) {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }

        // run portaudio autoconf
        Command::new("./configure")
            .args(&["--disable-shared", "--enable-static"]) // Only build static lib
            .args(&["--prefix", out_dir.to_str().unwrap()]) // Install on the outdir
            .arg("--with-pic") // Build position-independent code (required by Rust)
            .output()
            .unwrap();

        // then make
        match Command::new("make").output() {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }

        // "install" on the outdir
        match Command::new("make").arg("install").output() {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }

        // return to rust-portaudio root
        match env::set_current_dir("..") {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }

        // cleaning portaudio sources
        match Command::new("rm").arg("-rf").args(&[PORTAUDIO_TAR, PORTAUDIO_FOLDER]).output() {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }
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

    pub fn download() {
        match Command::new("wget").arg(unix_platform::PORTAUDIO_URL).output() {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }
    }

    pub fn build(out_dir: &Path) {
        unix_platform::build(out_dir);
    }

    pub fn print_libs(out_dir: &Path) {
        let portaudio_pc_file = out_dir.join("lib/pkgconfig/portaudio-2.0.pc");
        let portaudio_pc_file = portaudio_pc_file.to_str().unwrap();

        pkg_config::Config::new().statik(true).find(portaudio_pc_file).unwrap();
    }
}

#[cfg(windows)]
mod platform {
    use std::path::Path;

    pub fn download() {
        panic!("Don't know how to build portaudio on Windows yet!");
    }

    pub fn build(_: &Path) {
        panic!("Don't know how to build portaudio on Windows yet!");
    }

    pub fn print_libs(_: &Path) {
        panic!("Don't know how to build portaudio on Windows yet!");
    }
}
