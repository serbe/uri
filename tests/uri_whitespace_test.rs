use std::convert::TryFrom;

use uri::Uri;

//
#[test]
fn test_1() {
    let input = "http://www.example.com/foo    bar/?   foo   =   bar     #    foo";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://www.example.com/foo    bar/?   foo   =   bar     #    foo";
    assert_eq!(&uri.normalize(), expect_uri);
}

//
#[test]
fn test_2() {
    let input = "http://www.example.com/ ";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://www.example.com/ ";
    assert_eq!(&uri.normalize(), expect_uri);
}
