use std::convert::TryFrom;

use uri::Uri;

//
// #[test]
// fn test_1() {
//     let input = "http://user%40example.com/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     assert_eq!(&uri.normalize(), "http://user@example.com/");
//     assert_eq!(uri.scheme(), "http");
//     assert_eq!(uri.host_str(), "example.com");
//     assert_eq!(uri.port(), "80".parse::<u16>().ok());
//     assert_eq!(uri.path(), Some("/"));
//     assert_eq!(uri.query(), Some(""));
// }

//
// #[test]
// fn test_2() {
//     let input = "http://user%3Ainfo%40/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     assert_eq!(&uri.normalize(), "http://user:info@/");
//     assert_eq!(uri.scheme(), "http");
//     assert_eq!(uri.host_str(), "");
//     assert_eq!(uri.port(), "".parse::<u16>().ok());
//     assert_eq!(uri.path(), Some("/"));
//     assert_eq!(uri.query(), Some(""));
// }

//
// #[test]
// fn test_3() {
//     let input = "http://user@/";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     assert_eq!(&uri.normalize(), "http://user@/");
//     assert_eq!(uri.scheme(), "http");
//     assert_eq!(uri.host_str(), "");
//     assert_eq!(uri.port(), "".parse::<u16>().ok());
//     assert_eq!(uri.path(), Some("/"));
//     assert_eq!(uri.query(), Some(""));
// }

//
// #[test]
// fn test_4() {
//     let input = "$:foo/bar";
//     let uri: Uri = Uri::try_from(input).unwrap();
//     assert_eq!(&uri.normalize(), "$:");
//     assert_eq!(uri.scheme(), "$");
//     assert_eq!(uri.host_str(), "foo");
//     assert_eq!(uri.port(), "".parse::<u16>().ok());
//     assert_eq!(uri.path(), Some("bar"));
//     assert_eq!(uri.query(), Some(""));
// }

// Two colons after mailto scheme
#[test]
fn test_5() {
    let input = "mailto::foo@bar.com";
    let uri: Uri = Uri::try_from(input).unwrap();
    assert_eq!(&uri.normalize(), "mailto::foo@bar.com");
    assert_eq!(uri.scheme(), "mailto");
    assert_eq!(uri.host_str(), "");
    assert_eq!(uri.port(), None);
    assert_eq!(uri.path(), Some(":foo@bar.com"));
    assert_eq!(uri.query(), None);
}
