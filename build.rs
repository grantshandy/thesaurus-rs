use std::{fs, io::Write};
use libflate::gzip::Encoder;

// this is the obscure 'formfeed' char, not found in the dictionary. 
// It's like a newline.
const DELIMITER: char = 0x0C as char;

fn main() {
  // Tell Cargo that if the dictionary file changes, to rerun this build script.
  println!("cargo:rerun-if-changed=src/dictionary.json");

  // load the dictionary in, sort alphabetically by word
  let dict = fs::read_to_string("./src/en_thesaurus.jsonl").expect("Failed to read `en_thesaurus.jsonl` file");

  // for some reason running `gzip -9 --keep ./src/dictionary.ir` (on the ir) gives better compression by like, 600kb,
  // - if you are a good person you will figure out how to correct the compressor config / algorithm used here
  let compressed = {
    let mut encoder = Encoder::new(Vec::new()).expect("Failed to create compressor");
    encoder.write_all(dict.as_bytes()).expect("Failed to compress dictionary");
    encoder.finish().into_result().expect("Failed to compress dictionary")
  };

  let target_dir = std::env::var("OUT_DIR").unwrap();

  fs::write(format!("{}/{}", target_dir, "en_thesaurus.gz"), compressed).expect(&format!("Failed to write compressed dictionary to `{}/en_thesaurus.gz`", target_dir));
}