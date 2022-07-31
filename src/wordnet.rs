//! Thesaurus backend based on [wordnet](https://wordnet.princeton.edu/).
//!
//! ```rust
//! use thesaurus::wordnet;
//!
//! fn main() {
//!     let word = "good";
//!     let results = wordnet::synonyms(word).unwrap();
//!
//!     println!("Found {} results for {}:", results.len(), word);
//!
//!     for word in results {
//!         println!("  {word}");
//!     }
//! }
//! ```

pub use thesaurus_wordnet::{dict, init, synonyms};