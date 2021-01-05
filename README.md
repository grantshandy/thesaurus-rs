# thesaurus-rs
A thesaurus library for Rust.

`thesaurus-rs` is extremely simple library for simple programs that don't need an internet connection. It relies on the JSON file from (zaibacu)[https://github.com/zaibacu/thesaurus].

It's simple to use:
```
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
```

Result:
```
Word: good
Synonyms:
    skilled
    skilful
    practiced
    skillful
    expert
    adept
    proficient
    sainted
    saving
    beatific
    right
    righteous
    angelical
    saintly
    good
    angelic

    etc...
```
