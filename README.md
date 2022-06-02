# thesaurus-rs
[![Crates.io](https://img.shields.io/crates/v/thesaurus.svg)](https://crates.io/crates/thesaurus)
[![Crates.io](https://img.shields.io/crates/d/thesaurus)](https://crates.io/crates/thesaurus)
[![API](https://docs.rs/thesaurus/badge.svg)](https://docs.rs/thesaurus)

A thesaurus library for Rust.

Add to `Cargo.toml`
```
thesaurus = "0.3.0"
```

`thesaurus-rs` is extremely simple library for simple programs that need a thesaurus, but don't have an internet connection. It relies on the thesaurus file from the [Moby Project](https://en.wikipedia.org/wiki/Moby_Project).

It's fairly simple to use:
```rust
fn main() {
    let word = "good";
    let results = thesaurus::synonym(word).unwrap();

    println!("Found {} results for {}:", results.len(), word);

    for word in results {
        println!("  {word}");
    }
}
```

Result:
```
Found 665 results for good:
  acceptable
  accomplished
  according to Hoyle
  ace
  actual
  adept
  adequate
  admirable
  admissible
  adroit
  advantage
  ...
```

## Licenses
`thesaurus-rs` is written under the MIT license:
```
MIT License

Copyright (c) 2021 Grant Handy

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

The actual underlying thesaurus in the public domain.

### Runtime Decompression
This library compresses the text file at build time into a file with gzip compression that is then built into the binary. This makes your binary significantly smaller than if it was stored uncompressed.

- Uncompressed: 24M
- Compressed: 11M