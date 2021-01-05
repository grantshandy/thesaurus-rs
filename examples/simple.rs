use thesaurus::synonym;

fn main() {
    match synonym("good") {
        Ok(data) => {
            let mut synonyms = String::new();

            for synonym in data.synonyms.iter() {
                synonyms.push_str(&format!("    {}\n", synonym))
            };

            println!("Word: {}\nSynonyms:\n{}", data.word, synonyms);
        },
        Err(error) => eprintln!("Error: {}", error),
    };
}
