use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let word: String = match args.get(1) {
        Some(word) => word.to_string(),
        None => {
            eprintln!("Must include a word as an argument");
            process::exit(1);
        }
    };

    let synonyms = thesaurus::synonyms(&word);
    let num_words = thesaurus::dict().len();

    cfg_if::cfg_if! {
        if #[cfg(all(feature = "moby", feature = "wordnet"))] {
            print!("both wordnet and moby have ");
        } else if #[cfg(feature = "moby")] {
            print!("moby has ");
        } else if #[cfg(feature = "wordnet")] {
            print!("wordnet has ");
        }
    }

    println!("{num_words} words indexed, and {} synonyms for \"{word}\"...", synonyms.len());
    println!("synonyms...");
    for x in &synonyms {
        println!("   {x}");
    }
}
