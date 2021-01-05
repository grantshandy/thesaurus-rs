extern crate serde_json;

use serde_json::Value;

pub fn synonym(word: &str) -> std::result::Result<(), String> {
    let dict_jsonl: String = include_str!("en_thesaurus.jsonl").to_string();

    for json in dict_jsonl.lines() {
        let parsed_json: Value = match serde_json::from_str(&json) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(format!("Couldn't parse json: {}", error));
            }
        };

        let json_word = match &parsed_json["word"] {
            Value::String(json_word) => json_word,
            _ => {
                return Err("Unable to find word in parsed JSON".to_string());
            }
        };

        if json_word == word {
            println!("found the word {}", json_word);
        };
    };
    Ok(())
}
