// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = "http://www.google.com/foo?bar=baz#";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.google.com/foo?bar=baz#";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_2() {
//     let input = "http://[www.google.com]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[www.google.com]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_3() {
//     let input = "http:////////user:@google.com:99?foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://user@google.com:99/?foo";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_4() {
//     let input = "http://192.0x00A80001";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_5() {
//     let input = "http://www/foo%2Ehtml";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www/foo.html";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_6() {
//     let input = "http://user:pass@/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://user:pass@/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_7() {
//     let input = "http://%25DOMAIN:foobar@foodomain.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://%25DOMAIN:foobar@foodomain.com/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_8() {
//     let input = "http:\\\\\\\\www.google.com\\\\foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.google.com/foo";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_9() {
//     let input = r#"http://www.google.com/asdf#\\\ud800"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.google.com/asdf#\\\uFFFD"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_10() {
//     let input = "http://foo:80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://foo/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_11() {
//     let input = "http://foo:81/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://foo:81/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_12() {
//     let input = "httpa://foo:80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "httpa://foo:80/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_13() {
//     let input = "http://foo:-80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://foo:-80/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_14() {
//     let input = "https://foo:443/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "https://foo/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_15() {
//     let input = "https://foo:80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "https://foo:80/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_16() {
//     let input = "ftp://foo:21/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "ftp://foo/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_17() {
//     let input = "ftp://foo:80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "ftp://foo:80/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_18() {
//     let input = "gopher://foo:70/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "gopher://foo/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_19() {
//     let input = "gopher://foo:443/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "gopher://foo:443/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_20() {
//     let input = "ws://foo:80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "ws://foo/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_21() {
//     let input = "ws://foo:81/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "ws://foo:81/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_22() {
//     let input = "ws://foo:443/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "ws://foo:443/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_23() {
//     let input = "ws://foo:815/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "ws://foo:815/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_24() {
//     let input = "wss://foo:80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "wss://foo:80/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_25() {
//     let input = "wss://foo:81/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "wss://foo:81/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_26() {
//     let input = "wss://foo:443/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "wss://foo/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_27() {
//     let input = "wss://foo:815/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "wss://foo:815/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }
