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

#[cfg(not(target_os = "windows"))]
const PORTAUDIO_LIBRARY: &'static str = "libportaudio.a";
#[cfg(target_os = "windows")]
const PORTAUDIO_LIBRARY: &'static str = "portaudio.lib";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-env-changed=PORTAUDIO_ONLY_STATIC");
    if env::var("PORTAUDIO_ONLY_STATIC").is_err() {
        // If pkg-config finds a library on the system, we are done
        if pkg_config::Config::new()
            .atleast_version("19")
            .find("portaudio-2.0")
            .is_ok()
        {
            return;
        }
    }

    build();
}

fn build() {
    // retrieve cargo deps out dir
    let out_dir_str = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    let static_lib = out_dir.join("lib/").join(PORTAUDIO_LIBRARY);
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
        Err(e) => panic!("{}", e),
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
    use std::path::Path;
    use std::process::Command;

    use std::env;

    use super::{err_to_panic, run};

    pub const PORTAUDIO_URL: &'static str =
        "http://www.portaudio.com/archives/pa_stable_v19_20140130.tgz";
    pub const PORTAUDIO_TAR: &'static str = "pa_stable_v19_20140130.tgz";
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
        run(Command::new("./configure")
            .args(&["--disable-shared", "--enable-static"]) // Only build static lib
            .args(&["--prefix", out_dir.to_str().unwrap()]) // Install on the outdir
            .arg("--with-pic")); // Build position-independent code (required by Rust)

        // then make
        run(&mut Command::new("make"));

        // "install" on the outdir
        run(Command::new("make").arg("install"));

        // return to rust-portaudio root
        err_to_panic(env::set_current_dir(".."));

        // cleaning portaudio sources
        run(Command::new("rm")
            .arg("-rf")
            .args(&[PORTAUDIO_TAR, PORTAUDIO_FOLDER]));
    }

    pub fn print_libs(out_dir: &Path) {
        let out_str = out_dir.to_str().unwrap();
        println!(
            "cargo:rustc-flags=-L native={}/lib -l static=portaudio",
            out_str
        );
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::unix_platform;
    use pkg_config;
    use std::path::Path;
    use std::process::Command;

    use super::{err_to_panic, run};

    pub fn download() {
        run(Command::new("wget").arg(unix_platform::PORTAUDIO_URL));
    }

    pub fn build(out_dir: &Path) {
        unix_platform::build(out_dir);
    }

    pub fn print_libs(out_dir: &Path) {
        let portaudio_pc_file = out_dir.join("lib/pkgconfig/portaudio-2.0.pc");
        let portaudio_pc_file = portaudio_pc_file.to_str().unwrap();

        err_to_panic(
            pkg_config::Config::new()
                .statik(true)
                .find(portaudio_pc_file),
        );
    }
}

#[cfg(windows)]
mod platform {
    use std::{env, fs};
    use std::path::Path;
    use std::process::Command;

    use super::{err_to_panic, run};

    const PORTAUDIO_REPOSITORY: &'static str = "https://git.assembla.com/portaudio.git";
    const PORTAUDIO_STABLE_BRANCH: &'static str = "pa_stable_v190600_20161030";
    const PORTAUDIO_FOLDER: &'static str = "portaudio";
    const STEINBERG_URL: &'static str = "http://www.steinberg.net/sdk_downloads/";
    const ASIOSDK_ZIP: &'static str = "asiosdk2.3.zip";
    const ASIOSDK_EXPANDED_FOLDER: &'static str = "asiosdk2.3";
    const ASIOSDK_EXPANDED_ENTITY: &'static str = "asiosdk2.3\\ASIOSDK2.3";
    const ASIOSDK_SPECIFIED_LOCATION: &'static str = "portaudio\\src\\hostapi\\asio\\ASIOSDK";
    const PORTAUDIO_BUILD_DIR: &'static str = "portaudio\\build\\msvc";
    const PORTAUDIO_SOLUTION: &'static str = "portaudio.sln";
    const PORTAUDIO_PROJECT: &'static str = "portaudio.vcxproj";
    const MSBUILD_BUILD_CONFIG: &'static str =
        "/p:Configuration=Release;Platform=x64;ConfigurationType=StaticLibrary";
    const MSBUILD_TARGET_DIRECTORY: &'static str = "x64\\Release";

    /// Gets the source of portaudio from git repository.
    /// Windows users use different tar expanding tools, but Git is uniformly consistent.
    fn download_portaudio() {
        // If the source code from the previous build remains, delete it.
        if Path::new(PORTAUDIO_FOLDER).exists() {
            err_to_panic(force_remove::force_remove_dir_all(PORTAUDIO_FOLDER));
        }
        run(Command::new("git").args(&["clone", PORTAUDIO_REPOSITORY]));
        // Checkouts the latest stable branch
        err_to_panic(env::set_current_dir(PORTAUDIO_FOLDER));
        run(Command::new("git").args(&["checkout", PORTAUDIO_STABLE_BRANCH]));
        // Backs to the project dir
        err_to_panic(env::set_current_dir(".."));
    }

    /// Gets the ASIO SDK from Steinberg's web site. Places the SDK in the specified location.
    fn download_asiosdk() {
        // Windows' standard wget "bitsadmin" requires the full path to the installation location.
        let asiosdk_url = &format!("{}{}", STEINBERG_URL, ASIOSDK_ZIP);
        let path = env::current_dir().unwrap();
        let zip_fullpath = &format!("{}\\{}", path.display(), ASIOSDK_ZIP);
        run(Command::new("bitsadmin").args(&["/TRANSFER", "htmlget", asiosdk_url, zip_fullpath]));

        // unzip
        run(Command::new("powershell").args(&["expand-archive", ASIOSDK_ZIP]));

        // Places the SDK in the specified location, and removes a zip and an empty directory.
        err_to_panic(fs::rename(
            ASIOSDK_EXPANDED_ENTITY,
            ASIOSDK_SPECIFIED_LOCATION,
        ));
        err_to_panic(fs::remove_file(ASIOSDK_ZIP));
        err_to_panic(fs::remove_dir(ASIOSDK_EXPANDED_FOLDER));
    }

    pub fn download() {
        download_portaudio();
        download_asiosdk();
    }

    pub fn build(out_dir: &Path) {
        // Backups the current dir
        let path = env::current_dir().unwrap();

        // Builds portaudio!
        err_to_panic(env::set_current_dir(PORTAUDIO_BUILD_DIR));
        run(Command::new("devenv").args(&[PORTAUDIO_SOLUTION, "/Upgrade"]));
        run(Command::new("msbuild").args(&[PORTAUDIO_PROJECT, MSBUILD_BUILD_CONFIG]));

        // Places the library to the target.
        let prefix = &format!("{}\\lib", out_dir.display());
        err_to_panic(fs::create_dir_all(prefix));
        let current_portoudio_path =
            &format!("{}\\{}", MSBUILD_TARGET_DIRECTORY, super::PORTAUDIO_LIBRARY);
        let portaudio_target_path =
            &format!("{}\\lib\\{}", out_dir.display(), super::PORTAUDIO_LIBRARY);
        err_to_panic(fs::rename(current_portoudio_path, portaudio_target_path));

        // Backs to the current dir and removes the portaudio directory.
        err_to_panic(env::set_current_dir(path));
        err_to_panic(force_remove::force_remove_dir_all(PORTAUDIO_FOLDER));
    }

    pub fn print_libs(out_dir: &Path) {
        println!(
            "cargo:rustc-flags=-L native={}/lib -l static=portaudio",
            out_dir.display()
        );
    }
}
