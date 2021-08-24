// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = "http://[/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_2() {
//     let input = "http://[:/";
//     // let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_3() {
//     let input = "http://]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_4() {
//     let input = "http://:]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_5() {
//     let input = "http://[]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_6() {
//     let input = "http://[:]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_7() {
//     let input = "http://2001:db8::1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_8() {
//     let input = "http://[2001:db8::1/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_9() {
//     let input = "http://2001:db8::1]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_10() {
//     let input = "http://[::]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[::]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_11() {
//     let input = "http://[::1]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[::1]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_12() {
//     let input = "http://[1::.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[1::.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_13() {
//     let input = "http://[::192.168.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[::c0a8:1.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_14() {
//     let input = "http://[::ffff:192.168.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[::ffff:c0a8:1.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_15() {
//     let input = "http://[000:01:02:003:004:5:6:007.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[0:1:2:3:4:5:6:7.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_16() {
//     let input = "http://[A:b:c:DE:fF:0:1:aC.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[a:b:c:de:ff:0:1:ac.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_17() {
//     let input = "http://[1:0:0:2::3:0.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[1::2:0:0:3:0.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_18() {
//     let input = "http://[1::2:0:0:3:0.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[1::2:0:0:3:0.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_19() {
//     let input = "http://[::eeee:192.168.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_20() {
//     let input = "http://[2001::192.168.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_21() {
//     let input = "http://[1:2:192.168.0.1:5:6.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_22() {
//     let input = "http://[::ffff:192.1.2.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[::ffff:c001:2.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_23() {
//     let input = "http://[::ffff:0xC0.0Xa8.0x0.0x1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[::ffff:c0a8:1.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_24() {
//     let input = "http://[0:0::0:0:8.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[::8.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_25() {
//     let input = "http://[2001:db8::1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[2001:db8::1.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_26() {
//     let input = "http://[2001::db8::1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_27() {
//     let input = "http://[2001:db8:::1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_28() {
//     let input = "http://[:::.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_29() {
//     let input = "http://[2001::.com.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_30() {
//     let input = "http://[::192.168.0.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_31() {
//     let input = "http://[::ffff:192.168.0.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_32() {
//     let input = "http://[1:2:3:4:5:6:7:8:9.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_33() {
//     let input = "http://[0:0:0:0:0:0:0:192.168.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_34() {
//     let input = "http://[1:2:3:4:5:6::192.168.0.1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_35() {
//     let input = "http://[1:2:3:4:5:6::8.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://[1:2:3:4:5:6:0:8.]/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_36() {
//     let input = "http://[1:2:3:4:5:6:7:8:.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_37() {
//     let input = "http://[1:2:3:4:5:6:192.168.0.1:.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_38() {
//     let input = "http://[-1:2:3:4:5:6:7:8.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_39() {
//     let input = "http://[1::%1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_40() {
//     let input = "http://[1::%eth0.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_41() {
//     let input = "http://[1::%.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_42() {
//     let input = "http://[%.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_43() {
//     let input = "http://[::%:.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_44() {
//     let input = "http://[:0:0::0:0:8.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_45() {
//     let input = "http://[0:0::0:0:8:.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_46() {
//     let input = "http://[:0:0::0:0:8:.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_47() {
//     let input = "http://[::192.168..1.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_48() {
//     let input = "http://[::1 hello.]/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://";
//     assert_eq!(&uri.normalize(), expect_uri);
// }
