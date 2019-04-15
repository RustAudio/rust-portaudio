//! Prints all Host APIs that are available on the system and that this instance of PortAudio can
//! support.

extern crate portaudio as pa;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), pa::Error> {
    let pa = pa::PortAudio::new()?;

    println!("Default Host API: {:?}", pa.default_host_api());
    println!("All Host APIs:");
    for host in pa.host_apis() {
        println!("{:#?}", host);
    }

    Ok(())
}
