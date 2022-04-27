# Simple trie implementation
This is an unoptimized not-production-ready trie implementation without path compression. Just a fun project.

[![Crates.io](https://img.shields.io/crates/v/mytrie.svg)](https://crates.io/crates/mytrie)
[![Documentation](https://docs.rs/mytrie/badge.svg)](https://docs.rs/mytrie/latest/mytrie/)
[![Dependency status](https://deps.rs/crate/mytrie/0.1.0/status.svg)](https://deps.rs/crate/mytrie/0.1.0)

## Example
```rs
use mytrie::Trie;

let trie = Trie::from(["Hallo", "Hallöchen", "Tschüs"]);
let mut content: Vec<String> = trie.iter_content("Hall").collect();

content.sort();
assert_eq!(content, ["Hallo", "Hallöchen"]);
```