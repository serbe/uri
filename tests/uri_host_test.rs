// use std::convert::TryFrom;

// use uri::Uri;

// // A pathless URL
// #[test]
// fn test_1() {
//     let input = "http://google.com";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://google.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_2() {
//     let input = "http://GoOgLe.CoM/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://google.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_3() {
//     let input = "http://Goo%20 goo%7C|.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://goo%20%20goo%7C%7C.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_4() {
//     let input = r#"http://GOO\u00a0\u3000goo.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://goo%20%20goo.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_5() {
//     let input = r#"http://GOO\u200b\u2060\ufeffgoo.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://googoo.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_6() {
//     let input = r#"http://www.foo\u3002bar.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://www.foo.bar.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_7() {
//     let input = r#"http://\ufdd0zyx.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://%EF%BF%BDzyx.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_8() {
//     let input = r#"http://%ef%b7%90zyx.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://%EF%BF%BDzyx.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_9() {
//     let input = r#"http://\uff27\uff4f.com/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://go.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// // #[test]
// // fn test_10() {
// //     let input = r#"http://\uff05\uff14\uff11.com/"#;
// //     let uri: Uri = Uri::try_from(input).unwrap();
// //     let expect_url = "http://a.com/";
// //     assert_eq!(&uri.normalize(), expect_url);
// // }

// //
// // #[test]
// // fn test_11() {
// //     let input = r#"http://%ef%bc%85%ef%bc%94%ef%bc%91.com/"#;
// //     let uri: Uri = Uri::try_from(input).unwrap();
// //     let expect_url = "http://a.com/";
// //     assert_eq!(&uri.normalize(), expect_url);
// // }

// //
// // #[test]
// // fn test_12() {
// //     let input = r#"http://\uff05\uff10\uff10.com/"#;
// //     let uri: Uri = Uri::try_from(input).unwrap();
// //     let expect_url = "http://%00.com/";
// //     assert_eq!(&uri.normalize(), expect_url);
// // }

// //
// // #[test]
// // fn test_13() {
// //     let input = "http://%ef%bc%85%ef%bc%90%ef%bc%90.com/";
// //     let uri: Uri = Uri::try_from(input).unwrap();
// //     let expect_url = "http://%00.com/";
// //     assert_eq!(&uri.normalize(), expect_url);
// // }

// //
// // #[test]
// // fn test_14() {
// //     let input = r#"http://\u4f60\u597d\u4f60\u597d/"#;
// //     let uri: Uri = Uri::try_from(input).unwrap();
// //     let expect_url = "http://xn--6qqa088eba/";
// //     assert_eq!(&uri.normalize(), expect_url);
// // }

// //
// // #[test]
// // fn test_15() {
// //     let input = r#"http://%E4%BD%A0%E5%A5%BD\u4f60\u597d/"#;
// //     let uri: Uri = Uri::try_from(input).unwrap();
// //     let expect_url = "http://xn--6qqa088eba/";
// //     assert_eq!(&uri.normalize(), expect_url);
// // }

// //
// // #[test]
// // fn test_16() {
// //     let input = "http://%zz%66%a/";
// //     let uri: Uri = Uri::try_from(input).unwrap();
// //     let expect_url = "http://%25zzf%25a/";
// //     assert_eq!(&uri.normalize(), expect_url);
// // }

// //
// #[test]
// fn test_17() {
//     let input = "http://%25/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://%25/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_18() {
//     let input = "http://hello%00/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://hello%00/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_19() {
//     let input = "http://%30%78%63%30%2e%30%32%35%30.01/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_20() {
//     let input = "http://%30%78%63%30%2e%30%32%35%30.01%2e/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_21() {
//     let input = "http://%3g%78%63%30%2e%30%32%35%30%2E.01/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://%253gxc0.0250..01/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_22() {
//     let input = "http://192.168.0.1 hello/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1%20hello/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_23() {
//     let input =
//         r#"http://\uff10\uff38\uff43\uff10\uff0e\uff10\uff12\uff15\uff10\uff0e\uff10\uff11/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_24() {
//     let input = "http://192.168.0.257/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.257/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_25() {
//     let input = "http://[google.com.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://[google.com]/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_26() {
//     let input = r#"http://\u0442("#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://xn--%28-7ed/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_27() {
//     let input = "http://go\\@ogle.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://go@ogle.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_28() {
//     let input = "http://go/@ogle.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://go@ogle.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_29() {
//     let input = "http://www.lookout.net::==80::==443::/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://lookout.net/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_30() {
//     let input = "http://www.lookout.net::80::443/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://lookout.net/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_31() {
//     let input = "http://\\";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://\\";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_32() {
//     let input = "http://\\\\/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_33() {
//     let input = "http://\\./";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_34() {
//     let input = "http:////:@/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_35() {
//     let input = "http://\\google.com/foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://google.com/foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_36() {
//     let input = "http://\\\\google.com/foo";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://google.com/foo";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_37() {
//     let input = "http:////asdf@/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_38() {
//     let input = "http:////:81/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_39() {
//     let input = "http://://";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_40() {
//     let input = "http://c:/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_41() {
//     let input = "http://xxxx:/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_42() {
//     let input = "http://.:./";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_43() {
//     let input = "http://////@google.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://.";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_44() {
//     let input = "http://@google.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://google.com/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// // A URL with a pipe character in the hostname.
// #[test]
// fn test_45() {
//     let input = "http://c|/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://c|/";
//     assert_eq!(&uri.normalize(), expect_url);
// }
