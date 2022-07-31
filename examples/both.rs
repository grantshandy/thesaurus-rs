use thesaurus::{moby, wordnet};

fn main() {
    let word = "good";

    println!("moby has {} words indexed", moby::dict().len());
    println!("wordnet has {} words indexed", wordnet::dict().len());

    println!("---------");

    let ms = moby::synonyms(word).unwrap();
    println!("moby got {} synonyms for \"{word}\"", ms.len());

    let ws = wordnet::synonyms(word).unwrap();
    println!("wordnet got {} synonyms for \"{word}\"", ws.len());
}
