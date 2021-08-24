use std::convert::TryFrom;

use uri::Uri;

//
#[test]
fn test_1() {
    let input = "mailto:addr1";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "mailto:addr1";
    assert_eq!(&uri.normalize(), expect_uri);
}

//
#[test]
fn test_2() {
    let input = "mailto:addr1@foo.com";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "mailto:addr1@foo.com";
    assert_eq!(&uri.normalize(), expect_uri);
}

//
// #[test]
// fn test_3() {
//     let input = "mailto:addr1 \\t ";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "mailto:addr1";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

//
#[test]
fn test_4() {
    let input = "mailto:addr1?to=jon";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "mailto:addr1?to=jon";
    assert_eq!(&uri.normalize(), expect_uri);
}

//
#[test]
fn test_5() {
    let input = "mailto:addr1,addr2";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "mailto:addr1,addr2";
    assert_eq!(&uri.normalize(), expect_uri);
}

//
#[test]
fn test_6() {
    let input = "mailto:addr1, addr2";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "mailto:addr1, addr2";
    assert_eq!(&uri.normalize(), expect_uri);
}

//
#[test]
fn test_7() {
    let input = "mailto:addr1%2caddr2";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "mailto:addr1%2caddr2";
    assert_eq!(&uri.normalize(), expect_uri);
}

//
// #[test]
// fn test_8() {
//     let input = r#"mailto:\uD800\uDF00"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "mailto:%F0%90%8C%80";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

//
#[test]
fn test_9() {
    let input = "mailto:addr1?";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "mailto:addr1?";
    assert_eq!(&uri.normalize(), expect_uri);
}
