use uri::Uri;

// // no path
// #[test]
// fn no_path() {
//     let s = "http://www.google.com";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     // ""
// }

// // path
// #[test]
// fn path() {
//     let s = "http://www.google.com/";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // path with hex escaping
// #[test]
// fn path_with_hex_escaping() {
//     let s = "http://www.google.com/file%20one%26two";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/file one&two"));
//     assert_eq!(u.path(), Some("/file%20one%26two"));
//     // ""
// }

// // fragment with hex escaping
// #[test]
// fn fragment_with_hex_escaping() {
//     let s = "http://www.google.com/#file%20one%26two";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     assert_eq!(u.fragment(), Some("file one&two"));
//     assert_eq!(u.fragment(), Some("file%20one%26two"));
//     // ""
// }

// // user
// #[test]
// fn user() {
//     let s = "ftp://webmaster@www.google.com/";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "ftp");
//     assert_eq!(u.username(), Some("webmaster"));
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // escape sequence in username
// #[test]
// fn escape_sequence_in_username() {
//     let s = "ftp://john%20doe@www.google.com/";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "ftp");
//     assert_eq!(u.username(), Some("john%20doe"));
//     assert_eq!(u.decode_username(), Some("john doe".to_string()));
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // "ftp://john%20doe@www.google.com/"
// }

// // empty query
// #[test]
// fn empty_query() {
//     let s = "http://www.google.com/?";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // ForceQuery: true,
//     // ""
// }

// // query ending in question mark (Issue 14573)
// #[test]
// fn query_ending_in_question_mark() {
//     let s = "http://www.google.com/?foo=bar?";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // RawQuery
//     assert_eq!(u.query(), Some("foo=bar?"));
//     // ""
// }

// // query
// #[test]
// fn query() {
//     let s = "http://www.google.com/?q=go+language";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // RawQuery
//     assert_eq!(u.query(), Some("q=go+language"));
//     // ""
// }

// // query with hex escaping: NOT parsed
// #[test]
// fn query_with_hex_escaping_not_parsed() {
//     let s = "http://www.google.com/?q=go%20language";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // RawQuery
//     assert_eq!(u.query(), Some("q=go%20language"));
//     // ""
// }

// // %20 outside query
// #[test]
// fn outside_query() {
//     let s = "http://www.google.com/a%20b?q=c+d";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/a b"));
//     // RawQuery
//     assert_eq!(u.query(), Some("q=c+d"));
//     // ""
// }

// // path without leading /, so no parsing
// #[test]
// fn path_without_leading_1() {
//     let s = "http:www.google.com/?q=go+language";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     // Opaque: "www.google.com/"
//     // RawQuery
//     assert_eq!(u.query(), Some("q=go+language"));
//     // "http:www.google.com/?q=go+language"
// }

// // path without leading /, so no parsing
// #[test]
// fn path_without_leading_2() {
//     let s = "http:%2f%2fwww.google.com/?q=go+language";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     // Opaque: "%2f%2fwww.google.com/"
//     // RawQuery
//     assert_eq!(u.query(), Some("q=go+language"));
//     // "http:%2f%2fwww.google.com/?q=go+language"
// }

// // non-authority with path; see golang.org/issue/46059
// #[test]
// fn non_authority_with_path() {
//     let s = "mailto:/webmaster@golang.org";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "mailto");
//     assert_eq!(u.path(), Some("/webmaster@golang.org"));
//     // OmitHost: true,
//     // ""
// }

// // non-authority
// #[test]
// fn non_authority() {
//     let s = "mailto:webmaster@golang.org";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "mailto");
//     // Opaque: "webmaster@golang.org"
//     // ""
// }

// // unescaped :// in query should not create a scheme
// #[test]
// fn unescaped_in_query_should_not_create_a_scheme() {
//     let s = "/foo?query=http://bad";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.path(), Some("/foo"));
//     // RawQuery
//     assert_eq!(u.query(), Some("query=http://bad"));
//     // ""
// }

// // leading // without scheme should create an authority
// #[test]
// fn leading_without_scheme_should_create_an_authority() {
//     let s = "//foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.host_str(), "foo");
//     // ""
// }

// // leading // without scheme, with userinfo, path, and query
// #[test]
// fn leading_without_scheme_with_userinfo_path_and_query() {
//     let s = "//user@foo/path?a=b";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.username(), Some("user"));
//     assert_eq!(u.host_str(), "foo");
//     assert_eq!(u.path(), Some("/path"));
//     // RawQuery
//     assert_eq!(u.query(), Some("a=b"));
//     // ""
// }

// // Three leading slashes isn't an authority, but doesn't return an error.
// // (We can't return an error, as this code is also used via
// // ServeHTTP -> ReadRequest -> Parse, which is arguably a
// // different URL parsing context, but currently shares the
// // same codepath)
// #[test]
// fn same_codepath_1() {
//     let s = "///threeslashes";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.path(), Some("///threeslashes"));
//     // ""
// }

// #[test]
// fn same_codepath_2() {
//     let s = "http://user:password@google.com";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.username(), Some("user"));
//     assert_eq!(u.password(), Some("password"));
//     assert_eq!(u.host_str(), "google.com");
//     // "http://user:password@google.com"
// }

// // unescaped @ in username should not confuse host
// #[test]
// fn unescaped_in_username_should_not_confuse_host() {
//     let s = "http://j@ne:password@google.com";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.username(), Some("j@ne"));
//     assert_eq!(u.password(), Some("password"));
//     assert_eq!(u.host_str(), "google.com");
//     // "http://j%40ne:password@google.com"
// }

// // unescaped @ in password should not confuse host
// #[test]
// fn unescaped_in_password_should_not_confuse_host_1() {
//     let s = "http://jane:p@ssword@google.com";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.username(), Some("jane"));
//     assert_eq!(u.password(), Some("p@ssword"));
//     assert_eq!(u.host_str(), "google.com");
//     // "http://jane:p%40ssword@google.com"
// }

// #[test]
// fn unescaped_in_password_should_not_confuse_host_2() {
//     let s = "http://j@ne:password@google.com/p@th?q=@go";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.username(), Some("j@ne"));
//     assert_eq!(u.password(), Some("password"));
//     assert_eq!(u.host_str(), "google.com");
//     assert_eq!(u.path(), Some("/p@th"));
//     // RawQuery
//     assert_eq!(u.query(), Some("q=@go"));
//     // "http://j%40ne:password@google.com/p@th?q=@go"
// }

// #[test]
// fn unescaped_in_password_should_not_confuse_host_3() {
//     let s = "http://www.google.com/?q=go+language#foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // RawQuery
//     assert_eq!(u.query(), Some("q=go+language"));
//     assert_eq!(u.fragment(), Some("foo"));
//     // ""
// }

// #[test]
// fn unescaped_in_password_should_not_confuse_host_4() {
//     let s = "http://www.google.com/?q=go+language#foo&bar";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // RawQuery
//     assert_eq!(u.query(), Some("q=go+language"));
//     assert_eq!(u.fragment(), Some("foo&bar"));
//     // "http://www.google.com/?q=go+language#foo&bar"
// }

// #[test]
// fn unescaped_in_password_should_not_confuse_host_5() {
//     let s = "http://www.google.com/?q=go+language#foo%26bar";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "www.google.com");
//     assert_eq!(u.path(), Some("/"));
//     // RawQuery
//     assert_eq!(u.query(), Some("q=go+language"));
//     // assert_eq!(u.fragment(), Some("foo&bar"));
//     assert_eq!(u.fragment(), Some("foo%26bar"));
//     // "http://www.google.com/?q=go+language#foo%26bar"
// }

// #[test]
// fn unescaped_in_password_should_not_confuse_host_6() {
//     let s = "file:///home/adg/rabbits";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "file");
//     assert_eq!(u.host_str(), "");
//     assert_eq!(u.path(), Some("/home/adg/rabbits"));
//     // "file:///home/adg/rabbits"
// }

// // "Windows" paths are no exception to the rule.
// // See golang.org/issue/6027, especially comment #9.
// #[test]
// fn windows_paths_are_no_exception_to_the_rule() {
//     let s = "file:///C:/FooBar/Baz.txt";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "file");
//     assert_eq!(u.host_str(), "");
//     assert_eq!(u.path(), Some("/C:/FooBar/Baz.txt"));
//     // "file:///C:/FooBar/Baz.txt"
// }

// // case-insensitive scheme
// #[test]
// fn case_insensitive_scheme() {
//     let s = "MaIlTo:webmaster@golang.org";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "mailto");
//     // Opaque: "webmaster@golang.org"
//     // "mailto:webmaster@golang.org"
// }

// // Relative path
// #[test]
// fn relative_path() {
//     let s = "a/b/c";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.path(), Some("a/b/c"));
//     // "a/b/c"
// }

// // escaped '?' in username and password
// #[test]
// fn escaped_in_username_and_password() {
//     let s = "http://%3Fam:pa%3Fsword@google.com";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.username(), Some("?am"));
//     assert_eq!(u.password(), Some("pa?sword"));
//     assert_eq!(u.host_str(), "google.com");
//     // ""
// }

// // host subcomponent; IPv4 address in RFC 3986
// #[test]
// fn host_subcomponent_ipv4_address_in_rfc_3986() {
//     let s = "http://192.168.0.1/";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "192.168.0.1");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // host and port subcomponents; IPv4 address in RFC 3986
// #[test]
// fn host_and_port_subcomponents_ipv4_address_in_rfc_3986() {
//     let s = "http://192.168.0.1:8080/";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "192.168.0.1:8080");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // host subcomponent; IPv6 address in RFC 3986
// #[test]
// fn host_subcomponent_ipv6_address_in_rfc_3986() {
//     let s = "http://[fe80::1]/";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "[fe80::1]");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // host and port subcomponents; IPv6 address in RFC 3986
// #[test]
// fn host_and_port_subcomponents_ipv6_address_in_rfc_3986() {
//     let s = "http://[fe80::1]:8080/";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "[fe80::1]:8080");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // host subcomponent; IPv6 address with zone identifier in RFC 6874
// #[test]
// fn host_subcomponent_ipv6_address_with_zone_identifier_in_rfc_6874() {
//     let s = "http://[fe80::1%25en0]/";
//     let u: Uri = s.parse().unwrap();
//     // alphanum zone identifier
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "[fe80::1%en0]");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // host and port subcomponents; IPv6 address with zone identifier in RFC 6874
// #[test]
// fn host_and_port_subcomponents_ipv6_address_with_zone_identifier_in_rfc_6874() {
//     let s = "http://[fe80::1%25en0]:8080/";
//     let u: Uri = s.parse().unwrap();
//     // alphanum zone identifier
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "[fe80::1%en0]:8080");
//     assert_eq!(u.path(), Some("/"));
//     // ""
// }

// // host subcomponent; IPv6 address with zone identifier in RFC 6874
// #[test]
// fn host_subcomponent_ipv6_address_with_zone_identifier_in_rfc_6874_2() {
//     let s = "http://[fe80::1%25%65%6e%301-._~]/";
//     let u: Uri = s.parse().unwrap();
//     // percent-encoded+unreserved zone identifier
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "[fe80::1%en01-._~]");
//     assert_eq!(u.path(), Some("/"));
//     // "http://[fe80::1%25en01-._~]/"
// }

// // host and port subcomponents; IPv6 address with zone identifier in RFC 6874
// #[test]
// fn host_and_port_subcomponents_ipv6_address_with_zone_identifier_in_rfc_6874_2() {
//     let s = "http://[fe80::1%25%65%6e%301-._~]:8080/";
//     let u: Uri = s.parse().unwrap();
//     // percent-encoded+unreserved zone identifier
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "[fe80::1%en01-._~]:8080");
//     assert_eq!(u.path(), Some("/"));
//     // "http://[fe80::1%25en01-._~]:8080/"
// }

// // alternate escapings of path survive round trip
// #[test]
// fn alternate_escapings_of_path_survive_round_trip() {
//     let s = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "rest.rsc.io");
//     assert_eq!(u.path(), Some("/foo%2fbar/baz%2Fquux"));
//     // RawQuery
//     assert_eq!(u.query(), Some("alt=media"));
//     // ""
// }

// // issue 12036
// #[test]
// fn issue_12036() {
//     let s = "mysql://a,b,c/bar";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "mysql");
//     assert_eq!(u.host_str(), "a,b,c");
//     assert_eq!(u.path(), Some("/bar"));
//     // ""
// }

// // worst case host, still round trips
// #[test]
// fn worst_case_host_still_round_trips() {
//     let s = "scheme://!$&'()*+,;=hello!:1/path";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "scheme");
//     assert_eq!(u.host_str(), "!$&'()*+,;=hello!:1");
//     assert_eq!(u.path(), Some("/path"));
//     // ""
// }

// // worst case path, still round trips
// #[test]
// fn worst_case_path_still_round_trips() {
//     let s = "http://host/!$&'()*+,;=:@[hello]";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "host");
//     assert_eq!(u.path(), Some("/!$&'()*+,;=:@[hello]"));
//     assert_eq!(u.path(), Some("/!$&'()*+,;=:@[hello]"));
//     // ""
// }

// // golang.org/issue/5684
// #[test]
// fn golang_org_issue_5684() {
//     let s = "http://example.com/oid/[order_id]";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "example.com");
//     assert_eq!(u.path(), Some("/oid/[order_id]"));
//     assert_eq!(u.path(), Some("/oid/[order_id]"));
//     // ""
// }

// // golang.org/issue/12200 (colon with empty port)
// #[test]
// fn golang_org_issue_12200_colon_with_empty_port_1() {
//     let s = "http://192.168.0.2:8080/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "192.168.0.2:8080");
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// #[test]
// fn golang_org_issue_12200_colon_with_empty_port_2() {
//     let s = "http://192.168.0.2:/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "192.168.0.2:");
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// // Malformed IPv6 but still accepted.
// #[test]
// fn malformed_ipv6_but_still_accepted() {
//     let s = "http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080");
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// // Malformed IPv6 but still accepted.
// #[test]
// fn malformed_ipv6_but_still_accepted_1() {
//     let s = "http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:");
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// #[test]
// fn malformed_ipv6_but_still_accepted_2() {
//     let s = "http://[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(
//         u.host_str(),
//         "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080"
//     );
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// #[test]
// fn malformed_ipv6_but_still_accepted_3() {
//     let s = "http://[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:");
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// // golang.org/issue/7991 and golang.org/issue/12719 (non-ascii %-encoded in host)
// #[test]
// fn golang_org_issue_7991_and_golang_org_issue_12719_non_ascii_encoded_in_host_1() {
//     let s = "http://hello.世界.com/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "hello.世界.com");
//     assert_eq!(u.path(), Some("/foo"));
//     // "http://hello.%E4%B8%96%E7%95%8C.com/foo"
// }

// #[test]
// fn golang_org_issue_7991_and_golang_org_issue_12719_non_ascii_encoded_in_host_2() {
//     let s = "http://hello.%e4%b8%96%e7%95%8c.com/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "hello.世界.com");
//     assert_eq!(u.path(), Some("/foo"));
//     // "http://hello.%E4%B8%96%E7%95%8C.com/foo"
// }

// #[test]
// fn golang_org_issue_7991_and_golang_org_issue_12719_non_ascii_encoded_in_host_3() {
//     let s = "http://hello.%E4%B8%96%E7%95%8C.com/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "hello.世界.com");
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// // golang.org/issue/10433 (path beginning with //)
// #[test]
// fn golang_org_issue_10433_path_beginning_with() {
//     let s = "http://example.com//foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "http");
//     assert_eq!(u.host_str(), "example.com");
//     assert_eq!(u.path(), Some("//foo"));
//     // ""
// }

// // test that we can reparse the host names we accept.
// #[test]
// fn test_that_we_can_reparse_the_host_names_we_accept() {
//     let s = "myscheme://authority<\"hi\">/foo";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "myscheme");
//     assert_eq!(u.host_str(), "authority<\"hi\">");
//     assert_eq!(u.path(), Some("/foo"));
//     // ""
// }

// // spaces in hosts are disallowed but escaped spaces in IPv6 scope IDs are grudgingly OK.
// // This happens on Windows.
// // golang.org/issue/14002
// #[test]
// fn golang_org_issue_14002() {
//     let s = "tcp://[2020::2020:20:2020:2020%25Windows%20Loves%20Spaces]:2020";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "tcp");
//     assert_eq!(
//         u.host_str(),
//         "[2020::2020:20:2020:2020%Windows Loves Spaces]:2020"
//     );
//     // ""
// }

// // test we can roundtrip magnet url
// // fix issue https://golang.org/issue/20054
// #[test]
// fn fix_issue_https_golang_org_issue_20054_1() {
//     let s = "magnet:?xt=urn:btih:c12fe1c06bba254a9dc9f519b335aa7c1367a88a&dn";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "magnet");
//     assert_eq!(u.host_str(), "");
//     assert_eq!(u.path(), Some(""));
//     // RawQuery
//     assert_eq!(
//         u.query(),
//         Some("xt=urn:btih:c12fe1c06bba254a9dc9f519b335aa7c1367a88a&dn")
//     );
//     // "magnet:?xt=urn:btih:c12fe1c06bba254a9dc9f519b335aa7c1367a88a&dn"
// }

// #[test]
// fn fix_issue_https_golang_org_issue_20054_2() {
//     let s = "mailto:?subject=hi";
//     let u: Uri = s.parse().unwrap();
//     assert_eq!(u.scheme(), "mailto");
//     assert_eq!(u.host_str(), "");
//     assert_eq!(u.path(), Some(""));
//     // RawQuery
//     assert_eq!(u.query(), Some("subject=hi"));
//     // "mailto:?subject=hi"
// }

#[test]
fn uri_() {
    let input = "stun://:443";
    let uri = Uri::try_from(input);
    dbg!(uri).unwrap();
    // assert!(uri.is_err());
}
