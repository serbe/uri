// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = "http://./";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_2() {
//     let input = "http://192.168.0.1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_3() {
//     let input = "http://0300.0250.00.01/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_4() {
//     let input = "http://0xC0.0Xa8.0x0.0x1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_5() {
//     let input = "http://192.168.9.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_6() {
//     let input = "http://19a.168.0.1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_7() {
//     let input = "http://0308.0250.00.01/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_8() {
//     let input = "http://0xCG.0xA8.0x0.0x1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_9() {
//     let input = "http://192/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.0.0.192/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_10() {
//     let input = "http://0xC0a80001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_11() {
//     let input = "http://030052000001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_12() {
//     let input = "http://000030052000001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_13() {
//     let input = "http://192.168/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.0.0.168/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_14() {
//     let input = "http://192.0x00A80001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_15() {
//     let input = "http://0xc0.052000001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_16() {
//     let input = "http://192.168.1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_17() {
//     let input = "http://192.168.0.0.1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_18() {
//     let input = "http://192.168.0.1./";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.168.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_19() {
//     let input = "http://192.168.0.1. hello/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_20() {
//     let input = "http://192.168.0.1../";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_21() {
//     let input = "http://192.168..1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_22() {
//     let input = "http://0x100.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_23() {
//     let input = "http://0x100.0.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_24() {
//     let input = "http://0x100.0.0.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_25() {
//     let input = "http://0.0x100.0.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_26() {
//     let input = "http://0.0.0x100.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_27() {
//     let input = "http://0.0.0.0x100/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_28() {
//     let input = "http://0.0.0x10000/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_29() {
//     let input = "http://0.0x1000000/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_30() {
//     let input = "http://0x100000000/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_32() {
//     let input = "http://0xFF.0.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://255.0.0.0/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_33() {
//     let input = "http://0xFF.0.0.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://255.0.0.0/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_34() {
//     let input = "http://0.0xFF.0.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.255.0.0/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_35() {
//     let input = "http://0.0.0xFF.0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.0.255.0/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_36() {
//     let input = "http://0.0.0.0xFF/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.0.0.255/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_37() {
//     let input = "http://0.0.0xFFFF/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.0.255.255/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_38() {
//     let input = "http://0.0xFFFFFF/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.255.255.255/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_39() {
//     let input = "http://0xFFFFFFFF/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://255.255.255.255/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_40() {
//     let input = "http://276.256.0xf1a2.077777/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_41() {
//     let input = "http://192.168.0.257/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_42() {
//     let input = "http://192.168.0xa20001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_43() {
//     let input = "http://192.015052000001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_44() {
//     let input = "http://0X12C0a80001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_45() {
//     let input = "http://276.1.2/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_46() {
//     let input = "http://192.168.0.1 hello/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_47() {
//     let input = "http://0000000000000300.0x00000000000000fF.00000000000000001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://192.255.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_48() {
//     let input = "http://0000000000000300.0xffffffffFFFFFFFF.3022415481470977/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_49() {
//     let input = "http://00000000000000000001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.0.0.1/";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_50() {
//     let input = "http://0000000000000000100000000000000001/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_51() {
//     let input = "http://0.0.0.000000000000000000z/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_52() {
//     let input = "http://0.0.0.100000000000000000z/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://";
//     assert_eq!(&uri.normalize(), expect_url);
// }

// //
// #[test]
// fn test_53() {
//     let input = "http://0.00.0x.0x0/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_url = "http://0.0.0.0/";
//     assert_eq!(&uri.normalize(), expect_url);
// }
