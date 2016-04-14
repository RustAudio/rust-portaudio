//! Prints all Host APIs that are available on the system and that this instance of PortAudio can
//! support.

extern crate portaudio as pa;

fn main() {
    let pa = pa::PortAudio::new().unwrap();

    println!("Default Host API: {:?}", pa.default_host_api());
    println!("All Host APIs:");
    for host in pa.host_apis() {
        println!("{:#?}", host);
    }
}
