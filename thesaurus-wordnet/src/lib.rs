use lazy_static::{initialize, lazy_static};
use libflate::gzip::Decoder;
use std::{collections::HashMap, io::Read, str};

const WORDNET_DICT_COMPRESSED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/wordnet.gz"));

lazy_static! {
    static ref WORDNET_DICT: HashMap<String, Vec<String>> = {
        let mut dec = Decoder::new(WORDNET_DICT_COMPRESSED)
            .expect("Failed to initialize runtime dictionary decoder");
        let mut output = Vec::new();
        dec.read_to_end(&mut output)
            .expect("Failed to decompress dictionary");

        let uncompressed_dictionary = String::from_utf8(output).unwrap();

        let mut dict: HashMap<String, Vec<String>> = HashMap::new();

        for line in uncompressed_dictionary.lines() {
            let mut line: Vec<String> = line.split('|').map(|x| x.to_string()).collect();

            let name = line.remove(0);

            dict.insert(name, line);
        }

        dict
    };
}

/// Initializes the dictionary in memory, this will make all subsequent calls faster (optional).
pub fn init() {
    initialize(&WORDNET_DICT);
}

/// Returns the dictionary from memory.
pub fn dict() -> HashMap<String, Vec<String>> {
    WORDNET_DICT.to_owned()
}

/// Retrieve synonyms for a word.
pub fn synonyms(word: impl AsRef<str>) -> Option<Vec<String>> {
    WORDNET_DICT
        .get(&word.as_ref().to_lowercase())
        .map(|x| x.clone())
}
