#![doc = include_str!("../README.md")]

use std::collections::HashMap;

/// Initializes the dictionary in memory, this will make the first subsequent call faster (optional).
pub fn init() {
    #[cfg(feature = "moby")]
    thesaurus_moby::init();

    #[cfg(feature = "wordnet")]
    thesaurus_wordnet::init();
}

/// Returns the dictionary from memory.
pub fn dict() -> HashMap<String, Vec<String>> {
    let mut dict = HashMap::new();

    #[cfg(feature = "moby")]
    dict.extend(thesaurus_moby::dict());

    #[cfg(feature = "wordnet")]
    dict.extend(thesaurus_wordnet::dict());

    dict
}

/// Retrieve synonyms for a word.
pub fn synonyms(word: impl AsRef<str>) -> Vec<String> {
    let mut synonyms = Vec::new();

    let word = word.as_ref().to_lowercase();

    #[cfg(feature = "moby")]
    synonyms.extend_from_slice(thesaurus_moby::synonyms(&word).as_slice());

    #[cfg(feature = "wordnet")]
    synonyms.extend_from_slice(thesaurus_wordnet::synonyms(&word).as_slice());

    synonyms
}
