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

use std::io::process::Command;
use std::os;

const PORTAUDIO_URL: &'static str = "http://www.portaudio.com/archives/pa_stable_v19_20140130.tgz";
const PORTAUDIO_TAR: &'static str = "pa_stable_v19_20140130.tgz";
const PORTAUDIO_FOLDER: &'static str = "portaudio";
const PORTAUDIO_LIB_PATH: &'static str = "portaudio/lib/.libs/libportaudio.dylib";

fn main() {
    // retrieve cargo deps out dir
    let out_dir = os::getenv("OUT_DIR").unwrap();

    // get portaudio library sources
    match Command::new("wget").arg(PORTAUDIO_URL).output() {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    // untar portaudio sources
    match Command::new("tar").arg("xvf").arg(PORTAUDIO_TAR).output() {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    // change dir to the portaudio folder
    match os::change_dir(&from_str(PORTAUDIO_FOLDER).unwrap()) {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    // run portaudio autoconf
    match Command::new("./configure").output() {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    // then make
    match Command::new("make").output() {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    // return to rust-portaudio root
    match os::change_dir(&from_str("..").unwrap()) {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    // move portaudio library inside the OUT_DIR folder
    match Command::new("cp").arg(PORTAUDIO_LIB_PATH).arg(out_dir.clone()).output() {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    // cleaning portaudio sources
    match Command::new("rm").arg("-rf").arg(PORTAUDIO_TAR).arg(PORTAUDIO_FOLDER).output() {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    println!("cargo:rustc-flags=-L {} -l portaudio", out_dir);

}