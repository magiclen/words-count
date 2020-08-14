extern crate word_count;

use word_count::*;

#[test]
fn empty() {
    assert_eq!(
        WordCount {
            words: 0,
            characters: 0,
            whitespaces: 0,
            cjk: 0,
        },
        count("")
    );
}

#[test]
fn whitespaces() {
    assert_eq!(
        WordCount {
            words: 0,
            characters: 6,
            whitespaces: 6,
            cjk: 0,
        },
        count("      ")
    );
}

#[test]
fn number() {
    assert_eq!(
        WordCount {
            words: 3,
            characters: 13,
            whitespaces: 2,
            cjk: 0,
        },
        count("1234 5678 -90")
    );
}

#[test]
fn english_alphabet() {
    assert_eq!(
        WordCount {
            words: 4,
            characters: 16,
            whitespaces: 3,
            cjk: 0,
        },
        count("abc def ghi, jkl")
    );
}

#[test]
fn cjk_alphabet() {
    assert_eq!(
        WordCount {
            words: 23,
            characters: 24,
            whitespaces: 1,
            cjk: 23,
        },
        count("涼風有訊 秋月無邊。虧我思嬌的情緒好比度日如年。")
    );
}

#[test]
fn english_cjk_mix() {
    assert_eq!(
        WordCount {
            words: 14,
            characters: 28,
            whitespaces: 0,
            cjk: 12,
        },
        count("大家來meething吧，不然要錯過deadline了。")
    );
}

#[test]
fn dashed_word() {
    assert_eq!(
        WordCount {
            words: 1,
            characters: 7,
            whitespaces: 0,
            cjk: 0,
        },
        count("abc-def")
    );

    assert_eq!(
        WordCount {
            words: 2,
            characters: 8,
            whitespaces: 0,
            cjk: 0,
        },
        count("abc--def")
    );

    assert_eq!(
        WordCount {
            words: 2,
            characters: 9,
            whitespaces: 0,
            cjk: 0,
        },
        count("abc---def")
    );
}

#[test]
fn dash_ended() {
    assert_eq!(
        WordCount {
            words: 1,
            characters: 4,
            whitespaces: 0,
            cjk: 0,
        },
        count("abc-")
    );

    assert_eq!(
        WordCount {
            words: 1,
            characters: 5,
            whitespaces: 1,
            cjk: 0,
        },
        count("abc- ")
    );

    assert_eq!(
        WordCount {
            words: 1,
            characters: 5,
            whitespaces: 0,
            cjk: 0,
        },
        count("abc--")
    );

    assert_eq!(
        WordCount {
            words: 1,
            characters: 6,
            whitespaces: 1,
            cjk: 0,
        },
        count("abc-- ")
    );

    assert_eq!(
        WordCount {
            words: 2,
            characters: 6,
            whitespaces: 0,
            cjk: 1,
        },
        count("abc--中")
    );
}
