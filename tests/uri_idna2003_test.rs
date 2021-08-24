// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = r#"http://fa\u00DF.de/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://fass.de/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_2() {
//     let input = r#"http://\u03B2\u03CC\u03BB\u03BF\u03C2.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://xn--nxasmq6b.com/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_3() {
//     let input = r#"http://\u0DC1\u0DCA\u200D\u0DBB\u0DD3.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://xn--10cl1a0b.com/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_4() {
//     let input = r#"http://\u0646\u0627\u0645\u0647\u200C\u0627\u06CC.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://xn--mgba3gch31f.com/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_5() {
//     let input = r#"http://www.loo\u0138out.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.xn--looout-5bb.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_6() {
//     let input = r#"http://\u15EF\u15EF\u15EF.lookout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://xn--1qeaa.lookout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_7() {
//     let input = r#"http://www.lookout.\u0441\u043E\u043C/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.lookout.xn--l1adi/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_8() {
//     let input = r#"http://www.lookout.net\uFF1A80/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.lookout.net:80/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_9() {
//     let input = r#"http://www\u2025lookout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www..lookout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_10() {
//     let input = r#"http://www.lookout\u2027net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.xn--lookoutnet-406e/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_11() {
//     let input = r#"http://www.loo\u0138out.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.xn--looout-5bb.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_12() {
//     let input = r#"http://www.lookout.net\u2A7480/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www.lookout.net::%3D80/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_13() {
//     let input = r#"http://www\u00A0.lookout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://www%20.lookout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_14() {
//     let input = r#"http://\u1680lookout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://%E1%9A%80lookout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_15() {
//     let input = r#"http://\u001Flookout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://%1Flookout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_16() {
//     let input = r#"http://look\u06DDout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%DB%9Dout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_17() {
//     let input = r#"http://look\u180Eout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%E1%A0%8Eout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_18() {
//     let input = r#"http://look\u2060out.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%E2%81%A0out.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_19() {
//     let input = r#"http://look\uFEFFout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%EF%BB%BFout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_20() {
//     let input = r#"http://look\uD83F\uDFFEout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%F0%9F%BF%BEout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_21() {
//     let input = r#"http://look\uDEADout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%ED%BA%ADout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_22() {
//     let input = r#"http://look\uFFFAout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%EF%BF%BAout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_23() {
//     let input = r#"http://look\u2FF0out.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%E2%BF%B0out.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_24() {
//     let input = r#"http://look\u0341out.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%CD%81out.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_25() {
//     let input = r#"http://look\u202Eout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%E2%80%AEout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_26() {
//     let input = r#"http://look\u206Bout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%E2%81%ABout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_27() {
//     let input = r#"http://look\uDB40\uDC01out.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%F3%A0%80%81out.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_28() {
//     let input = r#"http://look\uDB40\uDC20out.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%F3%A0%80%A0out.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_29() {
//     let input = r#"http://look\u05BEout.net/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = r#"http://look%D6%BEout.net/"#;
//     assert_eq!(&uri.normalize(), expect_uri);
// }
