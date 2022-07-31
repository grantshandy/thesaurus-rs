use thesaurus::moby;

fn main() {
    let word = "good";
    let results = moby::synonyms(word).unwrap();

    println!("Found {} results for {}:", results.len(), word);

    for word in results {
        println!("  {word}");
    }
}
