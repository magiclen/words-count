/*!
# Words Count

Count the words and characters, with or without whitespaces.

## Examples

```rust
extern crate words_count;

use words_count::WordsCount;

assert_eq!(WordsCount {
    words: 20,
    characters: 31,
    whitespaces: 2,
    cjk: 18,
}, words_count::count("Rust是由 Mozilla 主導開發的通用、編譯型程式語言。"));
```

```rust
extern crate words_count;

let result = words_count::count_separately("apple banana apple");

assert_eq!(2, result.len());
assert_eq!(Some(&2), result.get("apple"));
```
*/

#![no_std]

extern crate alloc;

extern crate unicode_blocks;

use core::str::from_utf8_unchecked;

use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WordsCount {
    pub words: usize,
    pub characters: usize,
    pub whitespaces: usize,
    pub cjk: usize,
}

/// Count the words in the given string. In general, every non-CJK string of characters between two whitespaces is a word. Dashes (at least two dashes) are word limit, too. A CJK character is considered to be an independent word.
pub fn count<S: AsRef<str>>(s: S) -> WordsCount {
    let mut in_word = false;
    let mut consecutive_dashes = 0usize;

    let mut count = WordsCount {
        words: 0,
        characters: 0,
        whitespaces: 0,
        cjk: 0,
    };

    for c in s.as_ref().chars() {
        count.characters += 1;

        if c.is_whitespace() {
            consecutive_dashes = 0;

            count.whitespaces += 1;

            if in_word {
                count.words += 1;

                in_word = false;
            }
        } else {
            match c {
                '-' => {
                    consecutive_dashes += 1;

                    if consecutive_dashes > 1 && in_word {
                        if consecutive_dashes == 2 {
                            count.words += 1;
                        }

                        in_word = false;

                        continue;
                    }
                }
                _ => {
                    consecutive_dashes = 0;

                    if unicode_blocks::is_cjk(c) {
                        count.words += 1;
                        count.cjk += 1;

                        if in_word {
                            count.words += 1;

                            in_word = false;
                        }

                        continue;
                    }
                }
            }

            if !in_word {
                in_word = true;
            }
        }
    }

    if in_word {
        count.words += 1;
    }

    count
}

/// Count the words separately in the given string. In general, every non-CJK string of characters between two whitespaces is a word. Dashes (at least two dashes) are word limit, too. A CJK character is considered to be an independent word. Punctuations are not handled.
pub fn count_separately<S: ?Sized + AsRef<str>>(s: &S) -> BTreeMap<&str, usize> {
    let mut in_word = false;
    let mut consecutive_dashes = 0usize;

    let mut count = BTreeMap::new();

    let mut p = 0;
    let mut pp = 0;

    let s = s.as_ref();
    let bytes = s.as_bytes();

    for c in s.chars() {
        let cl = c.len_utf8();

        if c.is_whitespace() {
            if in_word {
                inc_or_insert(&mut count, unsafe { from_utf8_unchecked(&bytes[p..pp]) });

                in_word = false;
            }

            p = pp + cl;

            consecutive_dashes = 0;
        } else {
            match c {
                '-' => {
                    consecutive_dashes += 1;

                    if consecutive_dashes > 1 {
                        if in_word {
                            if consecutive_dashes == 2 {
                                inc_or_insert(&mut count, unsafe {
                                    from_utf8_unchecked(&bytes[p..(pp - 1)])
                                });
                            }

                            in_word = false;

                            pp += cl;
                            p = pp;
                            continue;
                        } else {
                            p = pp + cl;
                        }
                    }
                }
                _ => {
                    if unicode_blocks::is_cjk(c) {
                        inc_or_insert(&mut count, unsafe {
                            from_utf8_unchecked(&bytes[pp..(pp + cl)])
                        });

                        if in_word {
                            inc_or_insert(&mut count, unsafe {
                                from_utf8_unchecked(&bytes[p..pp])
                            });

                            in_word = false;
                        }

                        consecutive_dashes = 0;
                        pp += cl;
                        p = pp;
                        continue;
                    }

                    consecutive_dashes = 0;
                }
            }

            if !in_word {
                in_word = true;
            }
        }

        pp += cl;
    }

    if in_word {
        inc_or_insert(&mut count, unsafe { from_utf8_unchecked(&bytes[p..pp]) });
    }

    count
}

#[inline]
fn inc_or_insert<'a>(map: &mut BTreeMap<&'a str, usize>, s: &'a str) {
    if let Some(count) = map.get_mut(s) {
        *count += 1;
    } else {
        map.insert(s, 1);
    }
}
