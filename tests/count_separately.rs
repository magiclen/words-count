extern crate words_count;

use words_count::*;

#[test]
fn empty() {
    let result = count_separately("");

    assert!(result.is_empty());
}

#[test]
fn whitespaces() {
    let result = count_separately("      ");

    assert!(result.is_empty());
}

#[test]
fn number() {
    let result = count_separately("1234 5678 -90");

    assert_eq!(3, result.len());
    assert_eq!(Some(&1), result.get("1234"));
    assert_eq!(Some(&1), result.get("5678"));
    assert_eq!(Some(&1), result.get("-90"));
}

#[test]
fn english_alphabet() {
    let result = count_separately("abc def ghi, jkl");

    assert_eq!(4, result.len());
    assert_eq!(Some(&1), result.get("abc"));
    assert_eq!(Some(&1), result.get("def"));
    assert_eq!(Some(&1), result.get("ghi,"));
    assert_eq!(Some(&1), result.get("jkl"));
}

#[test]
fn cjk_alphabet() {
    let result = count_separately("涼風有訊 秋月無邊。虧我思嬌的情緒好比度日如年。");

    assert_eq!(22, result.len());
    assert_eq!(Some(&2), result.get("。"));
}

#[test]
fn english_cjk_mix() {
    let result = count_separately("大家來meething吧，不然要錯過deadline了。");

    assert_eq!(14, result.len());
    assert_eq!(Some(&1), result.get("大"));
    assert_eq!(Some(&1), result.get("meething"));
}

#[test]
fn dashed_word() {
    let result = count_separately("abc-def");

    assert_eq!(1, result.len());
    assert_eq!(Some(&1), result.get("abc-def"));

    let result = count_separately("abc--def");

    assert_eq!(2, result.len());
    assert_eq!(Some(&1), result.get("abc"));
    assert_eq!(Some(&1), result.get("def"));

    let result = count_separately("abc---def");

    assert_eq!(2, result.len());
    assert_eq!(Some(&1), result.get("abc"));
    assert_eq!(Some(&1), result.get("def"));
}

#[test]
fn dash_ended() {
    let result = count_separately("abc-");

    assert_eq!(1, result.len());
    assert_eq!(Some(&1), result.get("abc-"));

    let result = count_separately("abc- ");

    assert_eq!(1, result.len());
    assert_eq!(Some(&1), result.get("abc-"));

    let result = count_separately("abc--");

    assert_eq!(1, result.len());
    assert_eq!(Some(&1), result.get("abc"));

    let result = count_separately("abc-- ");

    assert_eq!(1, result.len());
    assert_eq!(Some(&1), result.get("abc"));

    let result = count_separately("abc--中");

    assert_eq!(2, result.len());
    assert_eq!(Some(&1), result.get("abc"));
    assert_eq!(Some(&1), result.get("中"));
}
