Words Count
====================

[![CI](https://github.com/magiclen/words-count/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/words-count/actions/workflows/ci.yml)

Count the words and characters, with or without whitespaces.

The algorithm is roughly aligned with the way Libreoffice is counting words.
This means that it does not exactly match the
[Unicode Text Segmentation](https://unicode.org/reports/tr29/#Word_Boundaries)
standard.

## Examples

```rust
use words_count::WordsCount;

assert_eq!(WordsCount {
    words: 20,
    characters: 31,
    whitespaces: 2,
    cjk: 18,
}, words_count::count("Rust是由 Mozilla 主導開發的通用、編譯型程式語言。"));
```

```rust
let result = words_count::count_separately("apple banana apple");

assert_eq!(2, result.len());
assert_eq!(Some(&2), result.get("apple"));
```

## Crates.io

https://crates.io/crates/words-count

## Documentation

https://docs.rs/words-count

## License

[MIT](LICENSE)