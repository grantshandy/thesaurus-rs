fn main() {
    let word = "good";
    let results = thesaurus::synonym(word).unwrap();

    println!("Found {} results for {}:", results.len(), word);

    for word in results {
        println!("  {word}");
    }
}