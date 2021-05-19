//! # thesaurus-rs
//! A thesaurus library for Rust.
//!
//! `thesaurus-rs` is extremely simple library for simple programs that need a thesaurus, but don't need an internet connection. It relies on the JSON file from [zaibacu](https://github.com/zaibacu/thesaurus).
//!
//! It's simple to use:
//! ```rust
//! use thesaurus::{WordType, Thesaurus};
//!
//! fn main() {
//!     match Thesaurus::synonym("good", Some(WordType::Noun)) {
//!         Ok(data) => {
//!             let mut synonyms = String::new();
//!
//!             for synonym in data.words.iter() {
//!                 synonyms.push_str(&format!("\n    {} ({})", synonym.name, synonym.word_type));
//!             };
//!
//!             println!("Word: {}\nSynonyms:{}", data.name, synonyms);
//!         },
//!         Err(error) => eprintln!("Error: {}", error),
//!     };
//! }
//! ```
//!
//! Result:
//! ```
//! Word: good
//! Synonyms:
//!     commodity (Noun)
//!     trade good (Noun)
//!     goodness (Noun)
//! ```

use serde_json::Value;
use futures::future::Future;

/// The main struct for the Thesaurus
pub struct Thesaurus {
    pub name: String,
    pub synonyms: Vec<Synonym>,
}

/// Words output by the Thesaurus
#[derive(Clone, PartialEq)]
pub struct Synonym {
    pub name: String,
    pub word_type: WordType,
}

/// Parts of speech
#[derive(Clone, PartialEq, Copy)]
pub enum WordType {
    Noun,
    Adjective,
    Adverb,
    Verb,
}

impl WordType {
    pub fn from<T: AsRef<str>>(word_type: T) -> Result<Self, String> {
        match word_type.as_ref() {
            "noun" => Ok(WordType::Noun),
            "adj" => Ok(WordType::Adjective),
            "adv" => Ok(WordType::Adverb),
            "verb" => Ok(WordType::Verb),
            &_ => Err(String::from(word_type.as_ref())),
        }
    }
}

impl std::fmt::Display for WordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordType::Noun => {
                write!(f, "Noun")
            }
            WordType::Adjective => {
                write!(f, "Adjective")
            }
            WordType::Adverb => {
                write!(f, "Adverb")
            }
            WordType::Verb => {
                write!(f, "Verb")
            }
        }
    }
}


impl Thesaurus {
    /// Gets a synonym, you can set a word and wether or not you want to specify your parts of speech you want returned.
    pub fn synonym<T: AsRef<str>>(word: T, word_type_request: Option<WordType>) -> Result<Thesaurus, Error> {
        let dict_jsonl: String = include_str!("en_thesaurus.jsonl").to_string();
        let mut lines: Vec<String> = Vec::new();
        
        for line in dict_jsonl.lines() {
            lines.push(line.to_string());
        };

        let synonyms = match Self::parse(lines, word.as_ref(), word_type_request) {
            Ok(data) => data,
            Err(error) => return Err(error), 
        };

        let thesaurus = Self {
            name: word.as_ref().to_string(),
            synonyms,
        };

        return Ok(thesaurus);
    }

    pub fn faster_synonym<T: AsRef<str>>(word: T, word_type_request: Option<WordType>) {
        // -> Result<Thesaurus, Error>
    }

    // parse a vec of lines and return a vec of synonyms.
    async fn parse<T: AsRef<str>>(dict_jsonl: Vec<String>, word: T, word_type_request: Option<WordType>) -> Future<Result<Vec<Synonym>, Error>> {
        let is_word_type_request: bool = match word_type_request {
            Some(_) => true,
            None => false,
        };
   
        let mut words: Vec<Synonym> = Vec::new();
    
        for json in dict_jsonl {
            let parsed_json: Value = match serde_json::from_str(&json) {
                Ok(parsed_json) => parsed_json,
                Err(error) => {
                    return Err(Error::Json(format!("Couldn't parse json: {}", error)));
                }
            };
    
            let json_word = match &parsed_json["word"] {
                Value::String(json_word) => json_word,
                _ => {
                    return Err(Error::Json("Unable to find word in parsed JSON".to_string()));
                }
            };
    
            if json_word == word.as_ref() {
                let json_synonyms = match &parsed_json["synonyms"] {
                    Value::Array(json_synonyms) => json_synonyms,
                    _ => return Err(Error::Json("Unable to find synonyms in parsed JSON".to_string())),
                };
    
                let json_word_type = match &parsed_json["pos"] {
                    Value::String(word_type) => word_type,
                    _ => return Err(Error::Json("Unable to find pos values in parsed JSON".to_string())),
                };
    
                for synonym_value in json_synonyms.iter() {
                    let synonym = match synonym_value {
                        Value::String(synonym_value) => synonym_value,
                        _ => return Err(Error::Json("Unable to find synonym values in parsed JSON".to_string())),
                    };

                    let word_type = match WordType::from(json_word_type.clone()) {
                        Ok(word_type) => word_type,
                        Err(error) => return Err(Error::UnexpectedType(error)),
                    };
    
                    let word_struct = Synonym {
                        name: synonym.clone(),
                        word_type: word_type,
                    };
                    
                    if is_word_type_request {
                        if word_struct.clone().word_type == word_type_request.unwrap() {
                            if !words.contains(&word_struct.clone()) {
                                words.push(word_struct.clone());
                            };
                        } else {
                            continue;
                        };
                    };

                    if !words.contains(&word_struct.clone()) {
                        words.push(word_struct.clone());
                    };
                };
            };
        };
    
        if words.len() == 0 {
            return Err(Error::Unknown);
        }
    
        return Ok(words);
    }
}

/// Errors output by the Thesaurus
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnexpectedType(String),
    Json(String),
    Unknown,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedType(word_type) => {
                write!(f, "Unexpected type in JSON document: {}", word_type)
            }
            Error::Json(error) => {
                write!(f, "{}", error.to_string())
            }
            Error::Unknown => {
                write!(f, "Unknown Word")
            }
        }
    }
}