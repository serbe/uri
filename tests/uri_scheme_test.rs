use std::convert::TryFrom;

use uri::Uri;

#[test]
fn test_scheme_case() {
    let input = "HTTP://example.com/";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_uri = "http://example.com/";
    assert_eq!(&uri.normalize(), expect_uri);
}

#[test]
fn test_scheme_ftp() {
    let input = "ftp://ftp.is.co.za/rfc/rfc1808.txt";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "ftp";
    assert_eq!(uri.scheme(), expect_scheme);
}

#[test]
fn test_scheme_http() {
    let input = "http://www.ietf.org/rfc/rfc2396.txt";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "http";
    assert_eq!(uri.scheme(), expect_scheme);
}

#[test]
fn test_scheme_ldap() {
    let input = "ldap://[2001:db8::7]/c=GB?objectClass?one";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "ldap";
    assert_eq!(uri.scheme(), expect_scheme);
}

#[test]
fn test_scheme_mailto() {
    let input = "mailto:John.Doe@example.com";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "mailto";
    assert_eq!(uri.scheme(), expect_scheme);
}

#[test]
fn test_scheme_news() {
    let input = "news:comp.infosystems.www.servers.unix";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "news";
    assert_eq!(uri.scheme(), expect_scheme);
}

#[test]
fn test_scheme_tel() {
    let input = "tel:+1-816-555-1212";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "tel";
    assert_eq!(uri.scheme(), expect_scheme);
}

#[test]
fn test_scheme_telnet() {
    let input = "telnet://192.0.2.16:80/";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "telnet";
    assert_eq!(uri.scheme(), expect_scheme);
}

#[test]
fn test_scheme_urn() {
    let input = "urn:oasis:names:specification:docbook:dtd:xml:4.1.2";
    let uri: Uri = Uri::try_from(input).unwrap();
    let expect_scheme = "urn";
    assert_eq!(uri.scheme(), expect_scheme);
}
