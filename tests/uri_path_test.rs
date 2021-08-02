// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = "http://www.example.com/././foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_2() {
//     let input = "http://www.example.com/./.foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/.foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_3() {
//     let input = "http://www.example.com/foo/.";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_4() {
//     let input = "http://www.example.com/foo/./";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_5() {
//     let input = "http://www.example.com/foo/bar/..";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_6() {
//     let input = "http://www.example.com/foo/bar/../";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_7() {
//     let input = "http://www.example.com/foo/..bar";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/..bar";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_8() {
//     let input = "http://www.example.com/foo/bar/../ton";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/ton";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_9() {
//     let input = "http://www.example.com/foo/bar/../ton/../../a";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/a";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_10() {
//     let input = "http://www.example.com/foo/../../..";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_11() {
//     let input = "http://www.example.com/foo/../../../ton";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/ton";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_12() {
//     let input = "http://www.example.com/foo/%2e";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_13() {
//     let input = "http://www.example.com/foo/%2e%2";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/.%2";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_14() {
//     let input = "http://www.example.com/foo/%2e./%2e%2e/.%2e/%2e.bar";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/..bar";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_15() {
//     let input = "http://www.example.com////../..";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com//";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_16() {
//     let input = "http://www.example.com/foo/bar//../..";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_17() {
//     let input = "http://www.example.com/foo/bar//..";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/bar/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_18() {
//     let input = "http://www.example.com/foo/bar/..";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_19() {
//     let input = "http://www.example.com/foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_20() {
//     let input = "http://www.example.com/%20foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%20foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_21() {
//     let input = "http://www.example.com/foo%";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo%";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_22() {
//     let input = "http://www.example.com/foo%2";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo%2";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_23() {
//     let input = "http://www.example.com/foo%2zbar";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo%2zbar";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_24() {
//     let input = r#"http://www.example.com/foo%2\u00c2\u00a9zbar"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo%2%C3%82%C2%A9zbar";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_25() {
//     let input = "http://www.example.com/foo%41%7a";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/fooAz";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_26() {
//     let input = r#"http://www.example.com/foo\u0009\u0091%91"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo%09%C2%91%91";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_27() {
//     let input = "http://www.example.com/foo%00%51";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo%00Q";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_28() {
//     let input = "http://www.example.com/(%28:%3A%29)";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/(%28:%3A%29)";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_29() {
//     let input = "http://www.example.com/%3A%3a%3C%3c";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%3A%3a%3C%3c";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_30() {
//     let input = "http://www.example.com/foo\\tbar";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo%09bar";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_31() {
//     let input = "http://www.example.com\\\\foo\\\\bar";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/bar";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_32() {
//     let input = "http://www.example.com/%7Ffp3%3Eju%3Dduvgw%3Dd";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%7Ffp3%3Eju%3Dduvgw%3Dd";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_33() {
//     let input = "http://www.example.com/@asdf%40";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/@asdf%40";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_34() {
//     let input = r#"http://www.example.com/\u4f60\u597d\u4f60\u597d"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%E4%BD%A0%E5%A5%BD%E4%BD%A0%E5%A5%BD";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_35() {
//     let input = r#"http://www.example.com/\ufdd0zyx"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%EF%BF%BDzyx";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_36() {
//     let input = r#"http://www.example.com/\u2025/foo"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%E2%80%A5/foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_37() {
//     let input = r#"http://www.example.com/\uDEAD/foo"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://www.example.com/\uFFFD/foo"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_38() {
//     let input = r#"http://www.example.com/\uFEFF/foo"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%EF%BB%BF/foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_39() {
//     let input = r#"http://www.example.com/\u202E/foo/\u202D/bar"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/%E2%80%AE/foo/%E2%80%AD/bar";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_40() {
//     let input = r#"http://www.example.com\uFF0Ffoo/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.example.com/foo/";
//     assert_eq!(&uri.normalize(), expect_url);
// }
