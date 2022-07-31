use libflate::gzip::Encoder;
use std::{env, fs, io::Write};

const MOBY: &'static str = "./src/moby.txt";

fn main() {
    // Tell Cargo that if the dictionary files change, to rerun this build script.
    println!("cargo:rerun-if-changed={MOBY}");

    let thes = fs::read_to_string(MOBY)
        .expect(&format!("Failed to read \"{MOBY}\""))
        .replace(",", "|");

    let compressed = {
        let mut encoder = Encoder::new(Vec::new()).expect("Failed to create compressor");
        encoder
            .write_all(thes.as_bytes())
            .expect("Failed to compress dictionary");
        encoder
            .finish()
            .into_result()
            .expect("Failed to finish compressing dictionary")
    };

    let target_dir = env::var("OUT_DIR").unwrap();

    fs::write(format!("{}/{}", target_dir, "moby.gz"), compressed).expect(&format!(
        "Failed to write compressed dictionary to `{}/moby.gz`",
        target_dir
    ));
}
