use lazy_static::{initialize, lazy_static};
use libflate::gzip::Decoder;
use std::{collections::HashMap, io::Read, str};

pub const MOBY_DICT_COMPRESSED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/moby.gz"));

lazy_static! {
    static ref MOBY_DICT: HashMap<String, Vec<String>> = {
        let mut dec = Decoder::new(MOBY_DICT_COMPRESSED)
            .expect("Failed to initialize runtime dictionary decoder");
        let mut output = Vec::new();
        dec.read_to_end(&mut output)
            .expect("Failed to decompress dictionary");

        let uncompressed_dictionary = String::from_utf8(output).unwrap();

        let mut dict: HashMap<String, Vec<String>> = HashMap::new();

        for line in uncompressed_dictionary.lines() {
            let mut line: Vec<String> = line.split('|').map(|x| x.to_string()).collect();

            let name = line.remove(0);

            if let Some(x) = dict.get_mut(&name) {
                x.extend_from_slice(&line);
                x.dedup();
            } else {
                dict.insert(name, line);
            }
        }

        dict
    };
}

/// Initializes the dictionary in memory, this will make the first subsequent call faster (optional).
pub fn init() {
    initialize(&MOBY_DICT);
}

/// Returns the dictionary from memory.
pub fn dict() -> HashMap<String, Vec<String>> {
    MOBY_DICT.to_owned()
}

/// Retrieve synonyms for a word.
pub fn synonyms(word: impl AsRef<str>) -> Vec<String> {
    MOBY_DICT
        .get(&word.as_ref().to_lowercase())
        .map(|x| x.clone())
        .unwrap_or_default()
}
