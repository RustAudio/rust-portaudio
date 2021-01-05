extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut cc = cc::Build::new();

    let src_common = [
        "portaudio/src/common/pa_allocation.c",
        "portaudio/src/common/pa_converters.c",
        "portaudio/src/common/pa_cpuload.c",
        "portaudio/src/common/pa_debugprint.c",
        "portaudio/src/common/pa_dither.c",
        "portaudio/src/common/pa_front.c",
        "portaudio/src/common/pa_process.c",
        "portaudio/src/common/pa_ringbuffer.c",
        "portaudio/src/common/pa_stream.c",
        "portaudio/src/common/pa_trace.c",
    ];

    let src_win = [
        "portaudio/src/hostapi/wasapi/pa_win_wasapi.c",
        "portaudio/src/os/win/pa_win_hostapis.c",
        "portaudio/src/os/win/pa_win_util.c",
        "portaudio/src/os/win/pa_win_waveformat.c",
        "portaudio/src/os/win/pa_win_wdmks_utils.c",
        "portaudio/src/os/win/pa_win_coinitialize.c",
        "portaudio/src/os/win/pa_x86_plain_converters.c",
    ];

    let src_mac = [
        "portaudio/src/hostapi/coreaudio/pa_mac_core.c",
        "portaudio/src/hostapi/coreaudio/pa_mac_core_blocking.c",
        "portaudio/src/hostapi/coreaudio/pa_mac_core_utilities.c",
    ];

    let src_unix = [
        "portaudio/src/os/unix/pa_unix_hostapis.c",
        "portaudio/src/os/unix/pa_unix_util.c",
    ];

    let src_alsa = ["portaudio/src/hostapi/alsa/pa_linux_alsa"];

    cc.include("portaudio/include");
    cc.include("portaudio/src/common");

    for src in src_common.iter() {
        cc.file(src);
    }

    if target.contains("linux") {
        println!("cargo:rustc-link-lib=asound");
        for src in src_alsa.iter() {
            cc.file(src);
        }
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Carbon");
        for src in src_mac.iter() {
            cc.file(src);
        }
        for src in src_unix.iter() {
            cc.file(src);
        }
        cc.flag("-DPA_USE_COREAUDIO=1");
        cc.flag("-mmacosx-version-min=10.4");
        cc.flag("-std=c99");
    } else if target.contains("windows") {
        for src in src_win.iter() {
            cc.file(src);
        }
        cc.flag("-DPA_USE_WASAPI=1");
        cc.include("portaudio/src/os/win");
        println!("cargo:rustc-link-lib=ole32");
    }
    cc.compile("libportaudio.a");
}
