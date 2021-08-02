// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = r#"http://B\u00FCcher.de/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://xn--bcher-kva.de/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_2() {
//     let input = r#"http://fa\u00DF.de/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://xn--fa-hia.de/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_3() {
//     let input = r#"http://\u03B2\u03CC\u03BB\u03BF\u03C2.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://xn--nxasmm1c.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_4() {
//     let input = r#"http://\u0DC1\u0DCA\u200D\u0DBB\u0DD3.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://xn--10cl1a0b660p.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_5() {
//     let input = r#"http://\u0646\u0627\u0645\u0647\u200C\u0627\u06CC.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://xn--mgba3gch31f060k.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_6() {
//     let input = r#"http://\u2665.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://\uFFFD.net/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_7() {
//     let input = r#"http://\u0378.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://\uFFFD.net/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_8() {
//     let input = r#"http://\u04C0.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://\uFFFD.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_9() {
//     let input = r#"http://\uD87E\uDC68.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://\uFFFD.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_10() {
//     let input = r#"http://\u2183.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://\uFFFD.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_11() {
//     let input = r#"http://look\u034Fout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://lookout.net/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_12() {
//     let input = r#"http://gOoGle.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://google.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_13() {
//     let input = r#"http://\u09dc.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://\u09A1\u09BC.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_14() {
//     let input = r#"http://\u1E9E.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://ss.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_15() {
//     let input = r#"http://\u1E9E.foo.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://ss.foo.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_16() {
//     let input = r#"http://-foo.bar.com"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_17() {
//     let input = r#"http://foo-.bar.com"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_18() {
//     let input = r#"http://ab--cd.com"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_19() {
//     let input = r#"http://xn--0.com"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_20() {
//     let input = r#"http://foo\u0300.bar.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = r#"http://foo%CC%80.bar.com/"#;
//     assert_eq!(&uri.normalize(), expect_url);
// }
