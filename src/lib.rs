#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

use std::collections::HashMap;

#[cfg(feature = "static")]
use lazy_static::{lazy_static, initialize};

#[cfg(feature = "static")]
lazy_static! {
    static ref DICT: HashMap<String, Vec<String>> = parse_dict();
}

/// Initialize a global dictionary so initialization isn't done on the first call to [`synonyms`] or [`dict`].
#[cfg(feature = "static")]
pub fn init() {
    initialize(&DICT);
}

/// Return the internal dictionary.
pub fn dict() -> HashMap<String, Vec<String>> {
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();

    #[cfg(feature = "static")]
    dict.extend(DICT.to_owned());

    // if we're not static...
    if dict.is_empty() {
        dict.extend(parse_dict());
    }

    dict
}

/// Return synonyms for a word.
pub fn synonyms(word: impl AsRef<str>) -> Vec<String> {
    let mut s = dict()
        .get(word.as_ref())
        .map(|x| x.clone())
        .unwrap_or_default();

    s.dedup();
    s.sort_by(|a, b| a.cmp(&b));

    s
}

fn parse_dict() -> HashMap<String, Vec<String>> {
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();

    #[cfg(feature = "wordnet")]
    for line in thesaurus_wordnet::uncompress().lines() {
        parse_dict_line(line, &mut dict);
    }

    #[cfg(feature = "moby")]
    for line in thesaurus_moby::uncompress().lines() {
        parse_dict_line(line, &mut dict);
    }

    dict
}

fn parse_dict_line(line: &str, dict: &mut HashMap<String, Vec<String>>) {
    let mut line: Vec<String> = line.split('|').map(|x| x.to_string().to_lowercase()).collect();

    let name = line.remove(0);

    if let Some(x) = dict.get_mut(&name) {
        x.extend_from_slice(&line);
        x.dedup();
    } else {
        dict.insert(name, line);
    }
}