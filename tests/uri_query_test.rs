// use std::convert::TryFrom;

// use uri::Uri;

// //
// #[test]
// fn test_1() {
//     let input = "http://www.example.com/?foo=bar";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?foo=bar";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_2() {
//     let input = "http://www.example.com/?as?df";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?as?df";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_3() {
//     let input = "http://www.example.com/?%02hello%7f bye";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?%02hello%7F%20bye";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_4() {
//     let input = "http://www.example.com/?%40%41123";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?%40%41123";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_5() {
//     let input = r#"http://www.example.com/?q=\u4F60\u597D"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?q=%26%2320320%3B%26%2322909%3B";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_6() {
//     let input = r#"http://www.example.com/?q=\\\ud800\\\ud800"#;
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?q=%26%2355296%3B%26%2355296%3B";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_7() {
//     let input = "http://www.example.com/?q=&lt;asdf&gt;";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?q=%3Casdf%3E";
//     assert_eq!(&uri.normalize(), expect_uri);
// }

// //
// #[test]
// fn test_8() {
//     let input = "http://www.example.com/?q=\"asdf\"";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     let expect_uri = "http://www.example.com/?q=%22asdf%22";
//     assert_eq!(&uri.normalize(), expect_uri);
// }
