use thesaurus::{WordType, Thesaurus};

fn main() {
    match Thesaurus::synonym("good", None) {
        Ok(data) => {
            let mut synonyms = String::new();

            for synonym in data.synonyms.iter() {
                synonyms.push_str(&format!("\n    {} ({})", synonym.name, synonym.word_type));
            };

            println!("Word: {}\nSynonyms:{}", data.name, synonyms);
        },
        Err(error) => eprintln!("Error: {}", error),
    };
}
