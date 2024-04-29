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

use std::env;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

#[cfg(all(unix, not(target_os = "linux")))]
use unix_platform as platform;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-env-changed=PORTAUDIO_ONLY_STATIC");
    println!("cargo:rerun-if-env-changed=PORTAUDIO_CONFIGURE_EXTRA_ARGS");
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

fn run(command: &mut Command) {
    let string = format!("{:?}", command);
    let status = err_to_panic(command.status());
    if !status.success() {
        panic!("`{}` did not execute successfully", string);
    }
}

#[allow(dead_code)]
mod unix_platform {
    use std::process::Command;
    use std::path::Path;

    use std::env;

    use super::{err_to_panic, run};

    pub const PORTAUDIO_URL: &'static str = "https://files.portaudio.com/archives/pa_stable_v190700_20210406.tgz";
    pub const PORTAUDIO_TAR: &'static str = "pa_stable_v190700_20210406.tgz";
    pub const PORTAUDIO_FOLDER: &'static str = "portaudio";

    pub fn download() {
        run(Command::new("curl").arg(PORTAUDIO_URL).arg("-O"));
    }

    pub fn build(out_dir: &Path) {
        // untar portaudio sources
        run(Command::new("tar").arg("xvf").arg(PORTAUDIO_TAR));

        // change dir to the portaudio folder
        err_to_panic(env::set_current_dir(PORTAUDIO_FOLDER));

        // run portaudio autoconf
        let mut cmd = Command::new("./configure");
        cmd
            .args(&["--disable-shared", "--enable-static"]) // Only build static lib
            .args(&["--prefix", out_dir.to_str().unwrap()]) // Install on the outdir
            .arg("--with-pic"); // Build position-independent code (required by Rust)
        if let Ok(extra_args) = env::var("PORTAUDIO_CONFIGURE_EXTRA_ARGS") {
            cmd.args(extra_args.split(" "));
        }
        run(&mut cmd);

        // then make
        run(&mut Command::new("make"));

        // "install" on the outdir
        run(Command::new("make").arg("install"));

        // return to rust-portaudio root
        err_to_panic(env::set_current_dir(".."));

        // cleaning portaudio sources
        run(Command::new("rm").arg("-rf")
            .args(&[PORTAUDIO_TAR, PORTAUDIO_FOLDER]));
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

    use super::{run, err_to_panic};

    pub fn download() {
        run(Command::new("wget").arg(unix_platform::PORTAUDIO_URL));
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
    use std::path::Path;

    const PORTAUDIO_DOWNLOAD_URL: &'static str = "http://www.portaudio.com";

    fn print_lib_url() {
        panic!("Don't know how to build portaudio on Windows yet. Sources and build instructions available at: {}", PORTAUDIO_DOWNLOAD_URL);
    }

    pub fn download() {
        print_lib_url();
    }

    pub fn build(_: &Path) {
        print_lib_url();
    }

    pub fn print_libs(_: &Path) {
        print_lib_url();
    }
}
