use std::convert::TryFrom;

use uri::Uri;

//
#[test]
fn test_1() {
    let input = "http://example.com/";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_url = "http://example.com/";
    assert_eq!(&uri.normalize(), expect_url);
}

//
#[test]
fn test_2() {
    let input = "HTTP://example.com/";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_url = "http://example.com/";
    assert_eq!(&uri.normalize(), expect_url);
}
