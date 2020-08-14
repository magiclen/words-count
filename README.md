Word Count
====================

[![Build Status](https://travis-ci.org/magiclen/word-count.svg?branch=master)](https://travis-ci.org/magiclen/word-count)

Count the words and characters, with or without whitespaces.

## Examples

```rust
extern crate word_count;

use word_count::WordCount;

assert_eq!(WordCount {
    words: 20,
    characters: 31,
    whitespaces: 2,
    cjk: 18,
}, word_count::count("Rust是由 Mozilla 主導開發的通用、編譯型程式語言。"));
```

```rust
extern crate word_count;

let result = word_count::count_separately("apple banana apple");

assert_eq!(2, result.len());
assert_eq!(Some(&2), result.get("apple"));
```

## Crates.io

https://crates.io/crates/word-count

## Documentation

https://docs.rs/word-count

## License

[MIT](LICENSE)