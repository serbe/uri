// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = "http://www.example.com:as df/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com:as%20df/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_2() {
//     let input = "http://www.example.com:-2/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com:-2/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_3() {
//     let input = "http://www.example.com:80/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_4() {
//     let input = "http://www.example.com:8080/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com:8080/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_5() {
//     let input = "http://www.example.com:/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_6() {
//     let input = r#"http://www.example.com:\u1369/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com:%E1%8D%A9/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_7() {
//     let input = r#"http://www.example.com:\uD835\uDFD6/"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com:%F0%9D%9F%96/";
//     assert_eq!(&uri.normalize(), expect_uri);
// }
