use std::convert::TryFrom;

use uri::Uri;

#[test]
fn parse_ipv4() {
    let socks = "socks5://127.0.0.1:5959";
    let http = "http://api.ipify.org";
    assert!(socks.parse::<Uri>().is_ok());
    assert!(http.parse::<Uri>().is_ok());
}

#[test]
fn domain_addr() {
    let http = "http://api.ipify.org".parse::<Uri>().unwrap();
    assert_eq!(http.host(), Some("api.ipify.org"));
    assert!(http.addr().is_ok());
}

#[test]
fn ipv4_addr() {
    let socks = "socks5://127.0.0.1:5959".parse::<Uri>().unwrap();
    assert_eq!(socks.host(), Some("127.0.0.1"));
    assert!(socks.addr().is_ok());
}

#[test]
fn get_host_str() {
    let http = "http://api.ipify.org".parse::<Uri>().unwrap();
    assert_eq!(http.host_str(), "api.ipify.org");
}

#[test]
fn get_host_with_port() {
    let http = "http://api.ipify.org".parse::<Uri>().unwrap();
    assert_eq!(http.host_with_port(), Some("api.ipify.org:80".to_string()));

    let http = "http://api.ipify.org:1245".parse::<Uri>().unwrap();
    assert_eq!(
        http.host_with_port(),
        Some("api.ipify.org:1245".to_string())
    );
}

#[test]
fn try_from_str() {
    let input = "http://www.example.org";
    let uri: Uri = Uri::try_from(input).unwrap();
    assert_eq!(uri.as_str(), input);
}

#[test]
fn try_from_string() {
    let input = "http://www.example.org".to_string();
    let uri: Uri = Uri::try_from(input.clone()).unwrap();
    assert_eq!(uri.as_str(), input.as_str());
}

#[test]
fn try_from_string_ref() {
    let input = &"http://www.example.org".to_string();
    let uri: Uri = Uri::try_from(input).unwrap();
    assert_eq!(uri.as_str(), input.as_str());
}

#[test]
fn as_str_t1() {
    let uri = "hTtP://www.example.org".parse::<Uri>().unwrap();
    assert_eq!(uri.as_str(), "http://www.example.org");
}

#[test]
fn scheme_t1() {
    let uri = "hTtP://www.example.org".parse::<Uri>().unwrap();
    assert_eq!(uri.scheme(), "http");
}

#[test]
fn scheme_t2() {
    let uri = "http123s://www.example.org".parse::<Uri>().unwrap();
    assert_eq!(uri.scheme(), "http123s");
}

#[test]
fn user_info_t1() {
    let uri = "ftp://webmaster@www.example.org/".parse::<Uri>().unwrap();
    assert_eq!(uri.user_info(), Some("webmaster"));
}

#[test]
fn user_info_t2() {
    let uri = "ftp://john%20doe@www.example.org/".parse::<Uri>().unwrap();
    assert_eq!(uri.user_info(), Some("john%20doe"));
}

#[test]
fn user_info_t3() {
    let uri = "http://%3Fam:pa%3Fsword@google.com".parse::<Uri>().unwrap();
    assert_eq!(uri.user_info(), Some("%3Fam:pa%3Fsword"));
    assert_eq!(uri.decode_username(), Some("?am".to_owned()));
    assert_eq!(uri.decode_password(), Some("pa?sword".to_owned()));
}

#[test]
fn host_t1() {
    let uri = "http://www.example.org".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("www.example.org"));
}

#[test]
fn host_t2() {
    let uri = "ftp://webmaster@www.example.org/".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("www.example.org"));
}

#[test]
fn host_t3() {
    let uri = "http://%3Fam:pa%3Fsword@google.com".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("google.com"));
}

#[test]
fn host_t4() {
    let uri = "http://192.168.0.1:8080/".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("192.168.0.1"));
}

#[test]
fn host_t4_1() {
    let uri = "http://192.168.0.1/".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("192.168.0.1"));
}

#[test]
fn host_t5() {
    let uri = "http://[fe80::1]:8080/".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("[fe80::1]"));
}

#[test]
fn host_t5_1() {
    let uri = "http://[fe80::1]/".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("[fe80::1]"));
}

#[test]
fn host_t6() {
    let uri = "mysql://a,b,c/bar".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("a,b,c"));
}

#[test]
fn host_t7() {
    let uri = "scheme://!$&'()*+,;=hello!:23/path".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("!$&'()*+,;=hello!"));
}

#[test]
fn host_t8() {
    let uri = "myscheme://authority<\"hi\">/foo".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("authority<\"hi\">"));
}

#[test]
fn host_t9() {
    let uri = "http://hello.世界.com/foo".parse::<Uri>().unwrap();
    assert_eq!(uri.host(), Some("hello.世界.com"));
}

#[test]
fn host_header_t1() {
    let uri = "http://www.example.org:8080".parse::<Uri>().unwrap();
    assert_eq!(uri.host_header(), "www.example.org:8080");
}

#[test]
fn host_header_t2() {
    let uri = "http://www.example.org:443".parse::<Uri>().unwrap();
    assert_eq!(uri.host_header(), "www.example.org:443");
}

#[test]
fn port_t1() {
    let uri = "http://192.168.0.1:8080/".parse::<Uri>().unwrap();
    assert_eq!(uri.port(), Some(8080));
}

#[test]
fn port_t2() {
    let uri = "http://[fe80::1]:8080/".parse::<Uri>().unwrap();
    assert_eq!(uri.port(), Some(8080));
}

#[test]
fn port_t3() {
    let uri = "scheme://!$&'()*+,;=hello!:23/path".parse::<Uri>().unwrap();
    assert_eq!(uri.port(), Some(23));
}

#[test]
fn default_port_t1() {
    let uri = "http://www.example.org:443".parse::<Uri>().unwrap();
    assert_eq!(uri.default_port(), Some(80));
}

#[test]
fn default_port_t2() {
    let uri = "https://www.example.org".parse::<Uri>().unwrap();
    assert_eq!(uri.default_port(), Some(443));
}

#[test]
fn default_port_t3() {
    let uri = "sptth://www.example.org".parse::<Uri>().unwrap();
    assert_eq!(uri.default_port(), None);
}

#[test]
fn path_t1() {
    let uri = "http://www.example.org/".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("/"));
}

#[test]
fn path_t2() {
    let uri = "http://www.example.org/file%20one%26two"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.path(), Some("/file%20one%26two"));
    assert_eq!(uri.decode_path(), Some("/file one&two".to_owned()));
}

#[test]
fn path_t3() {
    let uri = "ftp://john%20doe@www.example.org/".parse::<Uri>().unwrap();
    assert_eq!(uri.user_info(), Some("john%20doe"));
}

#[test]
fn path_t4() {
    let uri = "http://www.example.org/?".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("/"));
}

#[test]
fn path_t5() {
    let uri = "http://www.example.org/a%20b?q=c+d".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("/a%20b"));
}

#[test]
fn path_t6() {
    let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.path(), Some("/foo%2fbar/baz%2Fquux"));
    assert_eq!(uri.decode_path(), Some("/foo/bar/baz/quux".to_owned()));
}

#[test]
fn path_t7() {
    let uri = "mysql://a,b,c/bar".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("/bar"));
}

#[test]
fn path_t8() {
    let uri = "scheme://!$&'()*+,;=hello!:23/path".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("/path"));
}

#[test]
fn path_t9() {
    let uri = "http://host/!$&'()*+,;=:@[hello]".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("/!$&'()*+,;=:@[hello]"));
    // Rawu.path = Some("/!$&'()*+,;=:@[hello]");
}

#[test]
fn path_t10() {
    let uri = "http://example.com/oid/[order_id]".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("/oid/[order_id]"));
    // Rawu.path = Some("/oid/[order_id]");
}

#[test]
fn path_t11() {
    let uri = "http://example.com//foo".parse::<Uri>().unwrap();
    assert_eq!(uri.path(), Some("//foo"));
}

#[test]
fn query_t1() {
    let uri = "http://www.example.org/?".parse::<Uri>().unwrap();
    assert_eq!(uri.query(), Some(""));
}

#[test]
fn query_t2() {
    let uri = "http://www.example.org/?foo=bar?".parse::<Uri>().unwrap();
    assert_eq!(uri.query(), Some("foo=bar?"));
}

#[test]
fn query_t3() {
    let uri = "http://www.example.org/?q=rust+language"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.query(), Some("q=rust+language"));
}

#[test]
fn query_t4() {
    let uri = "http://www.example.org/?q=go%20language"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.query(), Some("q=go%20language"));
}

#[test]
fn query_t5() {
    let uri = "http://www.example.org/a%20b?q=c+d".parse::<Uri>().unwrap();
    assert_eq!(uri.query(), Some("q=c+d"));
    assert_eq!(uri.decode_path(), Some("/a b".to_owned()));
}

#[test]
fn query_t6() {
    let uri = "http://www.example.org/?q=rust+language"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.query(), Some("q=rust+language"));
}

#[test]
fn query_t7() {
    let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.query(), Some("alt=media"));
}

#[test]
fn fragment_t1() {
    let uri = "http://www.example.org/foo.html#bar"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.fragment(), Some("bar"));
}

#[test]
fn fragment_t2() {
    let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
    assert_eq!(uri.fragment(), None);
}

#[test]
fn request_uri_t1() {
    let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.abs_path(), "/foo%2fbar/baz%2Fquux?alt=media");
}

#[test]
fn proxy_request_uri() {
    let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(
        uri.absolute_uri(),
        "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
    );
}

#[test]
fn origin_t1() {
    let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
    assert_eq!(uri.origin(), Some("http://www.example.org".to_string()));
}

#[test]
fn host_port_t1() {
    let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
    assert_eq!(uri.host_port(), Some("www.example.org"));
}

#[test]
fn addr_port_t1() {
    let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
    assert_eq!(uri.default_port(), Some(80u16));
}

#[test]
fn path_without_leading() {
    let uri = "http:%2f%2fwww.example.org/?q=rust+language"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.scheme(), "http");
    // Opaque:   "%2f%2fwww.example.org/",
    assert_eq!(uri.query(), Some("q=rust+language"));
}

// #[test]
// fn host_subcomponent3() {
//             let uri = "http://[fe80::1%25en0]/".parse::<Uri>().unwrap();
//     assert_eq!(uri.scheme(), "http");
//     u.host = "[fe80::1%en0]";
//     assert_eq!(uri.path(), Some("/"));
//         }

// #[test]
// fn host_and_port_subcomponents3() {
//     let uri = "http://[fe80::1%25en0]:8080/".parse::<Uri>().unwrap();
//     assert_eq!(uri.scheme(), "http");
//     assert_eq!(uri.host(), Some("[fe80::1%en0]"));
//     assert_eq!(uri.port(), Some(8080));
//     assert_eq!(uri.path(), Some("/"));
// }

// #[test]
// fn host_subcomponent4() {
//         //     let uri = "http:[fe80::1%25%65%6e%301-._~]/".parse::<Uri>().unwrap();
//     assert_eq!(uri.scheme(), "http");
//     u.host = "[fe80::1%en01-._~]";
//     assert_eq!(uri.path(), Some("/"));
//         // }

// #[test]
// fn host_and_port_subcomponents4() {
//             let uri = "http:[fe80::1%25%65%6e%301-._~]:8080/".parse::<Uri>().unwrap();
//     assert_eq!(uri.scheme(), "http");
//     assert_eq!(uri.host(), Some("[fe80::1%25%65%6e%301-._~]"));
//     // assert_eq!(uri.host(), Some("[fe80::1%en01-._~]"));
//     assert_eq!(uri.port(), Some(8080));
//     assert_eq!(uri.path(), Some("/"));
//         }

// #[test]
// fn host_subcomponent4() {
//     let uri = "http://192.168.0.2:/foo".parse::<Uri>().unwrap();
//     assert_eq!(uri.scheme(), "http");
//     assert_eq!(uri.host(), Some("192.168.0.2:"));
//     assert_eq!(uri.path(), Some("/foo"));
// }

//          //      	 Malformed IPv6 but still accepted.
//      let uri = "http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080/foo".parse::<Uri>().unwrap();
//      		assert_eq!(uri.scheme(), "http");
//      		u.host = "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080";
//      		u.path = Some("/foo");
//          // }

//          //      	 Malformed IPv6 but still accepted.
//      let uri = "http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:/foo".parse::<Uri>().unwrap();
//      		assert_eq!(uri.scheme(), "http");
//      		u.host = "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:";
//      		u.path = Some("/foo");
//          // }

// #[test]
// fn ipv6_2() {
//          //      let uri = "http:[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080/foo".parse::<Uri>().unwrap();
//      		assert_eq!(uri.scheme(), "http");
//      		assert_eq!(uri.host(), Some("[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080"));
//      		assert_eq!(uri.path(), Some("/foo"));
//          // }

//          //      let uri = "http:[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:/foo".parse::<Uri>().unwrap();
//      		assert_eq!(uri.scheme(), "http");
//      		u.host = "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:";
//      		u.path = Some("/foo");
//          // }

//          //      let uri = "http://hello.%e4%b8%96%e7%95%8c.com/foo".parse::<Uri>().unwrap();
//      		assert_eq!(uri.scheme(), "http");
//      		u.host = "hello.世界.com";
//      		u.path = Some("/foo");
//          //      let uri = "http://hello.%E4%B8%96%E7%95%8C.com/foo".parse::<Uri>().unwrap();
//      }

//          //      let uri = "http://hello.%E4%B8%96%E7%95%8C.com/foo".parse::<Uri>().unwrap();
//      		assert_eq!(uri.scheme(), "http");
//      		u.host = "hello.世界.com";
//      		u.path = Some("/foo");
//          // }

// #[test]
// fn example5() {
//         //     let uri = "tcp:[2020::2020:20:2020:2020%25Windows%20Loves%20Spaces]:2020".parse::<Uri>().unwrap();
//     u.scheme = Some("tcp");
//     u.host = "[2020::2020:20:2020:2020%Windows Loves Spaces]:2020";
//         // }
