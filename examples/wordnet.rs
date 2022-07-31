use thesaurus::wordnet;

fn main() {
    let word = "good";
    let results = wordnet::synonyms(word).unwrap();

    println!("Found {} results for {}:", results.len(), word);

    for word in results {
        println!("  {word}");
    }
}
