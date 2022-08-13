//! This crate is fairly low level, I reccomend you use the higher-level [`thesaurus`](https://crates.io/crates/thesaurus) crate.

use libflate::gzip::Decoder;
use std::io::Read;

pub const MOBY_DICT_COMPRESSED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/moby.gz"));

/// Uncompress the dictionary from memory.
pub fn uncompress() -> String {
    let mut dec = Decoder::new(MOBY_DICT_COMPRESSED)
        .expect("Failed to initialize runtime dictionary decoder");
    let mut output = Vec::new();
    dec.read_to_end(&mut output)
        .expect("Failed to decompress dictionary");

    String::from_utf8(output).unwrap()
}