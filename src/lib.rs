use serde_json::Value;
use thiserror::Error;
use std::collections::HashSet;

pub struct Word {
    pub word: String,
    pub synonyms: Vec<String>,
}

#[derive(Error, Debug)]
pub enum ThesaurusError {
    #[error("Json Error")]
    JsonError(String),
    #[error("Unknown Word")]
    UnknownWord,
}

pub fn synonym(word: &str) -> std::result::Result<Word, ThesaurusError> {
    let dict_jsonl: String = include_str!("en_thesaurus.jsonl").to_string();

    let mut synonyms: Vec<String> = Vec::new();

    for json in dict_jsonl.lines() {
        let parsed_json: Value = match serde_json::from_str(&json) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(ThesaurusError::JsonError(format!("Couldn't parse json: {}", error)));
            }
        };

        let json_word = match &parsed_json["word"] {
            Value::String(json_word) => json_word,
            _ => {
                return Err(ThesaurusError::JsonError("Unable to find word in parsed JSON".to_string()));
            }
        };

        if json_word == word {
            let mut synonym_hashset: HashSet<String> = HashSet::new();

            for synonym in synonyms.iter() {
                synonym_hashset.insert(synonym.clone());
            }

            let json_synonyms = match &parsed_json["synonyms"] {
                Value::Array(json_synonyms) => json_synonyms,
                _ => {
                    return Err(ThesaurusError::JsonError("Unable to find synonyms in parsed JSON".to_string()));
                }
            };

            for synonym in json_synonyms.iter() {
                let synonym_result = match synonym {
                    Value::String(synonym_value) => synonym_value,
                    _ => {
                        return Err(ThesaurusError::JsonError("Unable to find synonym values in parsed JSON".to_string()));
                    }
                };

                if !synonym_hashset.contains(synonym_result) {
                    synonyms.push(synonym_result.clone());
                }
            };
        };
    };

    if synonyms.len() == 0 {
        return Err(ThesaurusError::UnknownWord);
    }

    let word_res = Word {
        word: word.to_string(),
        synonyms,
    };

    return Ok(word_res);
}
