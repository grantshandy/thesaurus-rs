# thesaurus-rs
A thesaurus library for Rust.

Add to `Cargo.toml`
```
thesaurus = "0.1.3"
```


`thesaurus-rs` is extremely simple library for simple programs that need a thesaurus, but don't need an internet connection. It relies on the JSON file from [zaibacu](https://github.com/zaibacu/thesaurus).

It's simple to use:
```rust
use thesaurus::{WordType, Thesaurus};

fn main() {
    match Thesaurus::synonym("good", Some(WordType::Noun)) {
        Ok(data) => {
            let mut synonyms = String::new();

            for synonym in data.words.iter() {
                synonyms.push_str(&format!("\n    {} ({})", synonym.name, synonym.word_type));
            };

            println!("Word: {}\nSynonyms:{}", data.name, synonyms);
        },
        Err(error) => eprintln!("Error: {}", error),
    };
}
```

Result:
```
Word: good
Synonyms:
    commodity (Noun)
    trade good (Noun)
    goodness (Noun)
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

While the actual dictionary, from [WordNet](https://wordnet.princeton.edu/) uses this license:

```
License and Commercial Use of WordNet

WordNet® is unencumbered, and may be used in commercial applications in accordance with the following license agreement. An attorney representing the commercial interest should review this WordNet license with respect to the intended use.

WordNet License

This license is available as the file LICENSE in any downloaded version of WordNet.

WordNet 3.0 license: (Download)

WordNet Release 3.0 This software and database is being provided to you, the LICENSEE, by Princeton University under the following license. By obtaining, using and/or copying this software and database, you agree that you have read, understood, and will comply with these terms and conditions.: Permission to use, copy, modify and distribute this software and database and its documentation for any purpose and without fee or royalty is hereby granted, provided that you agree to comply with the following copyright notice and statements, including the disclaimer, and that the same appear on ALL copies of the software, database and documentation, including modifications that you make for internal use or for distribution. WordNet 3.0 Copyright 2006 by Princeton University. All rights reserved. THIS SOFTWARE AND DATABASE IS PROVIDED "AS IS" AND PRINCETON UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED. BY WAY OF EXAMPLE, BUT NOT LIMITATION, PRINCETON UNIVERSITY MAKES NO REPRESENTATIONS OR WARRANTIES OF MERCHANT- ABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF THE LICENSED SOFTWARE, DATABASE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS. The name of Princeton University or Princeton may not be used in advertising or publicity pertaining to distribution of the software and/or database. Title to copyright in this software, database and any associated documentation shall at all times remain with Princeton University and LICENSEE agrees to preserve same.
```