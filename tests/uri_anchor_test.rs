use std::convert::TryFrom;

use uri::Uri;

// A U+0020 space in the fragment
#[test]
fn test_1() {
    let input = "http://www.example.com/#hello, world";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://www.example.com/#hello, world";
    assert_eq!(&uri.normalize(), expect_uri);
}

// Percent-encoding in the fragment
#[test]
fn test_2() {
    let input = "http://www.example.com/#%c2%a9";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://www.example.com/#%c2%a9";
    assert_eq!(&uri.normalize(), expect_uri);
}

// Unicode surrogates
#[test]
fn test_3() {
    let input = r#"http://www.example.com/#\ud800\udf00ss"#;
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = r#"http://www.example.com/#\ud800\udf00ss"#;
    assert_eq!(&uri.normalize(), expect_uri);
}

// illegal percent-encoding
#[test]
fn test_4() {
    let input = "http://www.example.com/#%41%a";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://www.example.com/#%41%a";
    assert_eq!(&uri.normalize(), expect_uri);
}

// // Illegal Unicode half-surrogate U+D800
// #[test]
// fn test_5() {
//     let input = r#"http://www.example.com/#\\\ud800\\\u597D"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.example.com/#\\\uFFFD\\\u597D"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// // Illegal Unicode U+FDD0 in the fragment
// #[test]
// fn test_6() {
//     let input = r#"http://www.example.com/#a\\\uFDD0"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.example.com/#a\\\uFFFD"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// An extra U+0023 NUMBER SIGN in middle of the fragment
#[test]
fn test_7() {
    let input = "http://www.example.com/#asdf#qwer";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://www.example.com/#asdf#qwer";
    assert_eq!(&uri.normalize(), expect_uri);
}

// An extra U+0023 NUMBER SIGN at the start of fragment
#[test]
fn test_8() {
    let input = "http://www.example.com/##asdf";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://www.example.com/##asdf";
    assert_eq!(&uri.normalize(), expect_uri);
}

// // Extra white space characters
// #[test]
// fn test_9() {
//     let input = r#"http://www.example.com/#a\u000Ab\u000Dc\u0009d"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/#abcd";
//     assert_eq!(&uri.normalize(), expect_uri);
// }
