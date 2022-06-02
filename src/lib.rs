#![doc = include_str!("../README.md")]

use lazy_static::{initialize, lazy_static};
use libflate::gzip::Decoder;
use std::{collections::BTreeMap, io::Read, str};

static COMPRESSED_DICTIONARY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/thesaurus.gz"));

lazy_static! {
    static ref DICTIONARY: BTreeMap<String, Vec<String>> = {
        let mut dec = Decoder::new(COMPRESSED_DICTIONARY)
            .expect("Failed to initialize runtime dictionary decoder");
        let mut output = Vec::new();
        dec.read_to_end(&mut output)
            .expect("Failed to decompress dictionary");

        let uncompressed_dictionary = String::from_utf8(output).unwrap();

        let mut dict: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for line in uncompressed_dictionary.lines() {
            let mut line = line.split(',');
            dict.insert(
                line.nth(0).unwrap().into(),
                line.skip(1).map(|x| x.into()).collect(),
            );
        }

        dict
    };
}

pub fn init() {
    initialize(&DICTIONARY);
}

pub fn synonym<T: AsRef<str>>(word: T) -> Option<Vec<String>> {
    DICTIONARY.get(&word.as_ref().to_lowercase()).map(|x| x.clone())
}
