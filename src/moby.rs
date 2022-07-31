//! Thesaurus backend based on [moby](https://en.wikipedia.org/wiki/Moby_Project).
//!
//! ```rust
//! use thesaurus::moby;
//!
//! fn main() {
//!     let word = "good";
//!     let results = moby::synonyms(word).unwrap();
//!
//!     println!("Found {} results for {}:", results.len(), word);
//!
//!     for word in results {
//!         println!("  {word}");
//!     }
//! }
//! ```

pub use thesaurus_moby::{dict, init, synonyms};