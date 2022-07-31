# thesaurus-rs
[![Crates.io](https://img.shields.io/crates/v/thesaurus.svg)](https://crates.io/crates/thesaurus)
[![Crates.io](https://img.shields.io/crates/d/thesaurus)](https://crates.io/crates/thesaurus)
[![API](https://docs.rs/thesaurus/badge.svg)](https://docs.rs/thesaurus)

The offline thesaurus library for Rust that can use both [wordnet](https://wordnet.princeton.edu/) and [moby](https://en.wikipedia.org/wiki/Moby_Project) backends.

Add to `Cargo.toml` for wordnet:
```toml
thesaurus = { version = "0.4.0", features = ["wordnet"] }
```

Add to `Cargo.toml` for moby (wordnet is on by default):
```toml
thesaurus = { version = "0.4.0", features = ["moby"], default_features = false }
```

Both moby and wordnet have the same APIs, but they are included in different modules so you can use both in the same binary (in theory).

## Backend Comparison
Name | Simple Example Binary Size | Simple Example Binary Size (Stripped) | Available Words | Average Number of Synonyms | Compressed Dictionary Size | License
---|---|---|---|---|---|---
Moby | 15M | 11M | 30259 | 83.287 | 11M | US Public Domain
Wordnet | 6.9M | 3.4M | 125701 | 3.394 | 2.9M | [Wordnet License](https://wordnet.princeton.edu/license-and-commercial-use)

## Basic Wordnet Example
```rust
use thesaurus::wordnet;

fn main() {
    let word = "good";
    let results = wordnet::synonyms(word).unwrap();

    println!("Found {} results for {}:", results.len(), word);

    for word in results {
        println!("  {word}");
    }
}
```

Result:
```none
Found 107 results for good:
  skilled
  skilful
  practiced
  skillful
  expert
  adept
  proficient
  sainted
  ...
```