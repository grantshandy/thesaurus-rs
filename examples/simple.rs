use thesaurus::{WordType, Thesaurus};

fn main() {
    match Thesaurus::synonym("good", Some(WordType::Noun)) {
        Ok(data) => {
            let mut synonyms = String::new();

            for synonym in data.words.iter() {
                synonyms.push_str(&format!("\n    {} ({})", synonym.name, synonym.word_type));
            };

            println!("Word: {}\nSynonyms:{}", data.name, synonyms);
        },
        Err(error) => eprintln!("Error: {}", error),
    };
}
