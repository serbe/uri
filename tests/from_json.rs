// //   "# Pulled from https://github.com/web-platform-tests/wpt/blob/befe66343e5f21dc464c8c772c6d20695936714f/url/resources/urltestdata.json",
// #[test]
// fn uri_() {
//     "input": "http://example\t.\norg",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://user:pass@foo:21/bar;par?b#c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://user:pass@foo:21/bar;par?b#c",
//     "origin": "http://foo:21",
//     "protocol": "http:",
//     "username": "user",
//     "password": "pass",
//     "host": "foo:21",
//     "hostname": "foo",
//     "port": "21",
//     "pathname": "/bar;par",
//     "search": "?b",
//     "hash": "#c"
// }

// #[test]
// fn uri_() {
//     "input": "https://test:@test",
//     "base": null,
//     "href": "https://test@test/",
//     "origin": "https://test",
//     "protocol": "https:",
//     "username": "test",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://:@test",
//     "base": null,
//     "href": "https://test/",
//     "origin": "https://test",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special://test:@test/x",
//     "base": null,
//     "href": "non-special://test@test/x",
//     "origin": "null",
//     "protocol": "non-special:",
//     "username": "test",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/x",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special://:@test/x",
//     "base": null,
//     "href": "non-special://test/x",
//     "origin": "null",
//     "protocol": "non-special:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/x",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:foo.com",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/foo.com",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/foo.com",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\t   :foo.com   \n",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:foo.com",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:foo.com",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": " foo.com  ",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/foo.com",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/foo.com",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "a:\t foo.com",
//     "base": "http://example.org/foo/bar",
//     "href": "a: foo.com",
//     "origin": "null",
//     "protocol": "a:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": " foo.com",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://f:21/ b ? d # e ",
//     "base": "http://example.org/foo/bar",
//     "href": "http://f:21/%20b%20?%20d%20#%20e",
//     "origin": "http://f:21",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "f:21",
//     "hostname": "f",
//     "port": "21",
//     "pathname": "/%20b%20",
//     "search": "?%20d%20",
//     "hash": "#%20e"
// }

// #[test]
// fn uri_() {
//     "input": "lolscheme:x x#x x",
//     "base": null,
//     "href": "lolscheme:x x#x%20x",
//     "protocol": "lolscheme:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "x x",
//     "search": "",
//     "hash": "#x%20x"
// }

// #[test]
// fn uri_() {
//     "input": "http://f:/c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://f/c",
//     "origin": "http://f",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "f",
//     "hostname": "f",
//     "port": "",
//     "pathname": "/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://f:0/c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://f:0/c",
//     "origin": "http://f:0",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "f:0",
//     "hostname": "f",
//     "port": "0",
//     "pathname": "/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://f:00000000000000/c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://f:0/c",
//     "origin": "http://f:0",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "f:0",
//     "hostname": "f",
//     "port": "0",
//     "pathname": "/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://f:00000000000000000000080/c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://f/c",
//     "origin": "http://f",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "f",
//     "hostname": "f",
//     "port": "",
//     "pathname": "/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://f:b/c",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://f: /c",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://f:\n/c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://f/c",
//     "origin": "http://f",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "f",
//     "hostname": "f",
//     "port": "",
//     "pathname": "/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://f:fifty-two/c",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://f:999999/c",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "non-special://f:999999/c",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://f: 21 / b ? d # e ",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "  \t",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":foo.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:foo.com/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:foo.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":foo.com\\",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:foo.com/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:foo.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":a",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:a",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:a",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":\\",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":#",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:#",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "#",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar#",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "#/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar#/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": "#/"
// }

// #[test]
// fn uri_() {
//     "input": "#\\",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar#\\",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": "#\\"
// }

// #[test]
// fn uri_() {
//     "input": "#;?",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar#;?",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": "#;?"
// }

// #[test]
// fn uri_() {
//     "input": "?",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar?",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ":23",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:23",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:23",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/:23",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/:23",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/:23",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\\x",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/x",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/x",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\\\\x\\hello",
//     "base": "http://example.org/foo/bar",
//     "href": "http://x/hello",
//     "origin": "http://x",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "x",
//     "hostname": "x",
//     "port": "",
//     "pathname": "/hello",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "::",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/::",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/::",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "::23",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/::23",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/::23",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://",
//     "base": "http://example.org/foo/bar",
//     "href": "foo://",
//     "origin": "null",
//     "protocol": "foo:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://a:b@c:29/d",
//     "base": "http://example.org/foo/bar",
//     "href": "http://a:b@c:29/d",
//     "origin": "http://c:29",
//     "protocol": "http:",
//     "username": "a",
//     "password": "b",
//     "host": "c:29",
//     "hostname": "c",
//     "port": "29",
//     "pathname": "/d",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http::@c:29",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/:@c:29",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/:@c:29",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://&a:foo(b]c@d:2/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://&a:foo(b%5Dc@d:2/",
//     "origin": "http://d:2",
//     "protocol": "http:",
//     "username": "&a",
//     "password": "foo(b%5Dc",
//     "host": "d:2",
//     "hostname": "d",
//     "port": "2",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://::@c@d:2",
//     "base": "http://example.org/foo/bar",
//     "href": "http://:%3A%40c@d:2/",
//     "origin": "http://d:2",
//     "protocol": "http:",
//     "username": "",
//     "password": "%3A%40c",
//     "host": "d:2",
//     "hostname": "d",
//     "port": "2",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://foo.com:b@d/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://foo.com:b@d/",
//     "origin": "http://d",
//     "protocol": "http:",
//     "username": "foo.com",
//     "password": "b",
//     "host": "d",
//     "hostname": "d",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://foo.com/\\@",
//     "base": "http://example.org/foo/bar",
//     "href": "http://foo.com//@",
//     "origin": "http://foo.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo.com",
//     "hostname": "foo.com",
//     "port": "",
//     "pathname": "//@",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:\\\\foo.com\\",
//     "base": "http://example.org/foo/bar",
//     "href": "http://foo.com/",
//     "origin": "http://foo.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo.com",
//     "hostname": "foo.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:\\\\a\\b:c\\d@foo.com\\",
//     "base": "http://example.org/foo/bar",
//     "href": "http://a/b:c/d@foo.com/",
//     "origin": "http://a",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "a",
//     "hostname": "a",
//     "port": "",
//     "pathname": "/b:c/d@foo.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo:/",
//     "base": "http://example.org/foo/bar",
//     "href": "foo:/",
//     "origin": "null",
//     "protocol": "foo:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo:/bar.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "foo:/bar.com/",
//     "origin": "null",
//     "protocol": "foo:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/bar.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://///////",
//     "base": "http://example.org/foo/bar",
//     "href": "foo://///////",
//     "origin": "null",
//     "protocol": "foo:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "///////",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://///////bar.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "foo://///////bar.com/",
//     "origin": "null",
//     "protocol": "foo:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "///////bar.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo:////://///",
//     "base": "http://example.org/foo/bar",
//     "href": "foo:////://///",
//     "origin": "null",
//     "protocol": "foo:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//://///",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "c:/foo",
//     "base": "http://example.org/foo/bar",
//     "href": "c:/foo",
//     "origin": "null",
//     "protocol": "c:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//foo/bar",
//     "base": "http://example.org/foo/bar",
//     "href": "http://foo/bar",
//     "origin": "http://foo",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://foo/path;a??e#f#g",
//     "base": "http://example.org/foo/bar",
//     "href": "http://foo/path;a??e#f#g",
//     "origin": "http://foo",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/path;a",
//     "search": "??e",
//     "hash": "#f#g"
// }

// #[test]
// fn uri_() {
//     "input": "http://foo/abcd?efgh?ijkl",
//     "base": "http://example.org/foo/bar",
//     "href": "http://foo/abcd?efgh?ijkl",
//     "origin": "http://foo",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/abcd",
//     "search": "?efgh?ijkl",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://foo/abcd#foo?bar",
//     "base": "http://example.org/foo/bar",
//     "href": "http://foo/abcd#foo?bar",
//     "origin": "http://foo",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/abcd",
//     "search": "",
//     "hash": "#foo?bar"
// }

// #[test]
// fn uri_() {
//     "input": "[61:24:74]:98",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/[61:24:74]:98",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/[61:24:74]:98",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:[61:27]/:foo",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/[61:27]/:foo",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/[61:27]/:foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://[1::2]:3:4",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://2001::1",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://2001::1]",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://2001::1]:80",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[2001::1]",
//     "base": "http://example.org/foo/bar",
//     "href": "http://[2001::1]/",
//     "origin": "http://[2001::1]",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "[2001::1]",
//     "hostname": "[2001::1]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://[::127.0.0.1]",
//     "base": "http://example.org/foo/bar",
//     "href": "http://[::7f00:1]/",
//     "origin": "http://[::7f00:1]",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "[::7f00:1]",
//     "hostname": "[::7f00:1]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://[::127.0.0.1.]",
//     "base": "http://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[0:0:0:0:0:0:13.1.68.3]",
//     "base": "http://example.org/foo/bar",
//     "href": "http://[::d01:4403]/",
//     "origin": "http://[::d01:4403]",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "[::d01:4403]",
//     "hostname": "[::d01:4403]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://[2001::1]:80",
//     "base": "http://example.org/foo/bar",
//     "href": "http://[2001::1]/",
//     "origin": "http://[2001::1]",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "[2001::1]",
//     "hostname": "[2001::1]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/example.com/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftp:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "ftp://example.com/",
//     "origin": "ftp://example.com",
//     "protocol": "ftp:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "https://example.com/",
//     "origin": "https://example.com",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "madeupscheme:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "madeupscheme:/example.com/",
//     "origin": "null",
//     "protocol": "madeupscheme:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "file:///example.com/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "file://example:1/";
// let uri: Uri = Uri::try_from(input);
// assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://example:test/";
// let uri: Uri = Uri::try_from(input);
// assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://example%/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://[example]/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "ftps:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "ftps:/example.com/",
//     "origin": "null",
//     "protocol": "ftps:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "gopher:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "gopher:/example.com/",
//     "origin": "null",
//     "protocol": "gopher:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "ws://example.com/",
//     "origin": "ws://example.com",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "wss://example.com/",
//     "origin": "wss://example.com",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "data:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "data:/example.com/",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "javascript:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "javascript:/example.com/",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "mailto:/example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "mailto:/example.com/",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/example.com/",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftp:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "ftp://example.com/",
//     "origin": "ftp://example.com",
//     "protocol": "ftp:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "https://example.com/",
//     "origin": "https://example.com",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "madeupscheme:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "madeupscheme:example.com/",
//     "origin": "null",
//     "protocol": "madeupscheme:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftps:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "ftps:example.com/",
//     "origin": "null",
//     "protocol": "ftps:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "gopher:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "gopher:example.com/",
//     "origin": "null",
//     "protocol": "gopher:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "ws://example.com/",
//     "origin": "ws://example.com",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "wss://example.com/",
//     "origin": "wss://example.com",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "data:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "data:example.com/",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "javascript:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "javascript:example.com/",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "mailto:example.com/",
//     "base": "http://example.org/foo/bar",
//     "href": "mailto:example.com/",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/a/b/c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/a/b/c",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/a/b/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/a/ /c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/a/%20/c",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/a/%20/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/a%2fc",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/a%2fc",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/a%2fc",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/a/%2f/c",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/a/%2f/c",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/a/%2f/c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "#β",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar#%CE%B2",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": "#%CE%B2"
// }

// #[test]
// fn uri_() {
//     "input": "data:text/html,test#test",
//     "base": "http://example.org/foo/bar",
//     "href": "data:text/html,test#test",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "text/html,test",
//     "search": "",
//     "hash": "#test"
// }

// #[test]
// fn uri_() {
//     "input": "tel:1234567890",
//     "base": "http://example.org/foo/bar",
//     "href": "tel:1234567890",
//     "origin": "null",
//     "protocol": "tel:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "1234567890",
//     "search": "",
//     "hash": ""
// }

//   "# Based on https://felixfbecker.github.io/whatwg-url-custom-host-repro/",
// #[test]
// fn uri_() {
//     "input": "ssh://example.com/foo/bar.git",
//     "base": "http://example.org/",
//     "href": "ssh://example.com/foo/bar.git",
//     "origin": "null",
//     "protocol": "ssh:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/bar.git",
//     "search": "",
//     "hash": ""
// }

//   "# Based on http://trac.webkit.org/browser/trunk/LayoutTests/fast/url/file.html",
// #[test]
// fn uri_() {
//     "input": "file:c:\\foo\\bar.html",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///c:/foo/bar.html",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/c:/foo/bar.html",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "  File:c|////foo\\bar.html",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///c:////foo/bar.html",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/c:////foo/bar.html",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|/foo/bar",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///C:/foo/bar",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/C|\\foo\\bar",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///C:/foo/bar",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//C|/foo/bar",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///C:/foo/bar",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//server/file",
//     "base": "file:///tmp/mock/path",
//     "href": "file://server/file",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "server",
//     "hostname": "server",
//     "port": "",
//     "pathname": "/file",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\\\\server\\file",
//     "base": "file:///tmp/mock/path",
//     "href": "file://server/file",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "server",
//     "hostname": "server",
//     "port": "",
//     "pathname": "/file",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/\\server/file",
//     "base": "file:///tmp/mock/path",
//     "href": "file://server/file",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "server",
//     "hostname": "server",
//     "port": "",
//     "pathname": "/file",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///foo/bar.txt",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///foo/bar.txt",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/foo/bar.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///home/me",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///home/me",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/home/me",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "///",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "///test",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///test",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://test",
//     "base": "file:///tmp/mock/path",
//     "href": "file://test/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://localhost",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://localhost/",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://localhost/test",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///test",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "test",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///tmp/mock/test",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/tmp/mock/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:test",
//     "base": "file:///tmp/mock/path",
//     "href": "file:///tmp/mock/test",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/tmp/mock/test",
//     "search": "",
//     "hash": ""
// }

//   "# Based on http://trac.webkit.org/browser/trunk/LayoutTests/fast/url/script-tests/path.js",
// #[test]
// fn uri_() {
//     "input": "http://example.com/././foo",
//     "base": null,
//     "href": "http://example.com/foo",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/./.foo",
//     "base": null,
//     "href": "http://example.com/.foo",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/.foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/.",
//     "base": null,
//     "href": "http://example.com/foo/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/./",
//     "base": null,
//     "href": "http://example.com/foo/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/bar/..",
//     "base": null,
//     "href": "http://example.com/foo/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/bar/../",
//     "base": null,
//     "href": "http://example.com/foo/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/..bar",
//     "base": null,
//     "href": "http://example.com/foo/..bar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/..bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/bar/../ton",
//     "base": null,
//     "href": "http://example.com/foo/ton",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/ton",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/bar/../ton/../../a",
//     "base": null,
//     "href": "http://example.com/a",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/a",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/../../..",
//     "base": null,
//     "href": "http://example.com/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/../../../ton",
//     "base": null,
//     "href": "http://example.com/ton",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/ton",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/%2e",
//     "base": null,
//     "href": "http://example.com/foo/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/%2e%2",
//     "base": null,
//     "href": "http://example.com/foo/%2e%2",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/%2e%2",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/%2e./%2e%2e/.%2e/%2e.bar",
//     "base": null,
//     "href": "http://example.com/%2e.bar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%2e.bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com////../..",
//     "base": null,
//     "href": "http://example.com//",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/bar//../..",
//     "base": null,
//     "href": "http://example.com/foo/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo/bar//..",
//     "base": null,
//     "href": "http://example.com/foo/bar/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo/bar/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo",
//     "base": null,
//     "href": "http://example.com/foo",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/%20foo",
//     "base": null,
//     "href": "http://example.com/%20foo",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%20foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo%",
//     "base": null,
//     "href": "http://example.com/foo%",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo%",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo%2",
//     "base": null,
//     "href": "http://example.com/foo%2",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo%2",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo%2zbar",
//     "base": null,
//     "href": "http://example.com/foo%2zbar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo%2zbar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo%2Â©zbar",
//     "base": null,
//     "href": "http://example.com/foo%2%C3%82%C2%A9zbar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo%2%C3%82%C2%A9zbar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo%41%7a",
//     "base": null,
//     "href": "http://example.com/foo%41%7a",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo%41%7a",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo\t\u0091%91",
//     "base": null,
//     "href": "http://example.com/foo%C2%91%91",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo%C2%91%91",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo%00%51",
//     "base": null,
//     "href": "http://example.com/foo%00%51",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foo%00%51",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/(%28:%3A%29)",
//     "base": null,
//     "href": "http://example.com/(%28:%3A%29)",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/(%28:%3A%29)",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/%3A%3a%3C%3c",
//     "base": null,
//     "href": "http://example.com/%3A%3a%3C%3c",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%3A%3a%3C%3c",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/foo\tbar",
//     "base": null,
//     "href": "http://example.com/foobar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/foobar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com\\\\foo\\\\bar",
//     "base": null,
//     "href": "http://example.com//foo//bar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "//foo//bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/%7Ffp3%3Eju%3Dduvgw%3Dd",
//     "base": null,
//     "href": "http://example.com/%7Ffp3%3Eju%3Dduvgw%3Dd",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%7Ffp3%3Eju%3Dduvgw%3Dd",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/@asdf%40",
//     "base": null,
//     "href": "http://example.com/@asdf%40",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/@asdf%40",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/你好你好",
//     "base": null,
//     "href": "http://example.com/%E4%BD%A0%E5%A5%BD%E4%BD%A0%E5%A5%BD",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%E4%BD%A0%E5%A5%BD%E4%BD%A0%E5%A5%BD",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/‥/foo",
//     "base": null,
//     "href": "http://example.com/%E2%80%A5/foo",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%E2%80%A5/foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/﻿/foo",
//     "base": null,
//     "href": "http://example.com/%EF%BB%BF/foo",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%EF%BB%BF/foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.com/\u{202e}/foo/\u{202d}/bar",
//     "base": null,
//     "href": "http://example.com/%E2%80%AE/foo/%E2%80%AD/bar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/%E2%80%AE/foo/%E2%80%AD/bar",
//     "search": "",
//     "hash": ""
// }

//   "# Based on http://trac.webkit.org/browser/trunk/LayoutTests/fast/url/script-tests/relative.js",
// #[test]
// fn uri_() {
//     "input": "http://www.google.com/foo?bar=baz#",
//     "base": null,
//     "href": "http://www.google.com/foo?bar=baz#",
//     "origin": "http://www.google.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.google.com",
//     "hostname": "www.google.com",
//     "port": "",
//     "pathname": "/foo",
//     "search": "?bar=baz",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://www.google.com/foo?bar=baz# »",
//     "base": null,
//     "href": "http://www.google.com/foo?bar=baz#%20%C2%BB",
//     "origin": "http://www.google.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.google.com",
//     "hostname": "www.google.com",
//     "port": "",
//     "pathname": "/foo",
//     "search": "?bar=baz",
//     "hash": "#%20%C2%BB"
// }

// #[test]
// fn uri_() {
//     "input": "data:test# »",
//     "base": null,
//     "href": "data:test#%20%C2%BB",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "test",
//     "search": "",
//     "hash": "#%20%C2%BB"
// }

// #[test]
// fn uri_() {
//     "input": "http://www.google.com",
//     "base": null,
//     "href": "http://www.google.com/",
//     "origin": "http://www.google.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.google.com",
//     "hostname": "www.google.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://192.0x00A80001",
//     "base": null,
//     "href": "http://192.168.0.1/",
//     "origin": "http://192.168.0.1",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "192.168.0.1",
//     "hostname": "192.168.0.1",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://www/foo%2Ehtml",
//     "base": null,
//     "href": "http://www/foo%2Ehtml",
//     "origin": "http://www",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www",
//     "hostname": "www",
//     "port": "",
//     "pathname": "/foo%2Ehtml",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://www/foo/%2E/html",
//     "base": null,
//     "href": "http://www/foo/html",
//     "origin": "http://www",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www",
//     "hostname": "www",
//     "port": "",
//     "pathname": "/foo/html",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "http://user:pass@/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "http://%25DOMAIN:foobar@foodomain.com/",
//     "base": null,
//     "href": "http://%25DOMAIN:foobar@foodomain.com/",
//     "origin": "http://foodomain.com",
//     "protocol": "http:",
//     "username": "%25DOMAIN",
//     "password": "foobar",
//     "host": "foodomain.com",
//     "hostname": "foodomain.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:\\\\www.google.com\\foo",
//     "base": null,
//     "href": "http://www.google.com/foo",
//     "origin": "http://www.google.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.google.com",
//     "hostname": "www.google.com",
//     "port": "",
//     "pathname": "/foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://foo:80/",
//     "base": null,
//     "href": "http://foo/",
//     "origin": "http://foo",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://foo:81/",
//     "base": null,
//     "href": "http://foo:81/",
//     "origin": "http://foo:81",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo:81",
//     "hostname": "foo",
//     "port": "81",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "httpa://foo:80/",
//     "base": null,
//     "href": "httpa://foo:80/",
//     "origin": "null",
//     "protocol": "httpa:",
//     "username": "",
//     "password": "",
//     "host": "foo:80",
//     "hostname": "foo",
//     "port": "80",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "http://foo:-80/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "https://foo:443/",
//     "base": null,
//     "href": "https://foo/",
//     "origin": "https://foo",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://foo:80/",
//     "base": null,
//     "href": "https://foo:80/",
//     "origin": "https://foo:80",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "foo:80",
//     "hostname": "foo",
//     "port": "80",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftp://foo:21/",
//     "base": null,
//     "href": "ftp://foo/",
//     "origin": "ftp://foo",
//     "protocol": "ftp:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftp://foo:80/",
//     "base": null,
//     "href": "ftp://foo:80/",
//     "origin": "ftp://foo:80",
//     "protocol": "ftp:",
//     "username": "",
//     "password": "",
//     "host": "foo:80",
//     "hostname": "foo",
//     "port": "80",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "gopher://foo:70/",
//     "base": null,
//     "href": "gopher://foo:70/",
//     "origin": "null",
//     "protocol": "gopher:",
//     "username": "",
//     "password": "",
//     "host": "foo:70",
//     "hostname": "foo",
//     "port": "70",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "gopher://foo:443/",
//     "base": null,
//     "href": "gopher://foo:443/",
//     "origin": "null",
//     "protocol": "gopher:",
//     "username": "",
//     "password": "",
//     "host": "foo:443",
//     "hostname": "foo",
//     "port": "443",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws://foo:80/",
//     "base": null,
//     "href": "ws://foo/",
//     "origin": "ws://foo",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws://foo:81/",
//     "base": null,
//     "href": "ws://foo:81/",
//     "origin": "ws://foo:81",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "foo:81",
//     "hostname": "foo",
//     "port": "81",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws://foo:443/",
//     "base": null,
//     "href": "ws://foo:443/",
//     "origin": "ws://foo:443",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "foo:443",
//     "hostname": "foo",
//     "port": "443",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws://foo:815/",
//     "base": null,
//     "href": "ws://foo:815/",
//     "origin": "ws://foo:815",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "foo:815",
//     "hostname": "foo",
//     "port": "815",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://foo:80/",
//     "base": null,
//     "href": "wss://foo:80/",
//     "origin": "wss://foo:80",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "foo:80",
//     "hostname": "foo",
//     "port": "80",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://foo:81/",
//     "base": null,
//     "href": "wss://foo:81/",
//     "origin": "wss://foo:81",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "foo:81",
//     "hostname": "foo",
//     "port": "81",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://foo:443/",
//     "base": null,
//     "href": "wss://foo/",
//     "origin": "wss://foo",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "foo",
//     "hostname": "foo",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://foo:815/",
//     "base": null,
//     "href": "wss://foo:815/",
//     "origin": "wss://foo:815",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "foo:815",
//     "hostname": "foo",
//     "port": "815",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:/example.com/",
//     "base": null,
//     "href": "http://example.com/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftp:/example.com/",
//     "base": null,
//     "href": "ftp://example.com/",
//     "origin": "ftp://example.com",
//     "protocol": "ftp:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https:/example.com/",
//     "base": null,
//     "href": "https://example.com/",
//     "origin": "https://example.com",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "madeupscheme:/example.com/",
//     "base": null,
//     "href": "madeupscheme:/example.com/",
//     "origin": "null",
//     "protocol": "madeupscheme:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:/example.com/",
//     "base": null,
//     "href": "file:///example.com/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftps:/example.com/",
//     "base": null,
//     "href": "ftps:/example.com/",
//     "origin": "null",
//     "protocol": "ftps:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "gopher:/example.com/",
//     "base": null,
//     "href": "gopher:/example.com/",
//     "origin": "null",
//     "protocol": "gopher:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws:/example.com/",
//     "base": null,
//     "href": "ws://example.com/",
//     "origin": "ws://example.com",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss:/example.com/",
//     "base": null,
//     "href": "wss://example.com/",
//     "origin": "wss://example.com",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "data:/example.com/",
//     "base": null,
//     "href": "data:/example.com/",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "javascript:/example.com/",
//     "base": null,
//     "href": "javascript:/example.com/",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "mailto:/example.com/",
//     "base": null,
//     "href": "mailto:/example.com/",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:example.com/",
//     "base": null,
//     "href": "http://example.com/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftp:example.com/",
//     "base": null,
//     "href": "ftp://example.com/",
//     "origin": "ftp://example.com",
//     "protocol": "ftp:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https:example.com/",
//     "base": null,
//     "href": "https://example.com/",
//     "origin": "https://example.com",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "madeupscheme:example.com/",
//     "base": null,
//     "href": "madeupscheme:example.com/",
//     "origin": "null",
//     "protocol": "madeupscheme:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ftps:example.com/",
//     "base": null,
//     "href": "ftps:example.com/",
//     "origin": "null",
//     "protocol": "ftps:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "gopher:example.com/",
//     "base": null,
//     "href": "gopher:example.com/",
//     "origin": "null",
//     "protocol": "gopher:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ws:example.com/",
//     "base": null,
//     "href": "ws://example.com/",
//     "origin": "ws://example.com",
//     "protocol": "ws:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss:example.com/",
//     "base": null,
//     "href": "wss://example.com/",
//     "origin": "wss://example.com",
//     "protocol": "wss:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "data:example.com/",
//     "base": null,
//     "href": "data:example.com/",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "javascript:example.com/",
//     "base": null,
//     "href": "javascript:example.com/",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "mailto:example.com/",
//     "base": null,
//     "href": "mailto:example.com/",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "example.com/",
//     "search": "",
//     "hash": ""
// }

//   "# Based on http://trac.webkit.org/browser/trunk/LayoutTests/fast/url/segments-userinfo-vs-host.html",
// #[test]
// fn uri_() {
//     "input": "http:@www.example.com",
//     "base": null,
//     "href": "http://www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:/@www.example.com",
//     "base": null,
//     "href": "http://www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://@www.example.com",
//     "base": null,
//     "href": "http://www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:a:b@www.example.com",
//     "base": null,
//     "href": "http://a:b@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "a",
//     "password": "b",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:/a:b@www.example.com",
//     "base": null,
//     "href": "http://a:b@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "a",
//     "password": "b",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://a:b@www.example.com",
//     "base": null,
//     "href": "http://a:b@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "a",
//     "password": "b",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://@pple.com",
//     "base": null,
//     "href": "http://pple.com/",
//     "origin": "http://pple.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "pple.com",
//     "hostname": "pple.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http::b@www.example.com",
//     "base": null,
//     "href": "http://:b@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "b",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:/:b@www.example.com",
//     "base": null,
//     "href": "http://:b@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "b",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://:b@www.example.com",
//     "base": null,
//     "href": "http://:b@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "b",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:/:@/www.example.com",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     let input = "http://user@/www.example.com";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "http:@/www.example.com",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     "input": "http:/@/www.example.com",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     let input = "http://@/www.example.com";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "https:@/www.example.com",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     "input": "http:a:b@/www.example.com",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     "input": "http:/a:b@/www.example.com",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     let input = "http://a:b@/www.example.com";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "http::@/www.example.com",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     "input": "http:a:@www.example.com",
//     "base": null,
//     "href": "http://a@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "a",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:/a:@www.example.com",
//     "base": null,
//     "href": "http://a@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "a",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://a:@www.example.com",
//     "base": null,
//     "href": "http://a@www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "a",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://www.@pple.com",
//     "base": null,
//     "href": "http://www.@pple.com/",
//     "origin": "http://pple.com",
//     "protocol": "http:",
//     "username": "www.",
//     "password": "",
//     "host": "pple.com",
//     "hostname": "pple.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_non_opaque_path_base() {
//     let input = "http://@:www.example.com";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "http://:@www.example.com",
//     "base": null,
//     "href": "http://www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "# Others",
// #[test]
// fn uri_() {
//     "input": "/",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/test.txt",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/test.txt",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/test.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": ".",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "..",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "test.txt",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/test.txt",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/test.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "./test.txt",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/test.txt",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/test.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "../test.txt",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/test.txt",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/test.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "../aaa/test.txt",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/aaa/test.txt",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/aaa/test.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "../../test.txt",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/test.txt",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/test.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "中/test.txt",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example.com/%E4%B8%AD/test.txt",
//     "origin": "http://www.example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "port": "",
//     "pathname": "/%E4%B8%AD/test.txt",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://www.example2.com",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example2.com/",
//     "origin": "http://www.example2.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example2.com",
//     "hostname": "www.example2.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//www.example2.com",
//     "base": "http://www.example.com/test",
//     "href": "http://www.example2.com/",
//     "origin": "http://www.example2.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.example2.com",
//     "hostname": "www.example2.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:...",
//     "base": "http://www.example.com/test",
//     "href": "file:///...",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/...",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:..",
//     "base": "http://www.example.com/test",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:a",
//     "base": "http://www.example.com/test",
//     "href": "file:///a",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/a",
//     "search": "",
//     "hash": ""
// }

//   "# Based on http://trac.webkit.org/browser/trunk/LayoutTests/fast/url/host.html",
//   "Basic canonicalization, uppercase should be converted to lowercase",
// #[test]
// fn uri_() {
//     "input": "http://ExAmPlE.CoM",
//     "base": "http://other.com/",
//     "href": "http://example.com/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example example.com",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://Goo%20 goo%7C|.com",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[:]",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "U+3000 is mapped to U+0020 (space) which is disallowed",
// #[test]
// fn uri_() {
//     "input": "http://GOO\u00a0\u3000goo.com",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "Other types of space (no-break, zero-width, zero-width-no-break) are name-prepped away to nothing. U+200B, U+2060, and U+FEFF, are ignored",
// #[test]
// fn uri_() {
//     "input": "http://GOO\u200b\u2060\ufeffgoo.com",
//     "base": "http://other.com/",
//     "href": "http://googoo.com/",
//     "origin": "http://googoo.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "googoo.com",
//     "hostname": "googoo.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Leading and trailing C0 control or space",
// #[test]
// fn uri_() {
//     "input": "\u0000\u001b\u0004\u0012 http://example.com/\u001f \u000d ",
//     "base": null,
//     "href": "http://example.com/",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Ideographic full stop (full-width period for Chinese, etc.) should be treated as a dot. U+3002 is mapped to U+002E (dot)",
// #[test]
// fn uri_() {
//     "input": "http://www.foo。bar.com",
//     "base": "http://other.com/",
//     "href": "http://www.foo.bar.com/",
//     "origin": "http://www.foo.bar.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "www.foo.bar.com",
//     "hostname": "www.foo.bar.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Invalid unicode characters should fail... U+FDD0 is disallowed; %ef%b7%90 is U+FDD0",
// #[test]
// fn uri_() {
//     "input": "http://\ufdd0zyx.com",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "This is the same as previous but escaped",
// #[test]
// fn uri_() {
//     "input": "http://%ef%b7%90zyx.com",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "U+FFFD",
// #[test]
// fn uri_() {
//     let input = "https://\ufffd";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://%EF%BF%BD";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "https://x/\ufffd?\ufffd#\ufffd",
//     "base": null,
//     "href": "https://x/%EF%BF%BD?%EF%BF%BD#%EF%BF%BD",
//     "origin": "https://x",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "x",
//     "hostname": "x",
//     "port": "",
//     "pathname": "/%EF%BF%BD",
//     "search": "?%EF%BF%BD",
//     "hash": "#%EF%BF%BD"
// }

//   "Domain is ASCII, but a label is invalid IDNA",
// #[test]
// fn uri_() {
//     let input = "http://a.b.c.xn--pokxncvks";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://10.0.0.xn--pokxncvks";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "IDNA labels should be matched case-insensitively",
// #[test]
// fn uri_() {
//     let input = "http://a.b.c.XN--pokxncvks";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a.b.c.Xn--pokxncvks";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://10.0.0.XN--pokxncvks";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://10.0.0.xN--pokxncvks";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Test name prepping, fullwidth input should be converted to ASCII and NOT IDN-ized. This is 'Go' in fullwidth UTF-8/UTF-16.",
// #[test]
// fn uri_() {
//     "input": "http://Ｇｏ.com",
//     "base": "http://other.com/",
//     "href": "http://go.com/",
//     "origin": "http://go.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "go.com",
//     "hostname": "go.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "URL spec forbids the following. https://www.w3.org/Bugs/Public/show_bug.cgi?id=24257",
// #[test]
// fn uri_() {
//     "input": "http://％４１.com",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://%ef%bc%85%ef%bc%94%ef%bc%91.com",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "...%00 in fullwidth should fail (also as escaped UTF-8 input)",
// #[test]
// fn uri_() {
//     "input": "http://％００.com",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://%ef%bc%85%ef%bc%90%ef%bc%90.com",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "Basic IDN support, UTF-8 and UTF-16 input should be converted to IDN",
// #[test]
// fn uri_() {
//     "input": "http://你好你好",
//     "base": "http://other.com/",
//     "href": "http://xn--6qqa088eba/",
//     "origin": "http://xn--6qqa088eba",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "xn--6qqa088eba",
//     "hostname": "xn--6qqa088eba",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://faß.ExAmPlE/",
//     "base": null,
//     "href": "https://xn--fa-hia.example/",
//     "origin": "https://xn--fa-hia.example",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "xn--fa-hia.example",
//     "hostname": "xn--fa-hia.example",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "sc://faß.ExAmPlE/",
//     "base": null,
//     "href": "sc://fa%C3%9F.ExAmPlE/",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "fa%C3%9F.ExAmPlE",
//     "hostname": "fa%C3%9F.ExAmPlE",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Invalid escaped characters should fail and the percents should be escaped. https://www.w3.org/Bugs/Public/show_bug.cgi?id=24191",
// #[test]
// fn uri_() {
//     "input": "http://%zz%66%a.com",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "If we get an invalid character that has been escaped.",
// #[test]
// fn uri_() {
//     "input": "http://%25",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://hello%00",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "Escaped numbers should be treated like IP addresses if they are.",
// #[test]
// fn uri_() {
//     "input": "http://%30%78%63%30%2e%30%32%35%30.01",
//     "base": "http://other.com/",
//     "href": "http://192.168.0.1/",
//     "origin": "http://192.168.0.1",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "192.168.0.1",
//     "hostname": "192.168.0.1",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://%30%78%63%30%2e%30%32%35%30.01%2e",
//     "base": "http://other.com/",
//     "href": "http://192.168.0.1/",
//     "origin": "http://192.168.0.1",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "192.168.0.1",
//     "hostname": "192.168.0.1",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://192.168.0.257",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "Invalid escaping in hosts causes failure",
// #[test]
// fn uri_() {
//     "input": "http://%3g%78%63%30%2e%30%32%35%30%2E.01",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "A space in a host causes failure",
// #[test]
// fn uri_() {
//     "input": "http://192.168.0.1 hello",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     let input = "https://x x:12";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Fullwidth and escaped UTF-8 fullwidth should still be treated as IP",
// #[test]
// fn uri_() {
//     "input": "http://０Ｘｃ０．０２５０．０１",
//     "base": "http://other.com/",
//     "href": "http://192.168.0.1/",
//     "origin": "http://192.168.0.1",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "192.168.0.1",
//     "hostname": "192.168.0.1",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Domains with empty labels",
// #[test]
// fn uri_() {
//     "input": "http://./",
//     "base": null,
//     "href": "http://./",
//     "origin": "http://.",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": ".",
//     "hostname": ".",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://../",
//     "base": null,
//     "href": "http://../",
//     "origin": "http://..",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "..",
//     "hostname": "..",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Non-special domains with empty labels",
// #[test]
// fn uri_() {
//     "input": "h://.",
//     "base": null,
//     "href": "h://.",
//     "origin": "null",
//     "protocol": "h:",
//     "username": "",
//     "password": "",
//     "host": ".",
//     "hostname": ".",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }

//   "Broken IPv6",
// #[test]
// fn uri_() {
//     let input = "http://[www.google.com]/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "http://[google.com]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[::1.2.3.4x]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[::1.2.3.]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[::1.2.]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[::.1.2]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[::1.]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[::.1]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://[::%31]",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://%5B::1]",
//     "base": "http://other.com/",
//     "failure": true
// }

//   "Misc Unicode",
// #[test]
// fn uri_() {
//     "input": "http://foo:💩@example.com/bar",
//     "base": "http://other.com/",
//     "href": "http://foo:%F0%9F%92%A9@example.com/bar",
//     "origin": "http://example.com",
//     "protocol": "http:",
//     "username": "foo",
//     "password": "%F0%9F%92%A9",
//     "host": "example.com",
//     "hostname": "example.com",
//     "port": "",
//     "pathname": "/bar",
//     "search": "",
//     "hash": ""
// }

//   "# resolving a fragment against any scheme succeeds",
// #[test]
// fn uri_() {
//     "input": "#",
//     "base": "test:test",
//     "href": "test:test#",
//     "origin": "null",
//     "protocol": "test:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "#x",
//     "base": "mailto:x@x.com",
//     "href": "mailto:x@x.com#x",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "x@x.com",
//     "search": "",
//     "hash": "#x"
// }

// #[test]
// fn uri_() {
//     "input": "#x",
//     "base": "data:,",
//     "href": "data:,#x",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": ",",
//     "search": "",
//     "hash": "#x"
// }

// #[test]
// fn uri_() {
//     "input": "#x",
//     "base": "about:blank",
//     "href": "about:blank#x",
//     "origin": "null",
//     "protocol": "about:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "blank",
//     "search": "",
//     "hash": "#x"
// }

// #[test]
// fn uri_() {
//     "input": "#x:y",
//     "base": "about:blank",
//     "href": "about:blank#x:y",
//     "origin": "null",
//     "protocol": "about:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "blank",
//     "search": "",
//     "hash": "#x:y"
// }

// #[test]
// fn uri_() {
//     "input": "#",
//     "base": "test:test?test",
//     "href": "test:test?test#",
//     "origin": "null",
//     "protocol": "test:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "test",
//     "search": "?test",
//     "hash": ""
// }

//   "# multiple @ in authority state",
// #[test]
// fn uri_() {
//     "input": "https://@test@test@example:800/",
//     "base": "http://doesnotmatter/",
//     "href": "https://%40test%40test@example:800/",
//     "origin": "https://example:800",
//     "protocol": "https:",
//     "username": "%40test%40test",
//     "password": "",
//     "host": "example:800",
//     "hostname": "example",
//     "port": "800",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://@@@example",
//     "base": "http://doesnotmatter/",
//     "href": "https://%40%40@example/",
//     "origin": "https://example",
//     "protocol": "https:",
//     "username": "%40%40",
//     "password": "",
//     "host": "example",
//     "hostname": "example",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "non-az-09 characters",
// #[test]
// fn uri_() {
//     "input": "http://`{}:`{}@h/`{}?`{}",
//     "base": "http://doesnotmatter/",
//     "href": "http://%60%7B%7D:%60%7B%7D@h/%60%7B%7D?`{}",
//     "origin": "http://h",
//     "protocol": "http:",
//     "username": "%60%7B%7D",
//     "password": "%60%7B%7D",
//     "host": "h",
//     "hostname": "h",
//     "port": "",
//     "pathname": "/%60%7B%7D",
//     "search": "?`{}",
//     "hash": ""
// }

//   "byte is ' and url is special",
// #[test]
// fn uri_() {
//     "input": "http://host/?'",
//     "base": null,
//     "href": "http://host/?%27",
//     "origin": "http://host",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/",
//     "search": "?%27",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "notspecial://host/?'",
//     "base": null,
//     "href": "notspecial://host/?'",
//     "origin": "null",
//     "protocol": "notspecial:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/",
//     "search": "?'",
//     "hash": ""
// }

//   "# Credentials in base",
// #[test]
// fn uri_() {
//     "input": "/some/path",
//     "base": "http://user@example.org/smth",
//     "href": "http://user@example.org/some/path",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "user",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/some/path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "",
//     "base": "http://user:pass@example.org:21/smth",
//     "href": "http://user:pass@example.org:21/smth",
//     "origin": "http://example.org:21",
//     "protocol": "http:",
//     "username": "user",
//     "password": "pass",
//     "host": "example.org:21",
//     "hostname": "example.org",
//     "port": "21",
//     "pathname": "/smth",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/some/path",
//     "base": "http://user:pass@example.org:21/smth",
//     "href": "http://user:pass@example.org:21/some/path",
//     "origin": "http://example.org:21",
//     "protocol": "http:",
//     "username": "user",
//     "password": "pass",
//     "host": "example.org:21",
//     "hostname": "example.org",
//     "port": "21",
//     "pathname": "/some/path",
//     "search": "",
//     "hash": ""
// }

//   "# a set of tests designed by zcorpan for relative URLs with unknown schemes",
// #[test]
// fn uri_() {
//     "input": "i",
//     "base": "sc:sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "i",
//     "base": "sc:sd/sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "i",
//     "base": "sc:/pa/pa",
//     "href": "sc:/pa/i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/pa/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "i",
//     "base": "sc://ho/pa",
//     "href": "sc://ho/i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "ho",
//     "hostname": "ho",
//     "port": "",
//     "pathname": "/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "i",
//     "base": "sc:///pa/pa",
//     "href": "sc:///pa/i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/pa/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "../i",
//     "base": "sc:sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "../i",
//     "base": "sc:sd/sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "../i",
//     "base": "sc:/pa/pa",
//     "href": "sc:/i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "../i",
//     "base": "sc://ho/pa",
//     "href": "sc://ho/i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "ho",
//     "hostname": "ho",
//     "port": "",
//     "pathname": "/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "../i",
//     "base": "sc:///pa/pa",
//     "href": "sc:///i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/i",
//     "base": "sc:sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "/i",
//     "base": "sc:sd/sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "/i",
//     "base": "sc:/pa/pa",
//     "href": "sc:/i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/i",
//     "base": "sc://ho/pa",
//     "href": "sc://ho/i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "ho",
//     "hostname": "ho",
//     "port": "",
//     "pathname": "/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/i",
//     "base": "sc:///pa/pa",
//     "href": "sc:///i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/i",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "?i",
//     "base": "sc:sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "?i",
//     "base": "sc:sd/sd",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "?i",
//     "base": "sc:/pa/pa",
//     "href": "sc:/pa/pa?i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/pa/pa",
//     "search": "?i",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "?i",
//     "base": "sc://ho/pa",
//     "href": "sc://ho/pa?i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "ho",
//     "hostname": "ho",
//     "port": "",
//     "pathname": "/pa",
//     "search": "?i",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "?i",
//     "base": "sc:///pa/pa",
//     "href": "sc:///pa/pa?i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/pa/pa",
//     "search": "?i",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "#i",
//     "base": "sc:sd",
//     "href": "sc:sd#i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "sd",
//     "search": "",
//     "hash": "#i"
// }

// #[test]
// fn uri_() {
//     "input": "#i",
//     "base": "sc:sd/sd",
//     "href": "sc:sd/sd#i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "sd/sd",
//     "search": "",
//     "hash": "#i"
// }

// #[test]
// fn uri_() {
//     "input": "#i",
//     "base": "sc:/pa/pa",
//     "href": "sc:/pa/pa#i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/pa/pa",
//     "search": "",
//     "hash": "#i"
// }

// #[test]
// fn uri_() {
//     "input": "#i",
//     "base": "sc://ho/pa",
//     "href": "sc://ho/pa#i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "ho",
//     "hostname": "ho",
//     "port": "",
//     "pathname": "/pa",
//     "search": "",
//     "hash": "#i"
// }

// #[test]
// fn uri_() {
//     "input": "#i",
//     "base": "sc:///pa/pa",
//     "href": "sc:///pa/pa#i",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/pa/pa",
//     "search": "",
//     "hash": "#i"
// }

//   "# make sure that relative URL logic works on known typically non-relative schemes too",
// #[test]
// fn uri_() {
//     "input": "about:/../",
//     "base": null,
//     "href": "about:/",
//     "origin": "null",
//     "protocol": "about:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "data:/../",
//     "base": null,
//     "href": "data:/",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "javascript:/../",
//     "base": null,
//     "href": "javascript:/",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "mailto:/../",
//     "base": null,
//     "href": "mailto:/",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "# unknown schemes and their hosts",
// #[test]
// fn uri_() {
//     "input": "sc://ñ.test/",
//     "base": null,
//     "href": "sc://%C3%B1.test/",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%C3%B1.test",
//     "hostname": "%C3%B1.test",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "sc://%/",
//     "base": null,
//     "href": "sc://%/",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%",
//     "hostname": "%",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "sc://@/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://te@s:t@/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://:/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://:12/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "x",
//     "base": "sc://ñ",
//     "href": "sc://%C3%B1/x",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%C3%B1",
//     "hostname": "%C3%B1",
//     "port": "",
//     "pathname": "/x",
//     "search": "",
//     "hash": ""
// }

//   "# unknown schemes and backslashes",
// #[test]
// fn uri_() {
//     "input": "sc:\\../",
//     "base": null,
//     "href": "sc:\\../",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "\\../",
//     "search": "",
//     "hash": ""
// }

//   "# unknown scheme with path looking like a password",
// #[test]
// fn uri_() {
//     "input": "sc::a@example.net",
//     "base": null,
//     "href": "sc::a@example.net",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": ":a@example.net",
//     "search": "",
//     "hash": ""
// }

//   "# unknown scheme with bogus percent-encoding",
// #[test]
// fn uri_() {
//     "input": "wow:%NBD",
//     "base": null,
//     "href": "wow:%NBD",
//     "origin": "null",
//     "protocol": "wow:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "%NBD",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "wow:%1G",
//     "base": null,
//     "href": "wow:%1G",
//     "origin": "null",
//     "protocol": "wow:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "%1G",
//     "search": "",
//     "hash": ""
// }

//   "# unknown scheme with non-URL characters",
// #[test]
// fn uri_() {
//     "input": "wow:\uFFFF",
//     "base": null,
//     "href": "wow:%EF%BF%BF",
//     "origin": "null",
//     "protocol": "wow:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "%EF%BF%BF",
//     "search": "",
//     "hash": ""
// }

//   "Forbidden host code points",
// #[test]
// fn uri_() {
//     let input = "sc://a\u0000b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a<b";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a>b";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a[b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a\\b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a]b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a^b";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "sc://a|b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Forbidden host codepoints: tabs and newlines are removed during preprocessing",
// #[test]
// fn uri_() {
//     "input": "foo://ho\u0009st/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href":"foo://host/",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://ho\u000Ast/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href":"foo://host/",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://ho\u000Dst/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href":"foo://host/",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": ""
// }

//   "Forbidden domain code-points",
// #[test]
// fn uri_() {
//     let input = "http://a\u0000b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0001b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0002b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0003b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0004b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0005b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0006b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0007b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0008b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u000Bb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u000Cb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u000Eb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u000Fb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0010b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0011b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0012b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0013b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0014b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0015b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0016b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0017b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0018b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u0019b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u001Ab/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u001Bb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u001Cb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u001Db/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u001Eb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u001Fb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a%b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a<b";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a>b";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a[b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a]b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a^b";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a|b/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://a\u007Fb/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Forbidden domain codepoints: tabs and newlines are removed during preprocessing",
// #[test]
// fn uri_() {
//     "input": "http://ho\u0009st/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href":"http://host/",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "http:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://ho\u000Ast/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href":"http://host/",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "http:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://ho\u000Dst/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href":"http://host/",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "http:",
//     "search": "",
//     "username": ""
// }

//   "Encoded forbidden domain codepoints in special URLs",
// #[test]
// fn uri_() {
//     let input = "http://ho%00st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%01st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%02st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%03st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%04st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%05st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%06st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%07st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%08st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%09st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%0Ast/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%0Bst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%0Cst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%0Dst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%0Est/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%0Fst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%10st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%11st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%12st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%13st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%14st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%15st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%16st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%17st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%18st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%19st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%1Ast/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%1Bst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%1Cst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%1Dst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%1Est/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%1Fst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%20st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%23st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%25st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%2Fst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%3Ast/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%3Cst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%3Est/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%3Fst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%40st/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%5Bst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%5Cst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%5Dst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%7Cst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://ho%7Fst/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Allowed host/domain code points",
// #[test]
// fn uri_() {
//     "input": "http://!\"$&'()*+,-.;=_`{}~/",
//     "base": null,
//     "href": "http://!\"$&'()*+,-.;=_`{}~/",
//     "origin": "http://!\"$&'()*+,-.;=_`{}~",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "!\"$&'()*+,-.;=_`{}~",
//     "hostname": "!\"$&'()*+,-.;=_`{}~",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "sc://\u0001\u0002\u0003\u0004\u0005\u0006\u0007\u0008\u000B\u000C\u000E\u000F\u0010\u0011\u0012\u0013\u0014\u0015\u0016\u0017\u0018\u0019\u001A\u001B\u001C\u001D\u001E\u001F\u007F!\"$%&'()*+,-.;=_`{}~/",
//     "base": null,
//     "href": "sc://%01%02%03%04%05%06%07%08%0B%0C%0E%0F%10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F%7F!\"$%&'()*+,-.;=_`{}~/",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%01%02%03%04%05%06%07%08%0B%0C%0E%0F%10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F%7F!\"$%&'()*+,-.;=_`{}~",
//     "hostname": "%01%02%03%04%05%06%07%08%0B%0C%0E%0F%10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F%7F!\"$%&'()*+,-.;=_`{}~",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "# Hosts and percent-encoding",
// #[test]
// fn uri_() {
//     let input = "ftp://example.com%80/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "ftp://example.com%A0/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://example.com%80/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://example.com%A0/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "ftp://%e2%98%83",
//     "base": null,
//     "href": "ftp://xn--n3h/",
//     "origin": "ftp://xn--n3h",
//     "protocol": "ftp:",
//     "username": "",
//     "password": "",
//     "host": "xn--n3h",
//     "hostname": "xn--n3h",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://%e2%98%83",
//     "base": null,
//     "href": "https://xn--n3h/",
//     "origin": "https://xn--n3h",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "xn--n3h",
//     "hostname": "xn--n3h",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "# tests from jsdom/whatwg-url designed for code coverage",
// #[test]
// fn uri_() {
//     "input": "http://127.0.0.1:10100/relative_import.html",
//     "base": null,
//     "href": "http://127.0.0.1:10100/relative_import.html",
//     "origin": "http://127.0.0.1:10100",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "127.0.0.1:10100",
//     "hostname": "127.0.0.1",
//     "port": "10100",
//     "pathname": "/relative_import.html",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://facebook.com/?foo=%7B%22abc%22",
//     "base": null,
//     "href": "http://facebook.com/?foo=%7B%22abc%22",
//     "origin": "http://facebook.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "facebook.com",
//     "hostname": "facebook.com",
//     "port": "",
//     "pathname": "/",
//     "search": "?foo=%7B%22abc%22",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://localhost:3000/jqueryui@1.2.3",
//     "base": null,
//     "href": "https://localhost:3000/jqueryui@1.2.3",
//     "origin": "https://localhost:3000",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "localhost:3000",
//     "hostname": "localhost",
//     "port": "3000",
//     "pathname": "/jqueryui@1.2.3",
//     "search": "",
//     "hash": ""
// }

//   "# tab/LF/CR",
// #[test]
// fn uri_() {
//     "input": "h\tt\nt\rp://h\to\ns\rt:9\t0\n0\r0/p\ta\nt\rh?q\tu\ne\rry#f\tr\na\rg",
//     "base": null,
//     "href": "http://host:9000/path?query#frag",
//     "origin": "http://host:9000",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "host:9000",
//     "hostname": "host",
//     "port": "9000",
//     "pathname": "/path",
//     "search": "?query",
//     "hash": "#frag"
// }

//   "# Stringification of URL.searchParams",
// #[test]
// fn uri_() {
//     "input": "?a=b&c=d",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar?a=b&c=d",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "?a=b&c=d",
//     "searchParams": "a=b&c=d",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "??a=b&c=d",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar??a=b&c=d",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "??a=b&c=d",
//     "searchParams": "%3Fa=b&c=d",
//     "hash": ""
// }

//   "# Scheme only",
// #[test]
// fn uri_() {
//     "input": "http:",
//     "base": "http://example.org/foo/bar",
//     "href": "http://example.org/foo/bar",
//     "origin": "http://example.org",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "searchParams": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http:",
//     "base": "https://example.org/foo/bar",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "sc:",
//     "base": "https://example.org/foo/bar",
//     "href": "sc:",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "searchParams": "",
//     "hash": ""
// }

//   "# Percent encoding of fragments",
// #[test]
// fn uri_() {
//     "input": "http://foo.bar/baz?qux#foo\bbar",
//     "base": null,
//     "href": "http://foo.bar/baz?qux#foo%08bar",
//     "origin": "http://foo.bar",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo.bar",
//     "hostname": "foo.bar",
//     "port": "",
//     "pathname": "/baz",
//     "search": "?qux",
//     "searchParams": "qux=",
//     "hash": "#foo%08bar"
// }

// #[test]
// fn uri_() {
//     "input": "http://foo.bar/baz?qux#foo\"bar",
//     "base": null,
//     "href": "http://foo.bar/baz?qux#foo%22bar",
//     "origin": "http://foo.bar",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo.bar",
//     "hostname": "foo.bar",
//     "port": "",
//     "pathname": "/baz",
//     "search": "?qux",
//     "searchParams": "qux=",
//     "hash": "#foo%22bar"
// }

// #[test]
// fn uri_() {
//     "input": "http://foo.bar/baz?qux#foo<bar",
//     "base": null,
//     "href": "http://foo.bar/baz?qux#foo%3Cbar",
//     "origin": "http://foo.bar",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo.bar",
//     "hostname": "foo.bar",
//     "port": "",
//     "pathname": "/baz",
//     "search": "?qux",
//     "searchParams": "qux=",
//     "hash": "#foo%3Cbar"
// }

// #[test]
// fn uri_() {
//     "input": "http://foo.bar/baz?qux#foo>bar",
//     "base": null,
//     "href": "http://foo.bar/baz?qux#foo%3Ebar",
//     "origin": "http://foo.bar",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo.bar",
//     "hostname": "foo.bar",
//     "port": "",
//     "pathname": "/baz",
//     "search": "?qux",
//     "searchParams": "qux=",
//     "hash": "#foo%3Ebar"
// }

// #[test]
// fn uri_() {
//     "input": "http://foo.bar/baz?qux#foo`bar",
//     "base": null,
//     "href": "http://foo.bar/baz?qux#foo%60bar",
//     "origin": "http://foo.bar",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "foo.bar",
//     "hostname": "foo.bar",
//     "port": "",
//     "pathname": "/baz",
//     "search": "?qux",
//     "searchParams": "qux=",
//     "hash": "#foo%60bar"
// }

//   "# IPv4 parsing (via https://github.com/nodejs/node/pull/10317)",
// #[test]
// fn uri_() {
//     "input": "http://1.2.3.4/",
//     "base": "http://other.com/",
//     "href": "http://1.2.3.4/",
//     "origin": "http://1.2.3.4",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "1.2.3.4",
//     "hostname": "1.2.3.4",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://1.2.3.4./",
//     "base": "http://other.com/",
//     "href": "http://1.2.3.4/",
//     "origin": "http://1.2.3.4",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "1.2.3.4",
//     "hostname": "1.2.3.4",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://192.168.257",
//     "base": "http://other.com/",
//     "href": "http://192.168.1.1/",
//     "origin": "http://192.168.1.1",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "192.168.1.1",
//     "hostname": "192.168.1.1",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://192.168.257.",
//     "base": "http://other.com/",
//     "href": "http://192.168.1.1/",
//     "origin": "http://192.168.1.1",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "192.168.1.1",
//     "hostname": "192.168.1.1",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://192.168.257.com",
//     "base": "http://other.com/",
//     "href": "http://192.168.257.com/",
//     "origin": "http://192.168.257.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "192.168.257.com",
//     "hostname": "192.168.257.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://256",
//     "base": "http://other.com/",
//     "href": "http://0.0.1.0/",
//     "origin": "http://0.0.1.0",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "0.0.1.0",
//     "hostname": "0.0.1.0",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://256.com",
//     "base": "http://other.com/",
//     "href": "http://256.com/",
//     "origin": "http://256.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "256.com",
//     "hostname": "256.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://999999999",
//     "base": "http://other.com/",
//     "href": "http://59.154.201.255/",
//     "origin": "http://59.154.201.255",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "59.154.201.255",
//     "hostname": "59.154.201.255",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://999999999.",
//     "base": "http://other.com/",
//     "href": "http://59.154.201.255/",
//     "origin": "http://59.154.201.255",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "59.154.201.255",
//     "hostname": "59.154.201.255",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://999999999.com",
//     "base": "http://other.com/",
//     "href": "http://999999999.com/",
//     "origin": "http://999999999.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "999999999.com",
//     "hostname": "999999999.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://10000000000",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://10000000000.com",
//     "base": "http://other.com/",
//     "href": "http://10000000000.com/",
//     "origin": "http://10000000000.com",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "10000000000.com",
//     "hostname": "10000000000.com",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://4294967295",
//     "base": "http://other.com/",
//     "href": "http://255.255.255.255/",
//     "origin": "http://255.255.255.255",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "255.255.255.255",
//     "hostname": "255.255.255.255",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://4294967296",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://0xffffffff",
//     "base": "http://other.com/",
//     "href": "http://255.255.255.255/",
//     "origin": "http://255.255.255.255",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "255.255.255.255",
//     "hostname": "255.255.255.255",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://0xffffffff1",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://256.256.256.256",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "https://0x.0x.0",
//     "base": null,
//     "href": "https://0.0.0.0/",
//     "origin": "https://0.0.0.0",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "0.0.0.0",
//     "hostname": "0.0.0.0",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "More IPv4 parsing (via https://github.com/jsdom/whatwg-url/issues/92)",
// #[test]
// fn uri_() {
//     let input = "https://0x100000000/test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://256.0.0.1/test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "# file URLs containing percent-encoded Windows drive letters (shouldn't work)",
// #[test]
// fn uri_() {
//     "input": "file:///C%3A/",
//     "base": null,
//     "href": "file:///C%3A/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C%3A/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///C%7C/",
//     "base": null,
//     "href": "file:///C%7C/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C%7C/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "file://%43%3A";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://%43%7C";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://%43|";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://C%7C";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://%43%7C/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://%43%7C/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "asdf://%43|/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "asdf://%43%7C/",
//     "base": null,
//     "href": "asdf://%43%7C/",
//     "origin": "null",
//     "protocol": "asdf:",
//     "username": "",
//     "password": "",
//     "host": "%43%7C",
//     "hostname": "%43%7C",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "# file URLs relative to other file URLs (via https://github.com/jsdom/whatwg-url/pull/60)",
// #[test]
// fn uri_() {
//     "input": "pix/submit.gif",
//     "base": "file:///C:/Users/Domenic/Dropbox/GitHub/tmpvar/jsdom/test/level2/html/files/anchor.html",
//     "href": "file:///C:/Users/Domenic/Dropbox/GitHub/tmpvar/jsdom/test/level2/html/files/pix/submit.gif",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/Users/Domenic/Dropbox/GitHub/tmpvar/jsdom/test/level2/html/files/pix/submit.gif",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "..",
//     "base": "file:///C:/",
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "..",
//     "base": "file:///",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "# More file URL tests by zcorpan and annevk",
// #[test]
// fn uri_() {
//     "input": "/",
//     "base": "file:///C:/a/b",
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/",
//     "base": "file://h/C:/a/b",
//     "href": "file://h/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "h",
//     "hostname": "h",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/",
//     "base": "file://h/a/b",
//     "href": "file://h/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "h",
//     "hostname": "h",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//d:",
//     "base": "file:///C:/a/b",
//     "href": "file:///d:",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/d:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//d:/..",
//     "base": "file:///C:/a/b",
//     "href": "file:///d:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/d:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "..",
//     "base": "file:///ab:/",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "..",
//     "base": "file:///1:/",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "",
//     "base": "file:///test?test#test",
//     "href": "file:///test?test",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "?test",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:",
//     "base": "file:///test?test#test",
//     "href": "file:///test?test",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "?test",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "?x",
//     "base": "file:///test?test#test",
//     "href": "file:///test?x",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "?x",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:?x",
//     "base": "file:///test?test#test",
//     "href": "file:///test?x",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "?x",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "#x",
//     "base": "file:///test?test#test",
//     "href": "file:///test?test#x",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "?test",
//     "hash": "#x"
// }

// #[test]
// fn uri_() {
//     "input": "file:#x",
//     "base": "file:///test?test#test",
//     "href": "file:///test?test#x",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "?test",
//     "hash": "#x"
// }

//   "# File URLs and many (back)slashes",
// #[test]
// fn uri_() {
//     "input": "file:\\\\//",
//     "base": null,
//     "href": "file:////",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:\\\\\\\\",
//     "base": null,
//     "href": "file:////",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:\\\\\\\\?fox",
//     "base": null,
//     "href": "file:////?fox",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "?fox",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:\\\\\\\\#guppy",
//     "base": null,
//     "href": "file:////#guppy",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": "#guppy"
// }

// #[test]
// fn uri_() {
//     "input": "file://spider///",
//     "base": null,
//     "href": "file://spider///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "spider",
//     "hostname": "spider",
//     "port": "",
//     "pathname": "///",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:\\\\localhost//",
//     "base": null,
//     "href": "file:////",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///localhost//cat",
//     "base": null,
//     "href": "file:///localhost//cat",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/localhost//cat",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://\\/localhost//cat",
//     "base": null,
//     "href": "file:////localhost//cat",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//localhost//cat",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://localhost//a//../..//",
//     "base": null,
//     "href": "file://///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "///",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/////mouse",
//     "base": "file:///elephant",
//     "href": "file://///mouse",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "///mouse",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\\//pig",
//     "base": "file://lion/",
//     "href": "file:///pig",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/pig",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\\/localhost//pig",
//     "base": "file://lion/",
//     "href": "file:////pig",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//pig",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//localhost//pig",
//     "base": "file://lion/",
//     "href": "file:////pig",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//pig",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/..//localhost//pig",
//     "base": "file://lion/",
//     "href": "file://lion//localhost//pig",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "lion",
//     "hostname": "lion",
//     "port": "",
//     "pathname": "//localhost//pig",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://",
//     "base": "file://ape/",
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "# File URLs with non-empty hosts",
// #[test]
// fn uri_() {
//     "input": "/rooibos",
//     "base": "file://tea/",
//     "href": "file://tea/rooibos",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "tea",
//     "hostname": "tea",
//     "port": "",
//     "pathname": "/rooibos",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/?chai",
//     "base": "file://tea/",
//     "href": "file://tea/?chai",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "tea",
//     "hostname": "tea",
//     "port": "",
//     "pathname": "/",
//     "search": "?chai",
//     "hash": ""
// }

//   "# Windows drive letter handling with the 'file:' base URL",
// #[test]
// fn uri_() {
//     "input": "C|",
//     "base": "file://host/dir/file",
//     "href": "file://host/C:",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|",
//     "base": "file://host/D:/dir1/dir2/file",
//     "href": "file://host/C:",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|#",
//     "base": "file://host/dir/file",
//     "href": "file://host/C:#",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|?",
//     "base": "file://host/dir/file",
//     "href": "file://host/C:?",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|/",
//     "base": "file://host/dir/file",
//     "href": "file://host/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|\n/",
//     "base": "file://host/dir/file",
//     "href": "file://host/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|\\",
//     "base": "file://host/dir/file",
//     "href": "file://host/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C",
//     "base": "file://host/dir/file",
//     "href": "file://host/dir/C",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/dir/C",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "C|a",
//     "base": "file://host/dir/file",
//     "href": "file://host/dir/C|a",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/dir/C|a",
//     "search": "",
//     "hash": ""
// }

//   "# Windows drive letter quirk in the file slash state",
// #[test]
// fn uri_() {
//     "input": "/c:/foo/bar",
//     "base": "file:///c:/baz/qux",
//     "href": "file:///c:/foo/bar",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/c:/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/c|/foo/bar",
//     "base": "file:///c:/baz/qux",
//     "href": "file:///c:/foo/bar",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/c:/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:\\c:\\foo\\bar",
//     "base": "file:///c:/baz/qux",
//     "href": "file:///c:/foo/bar",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/c:/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/c:/foo/bar",
//     "base": "file://host/path",
//     "href": "file://host/c:/foo/bar",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/c:/foo/bar",
//     "search": "",
//     "hash": ""
// }

//   "# Do not drop the host in the presence of a drive letter",
// #[test]
// fn uri_() {
//     "input": "file://example.net/C:/",
//     "base": null,
//     "href": "file://example.net/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "example.net",
//     "hostname": "example.net",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://1.2.3.4/C:/",
//     "base": null,
//     "href": "file://1.2.3.4/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "1.2.3.4",
//     "hostname": "1.2.3.4",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://[1::8]/C:/",
//     "base": null,
//     "href": "file://[1::8]/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "[1::8]",
//     "hostname": "[1::8]",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

//   "# Copy the host from the base URL in the following cases",
// #[test]
// fn uri_() {
//     "input": "C|/",
//     "base": "file://host/",
//     "href": "file://host/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/C:/",
//     "base": "file://host/",
//     "href": "file://host/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:C:/",
//     "base": "file://host/",
//     "href": "file://host/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:/C:/",
//     "base": "file://host/",
//     "href": "file://host/C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "host",
//     "hostname": "host",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

//   "# Copy the empty host from the input in the following cases",
// #[test]
// fn uri_() {
//     "input": "//C:/",
//     "base": "file://host/",
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://C:/",
//     "base": "file://host/",
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "///C:/",
//     "base": "file://host/",
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///C:/",
//     "base": "file://host/",
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

//   "# Windows drive letter quirk (no host)",
// #[test]
// fn uri_() {
//     "input": "file:/C|/",
//     "base": null,
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://C|/",
//     "base": null,
//     "href": "file:///C:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/C:/",
//     "search": "",
//     "hash": ""
// }

//   "# file URLs without base URL by Rimas Misevičius",
// #[test]
// fn uri_() {
//     "input": "file:",
//     "base": null,
//     "href": "file:///",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:?q=v",
//     "base": null,
//     "href": "file:///?q=v",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "?q=v",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:#frag",
//     "base": null,
//     "href": "file:///#frag",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": "#frag"
// }

//   "# file: drive letter cases from https://crbug.com/1078698",
// #[test]
// fn uri_() {
//     "input": "file:///Y:",
//     "base": null,
//     "href": "file:///Y:",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/Y:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///Y:/",
//     "base": null,
//     "href": "file:///Y:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/Y:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///./Y",
//     "base": null,
//     "href": "file:///Y",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/Y",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///./Y:",
//     "base": null,
//     "href": "file:///Y:",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/Y:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\\\\\\.\\Y:",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

//   "# file: drive letter cases from https://crbug.com/1078698 but lowercased",
// #[test]
// fn uri_() {
//     "input": "file:///y:",
//     "base": null,
//     "href": "file:///y:",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/y:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///y:/",
//     "base": null,
//     "href": "file:///y:/",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/y:/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///./y",
//     "base": null,
//     "href": "file:///y",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/y",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///./y:",
//     "base": null,
//     "href": "file:///y:",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/y:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "\\\\\\.\\y:",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

//   "# Additional file URL tests for (https://github.com/whatwg/url/issues/405)",
// #[test]
// fn uri_() {
//     "input": "file://localhost//a//../..//foo",
//     "base": null,
//     "href": "file://///foo",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "///foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://localhost////foo",
//     "base": null,
//     "href": "file://////foo",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "////foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:////foo",
//     "base": null,
//     "href": "file:////foo",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//foo",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///one/two",
//     "base": "file:///",
//     "href": "file:///one/two",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/one/two",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:////one/two",
//     "base": "file:///",
//     "href": "file:////one/two",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//one/two",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "//one/two",
//     "base": "file:///",
//     "href": "file://one/two",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "one",
//     "hostname": "one",
//     "port": "",
//     "pathname": "/two",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "///one/two",
//     "base": "file:///",
//     "href": "file:///one/two",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/one/two",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "////one/two",
//     "base": "file:///",
//     "href": "file:////one/two",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//one/two",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:///.//",
//     "base": "file:////",
//     "href": "file:////",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

//   "File URL tests for https://github.com/whatwg/url/issues/549",
// #[test]
// fn uri_() {
//     "input": "file:.//p",
//     "base": null,
//     "href": "file:////p",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//p",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file:/.//p",
//     "base": null,
//     "href": "file:////p",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//p",
//     "search": "",
//     "hash": ""
// }

//   "# IPv6 tests",
// #[test]
// fn uri_() {
//     "input": "http://[1:0::]",
//     "base": "http://example.net/",
//     "href": "http://[1::]/",
//     "origin": "http://[1::]",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "[1::]",
//     "hostname": "[1::]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://[0:1:2:3:4:5:6:7:8]",
//     "base": "http://example.net/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     let input = "https://[0::0::0]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://[0:.0]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://[0:0:]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://[0:1:2:3:4:5:6:7.0.0.0.1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://[0:1.00.0.0.0]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://[0:1.290.0.0.0]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://[0:1.23.23]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "# Empty host",
// #[test]
// fn uri_() {
//     let input = "http://?";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://#";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Port overflow (2^32 + 81)",
// #[test]
// fn uri_() {
//     "input": "http://f:4294967377/c",
//     "base": "http://example.org/",
//     "failure": true
// }

//   "Port overflow (2^64 + 81)",
// #[test]
// fn uri_() {
//     "input": "http://f:18446744073709551697/c",
//     "base": "http://example.org/",
//     "failure": true
// }

//   "Port overflow (2^128 + 81)",
// #[test]
// fn uri_() {
//     "input": "http://f:340282366920938463463374607431768211537/c",
//     "base": "http://example.org/",
//     "failure": true
// }

//   "# Non-special-URL path tests",
// #[test]
// fn uri_() {
//     "input": "sc://ñ",
//     "base": null,
//     "href": "sc://%C3%B1",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%C3%B1",
//     "hostname": "%C3%B1",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "sc://ñ?x",
//     "base": null,
//     "href": "sc://%C3%B1?x",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%C3%B1",
//     "hostname": "%C3%B1",
//     "port": "",
//     "pathname": "",
//     "search": "?x",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "sc://ñ#x",
//     "base": null,
//     "href": "sc://%C3%B1#x",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%C3%B1",
//     "hostname": "%C3%B1",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": "#x"
// }

// #[test]
// fn uri_() {
//     "input": "#x",
//     "base": "sc://ñ",
//     "href": "sc://%C3%B1#x",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%C3%B1",
//     "hostname": "%C3%B1",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": "#x"
// }

// #[test]
// fn uri_() {
//     "input": "?x",
//     "base": "sc://ñ",
//     "href": "sc://%C3%B1?x",
//     "origin": "null",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "%C3%B1",
//     "hostname": "%C3%B1",
//     "port": "",
//     "pathname": "",
//     "search": "?x",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "sc://?",
//     "base": null,
//     "href": "sc://?",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "sc://#",
//     "base": null,
//     "href": "sc://#",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "///",
//     "base": "sc://x/",
//     "href": "sc:///",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "////",
//     "base": "sc://x/",
//     "href": "sc:////",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "////x/",
//     "base": "sc://x/",
//     "href": "sc:////x/",
//     "protocol": "sc:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//x/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "tftp://foobar.com/someconfig;mode=netascii",
//     "base": null,
//     "href": "tftp://foobar.com/someconfig;mode=netascii",
//     "origin": "null",
//     "protocol": "tftp:",
//     "username": "",
//     "password": "",
//     "host": "foobar.com",
//     "hostname": "foobar.com",
//     "port": "",
//     "pathname": "/someconfig;mode=netascii",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "telnet://user:pass@foobar.com:23/",
//     "base": null,
//     "href": "telnet://user:pass@foobar.com:23/",
//     "origin": "null",
//     "protocol": "telnet:",
//     "username": "user",
//     "password": "pass",
//     "host": "foobar.com:23",
//     "hostname": "foobar.com",
//     "port": "23",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ut2004://10.10.10.10:7777/Index.ut2",
//     "base": null,
//     "href": "ut2004://10.10.10.10:7777/Index.ut2",
//     "origin": "null",
//     "protocol": "ut2004:",
//     "username": "",
//     "password": "",
//     "host": "10.10.10.10:7777",
//     "hostname": "10.10.10.10",
//     "port": "7777",
//     "pathname": "/Index.ut2",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "redis://foo:bar@somehost:6379/0?baz=bam&qux=baz",
//     "base": null,
//     "href": "redis://foo:bar@somehost:6379/0?baz=bam&qux=baz",
//     "origin": "null",
//     "protocol": "redis:",
//     "username": "foo",
//     "password": "bar",
//     "host": "somehost:6379",
//     "hostname": "somehost",
//     "port": "6379",
//     "pathname": "/0",
//     "search": "?baz=bam&qux=baz",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "rsync://foo@host:911/sup",
//     "base": null,
//     "href": "rsync://foo@host:911/sup",
//     "origin": "null",
//     "protocol": "rsync:",
//     "username": "foo",
//     "password": "",
//     "host": "host:911",
//     "hostname": "host",
//     "port": "911",
//     "pathname": "/sup",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "git://github.com/foo/bar.git",
//     "base": null,
//     "href": "git://github.com/foo/bar.git",
//     "origin": "null",
//     "protocol": "git:",
//     "username": "",
//     "password": "",
//     "host": "github.com",
//     "hostname": "github.com",
//     "port": "",
//     "pathname": "/foo/bar.git",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "irc://myserver.com:6999/channel?passwd",
//     "base": null,
//     "href": "irc://myserver.com:6999/channel?passwd",
//     "origin": "null",
//     "protocol": "irc:",
//     "username": "",
//     "password": "",
//     "host": "myserver.com:6999",
//     "hostname": "myserver.com",
//     "port": "6999",
//     "pathname": "/channel",
//     "search": "?passwd",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "dns://fw.example.org:9999/foo.bar.org?type=TXT",
//     "base": null,
//     "href": "dns://fw.example.org:9999/foo.bar.org?type=TXT",
//     "origin": "null",
//     "protocol": "dns:",
//     "username": "",
//     "password": "",
//     "host": "fw.example.org:9999",
//     "hostname": "fw.example.org",
//     "port": "9999",
//     "pathname": "/foo.bar.org",
//     "search": "?type=TXT",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "ldap://localhost:389/ou=People,o=JNDITutorial",
//     "base": null,
//     "href": "ldap://localhost:389/ou=People,o=JNDITutorial",
//     "origin": "null",
//     "protocol": "ldap:",
//     "username": "",
//     "password": "",
//     "host": "localhost:389",
//     "hostname": "localhost",
//     "port": "389",
//     "pathname": "/ou=People,o=JNDITutorial",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "git+https://github.com/foo/bar",
//     "base": null,
//     "href": "git+https://github.com/foo/bar",
//     "origin": "null",
//     "protocol": "git+https:",
//     "username": "",
//     "password": "",
//     "host": "github.com",
//     "hostname": "github.com",
//     "port": "",
//     "pathname": "/foo/bar",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "urn:ietf:rfc:2648",
//     "base": null,
//     "href": "urn:ietf:rfc:2648",
//     "origin": "null",
//     "protocol": "urn:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "ietf:rfc:2648",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "tag:joe@example.org,2001:foo/bar",
//     "base": null,
//     "href": "tag:joe@example.org,2001:foo/bar",
//     "origin": "null",
//     "protocol": "tag:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "joe@example.org,2001:foo/bar",
//     "search": "",
//     "hash": ""
// }

//   "Serialize /. in path",
// #[test]
// fn uri_() {
//     "input": "non-spec:/.//",
//     "base": null,
//     "href": "non-spec:/.//",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-spec:/..//",
//     "base": null,
//     "href": "non-spec:/.//",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-spec:/a/..//",
//     "base": null,
//     "href": "non-spec:/.//",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-spec:/.//path",
//     "base": null,
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-spec:/..//path",
//     "base": null,
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-spec:/a/..//path",
//     "base": null,
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/.//path",
//     "base": "non-spec:/p",
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "/..//path",
//     "base": "non-spec:/p",
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "..//path",
//     "base": "non-spec:/p",
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "a/..//path",
//     "base": "non-spec:/p",
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "",
//     "base": "non-spec:/..//p",
//     "href": "non-spec:/.//p",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//p",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "path",
//     "base": "non-spec:/..//p",
//     "href": "non-spec:/.//path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "//path",
//     "search": "",
//     "hash": ""
// }

//   "Do not serialize /. in path",
// #[test]
// fn uri_() {
//     "input": "../path",
//     "base": "non-spec:/.//p",
//     "href": "non-spec:/path",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/path",
//     "search": "",
//     "hash": ""
// }

//   "# percent encoded hosts in non-special-URLs",
// #[test]
// fn uri_() {
//     "input": "non-special://%E2%80%A0/",
//     "base": null,
//     "href": "non-special://%E2%80%A0/",
//     "protocol": "non-special:",
//     "username": "",
//     "password": "",
//     "host": "%E2%80%A0",
//     "hostname": "%E2%80%A0",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special://H%4fSt/path",
//     "base": null,
//     "href": "non-special://H%4fSt/path",
//     "protocol": "non-special:",
//     "username": "",
//     "password": "",
//     "host": "H%4fSt",
//     "hostname": "H%4fSt",
//     "port": "",
//     "pathname": "/path",
//     "search": "",
//     "hash": ""
// }

//   "# IPv6 in non-special-URLs",
// #[test]
// fn uri_() {
//     "input": "non-special://[1:2:0:0:5:0:0:0]/",
//     "base": null,
//     "href": "non-special://[1:2:0:0:5::]/",
//     "protocol": "non-special:",
//     "username": "",
//     "password": "",
//     "host": "[1:2:0:0:5::]",
//     "hostname": "[1:2:0:0:5::]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special://[1:2:0:0:0:0:0:3]/",
//     "base": null,
//     "href": "non-special://[1:2::3]/",
//     "protocol": "non-special:",
//     "username": "",
//     "password": "",
//     "host": "[1:2::3]",
//     "hostname": "[1:2::3]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special://[1:2::3]:80/",
//     "base": null,
//     "href": "non-special://[1:2::3]:80/",
//     "protocol": "non-special:",
//     "username": "",
//     "password": "",
//     "host": "[1:2::3]:80",
//     "hostname": "[1:2::3]",
//     "port": "80",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "non-special://[:80/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "blob:https://example.com:443/",
//     "base": null,
//     "href": "blob:https://example.com:443/",
//     "origin": "https://example.com",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "https://example.com:443/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:http://example.org:88/",
//     "base": null,
//     "href": "blob:http://example.org:88/",
//     "origin": "http://example.org:88",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "http://example.org:88/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:d3958f5c-0777-0845-9dcf-2cb28783acaf",
//     "base": null,
//     "href": "blob:d3958f5c-0777-0845-9dcf-2cb28783acaf",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "d3958f5c-0777-0845-9dcf-2cb28783acaf",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:",
//     "base": null,
//     "href": "blob:",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }

//   "blob: in blob:",
// #[test]
// fn uri_() {
//     "input": "blob:blob:",
//     "base": null,
//     "href": "blob:blob:",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "blob:",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:blob:https://example.org/",
//     "base": null,
//     "href": "blob:blob:https://example.org/",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "blob:https://example.org/",
//     "search": "",
//     "hash": ""
// }

//   "Non-http(s): in blob:",
// #[test]
// fn uri_() {
//     "input": "blob:about:blank",
//     "base": null,
//     "href": "blob:about:blank",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "about:blank",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:file://host/path",
//     "base": null,
//     "href": "blob:file://host/path",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "file://host/path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:ftp://host/path",
//     "base": null,
//     "href": "blob:ftp://host/path",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "ftp://host/path",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:ws://example.org/",
//     "base": null,
//     "href": "blob:ws://example.org/",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "ws://example.org/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "blob:wss://example.org/",
//     "base": null,
//     "href": "blob:wss://example.org/",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "wss://example.org/",
//     "search": "",
//     "hash": ""
// }

//   "Percent-encoded http: in blob:",
// #[test]
// fn uri_() {
//     "input": "blob:http%3a//example.org/",
//     "base": null,
//     "href": "blob:http%3a//example.org/",
//     "origin": "null",
//     "protocol": "blob:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "http%3a//example.org/",
//     "search": "",
//     "hash": ""
// }

//   "Invalid IPv4 radix digits",
// #[test]
// fn uri_() {
//     "input": "http://0x7f.0.0.0x7g",
//     "base": null,
//     "href": "http://0x7f.0.0.0x7g/",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "0x7f.0.0.0x7g",
//     "hostname": "0x7f.0.0.0x7g",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://0X7F.0.0.0X7G",
//     "base": null,
//     "href": "http://0x7f.0.0.0x7g/",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "0x7f.0.0.0x7g",
//     "hostname": "0x7f.0.0.0x7g",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Invalid IPv4 portion of IPv6 address",
// #[test]
// fn uri_() {
//     let input = "http://[::127.0.0.0.1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Uncompressed IPv6 addresses with 0",
// #[test]
// fn uri_() {
//     "input": "http://[0:1:0:1:0:1:0:1]",
//     "base": null,
//     "href": "http://[0:1:0:1:0:1:0:1]/",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "[0:1:0:1:0:1:0:1]",
//     "hostname": "[0:1:0:1:0:1:0:1]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://[1:0:1:0:1:0:1:0]",
//     "base": null,
//     "href": "http://[1:0:1:0:1:0:1:0]/",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "[1:0:1:0:1:0:1:0]",
//     "hostname": "[1:0:1:0:1:0:1:0]",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": ""
// }

//   "Percent-encoded query and fragment",
// #[test]
// fn uri_() {
//     "input": "http://example.org/test?\u0022",
//     "base": null,
//     "href": "http://example.org/test?%22",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?%22",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?\u0023",
//     "base": null,
//     "href": "http://example.org/test?#",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?\u003C",
//     "base": null,
//     "href": "http://example.org/test?%3C",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?%3C",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?\u003E",
//     "base": null,
//     "href": "http://example.org/test?%3E",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?%3E",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?\u2323",
//     "base": null,
//     "href": "http://example.org/test?%E2%8C%A3",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?%E2%8C%A3",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?%23%23",
//     "base": null,
//     "href": "http://example.org/test?%23%23",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?%23%23",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?%GH",
//     "base": null,
//     "href": "http://example.org/test?%GH",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?%GH",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?a#%EF",
//     "base": null,
//     "href": "http://example.org/test?a#%EF",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?a",
//     "hash": "#%EF"
// }

// #[test]
// fn uri_() {
//     "input": "http://example.org/test?a#%GH",
//     "base": null,
//     "href": "http://example.org/test?a#%GH",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?a",
//     "hash": "#%GH"
// }

//   "URLs that require a non-about:blank base. (Also serve as invalid base tests.)",
// #[test]
// fn uri_() {
//     "input": "a",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     "input": "a/",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     "input": "a//",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

//   "Bases that don't fail to parse but fail to be bases",
// #[test]
// fn uri_() {
//     "input": "test-a-colon.html",
//     "base": "a:",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "test-a-colon-b.html",
//     "base": "a:b",
//     "failure": true
// }

//   "Other base URL tests, that must succeed",
// #[test]
// fn uri_() {
//     "input": "test-a-colon-slash.html",
//     "base": "a:/",
//     "href": "a:/test-a-colon-slash.html",
//     "protocol": "a:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test-a-colon-slash.html",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "test-a-colon-slash-slash.html",
//     "base": "a://",
//     "href": "a:///test-a-colon-slash-slash.html",
//     "protocol": "a:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test-a-colon-slash-slash.html",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "test-a-colon-slash-b.html",
//     "base": "a:/b",
//     "href": "a:/test-a-colon-slash-b.html",
//     "protocol": "a:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test-a-colon-slash-b.html",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "test-a-colon-slash-slash-b.html",
//     "base": "a://b",
//     "href": "a://b/test-a-colon-slash-slash-b.html",
//     "protocol": "a:",
//     "username": "",
//     "password": "",
//     "host": "b",
//     "hostname": "b",
//     "port": "",
//     "pathname": "/test-a-colon-slash-slash-b.html",
//     "search": "",
//     "hash": ""
// }

//   "Null code point in fragment",
// #[test]
// fn uri_() {
//     "input": "http://example.org/test?a#b\u0000c",
//     "base": null,
//     "href": "http://example.org/test?a#b%00c",
//     "protocol": "http:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?a",
//     "hash": "#b%00c"
// }

// #[test]
// fn uri_() {
//     "input": "non-spec://example.org/test?a#b\u0000c",
//     "base": null,
//     "href": "non-spec://example.org/test?a#b%00c",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/test",
//     "search": "?a",
//     "hash": "#b%00c"
// }

// #[test]
// fn uri_() {
//     "input": "non-spec:/test?a#b\u0000c",
//     "base": null,
//     "href": "non-spec:/test?a#b%00c",
//     "protocol": "non-spec:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "?a",
//     "hash": "#b%00c"
// }

//   "First scheme char - not allowed: https://github.com/whatwg/url/issues/464",
// #[test]
// fn uri_() {
//     "input": "10.0.0.7:8080/foo.html",
//     "base": "file:///some/dir/bar.html",
//     "href": "file:///some/dir/10.0.0.7:8080/foo.html",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/some/dir/10.0.0.7:8080/foo.html",
//     "search": "",
//     "hash": ""
// }

//   "Subsequent scheme chars - not allowed",
// #[test]
// fn uri_() {
//     "input": "a!@$*=/foo.html",
//     "base": "file:///some/dir/bar.html",
//     "href": "file:///some/dir/a!@$*=/foo.html",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/some/dir/a!@$*=/foo.html",
//     "search": "",
//     "hash": ""
// }

//   "First and subsequent scheme chars - allowed",
// #[test]
// fn uri_() {
//     "input": "a1234567890-+.:foo/bar",
//     "base": "http://example.com/dir/file",
//     "href": "a1234567890-+.:foo/bar",
//     "protocol": "a1234567890-+.:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "foo/bar",
//     "search": "",
//     "hash": ""
// }

//   "IDNA ignored code points in file URLs hosts",
// #[test]
// fn uri_() {
//     "input": "file://a\u00ADb/p",
//     "base": null,
//     "href": "file://ab/p",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "ab",
//     "hostname": "ab",
//     "port": "",
//     "pathname": "/p",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "file://a%C2%ADb/p",
//     "base": null,
//     "href": "file://ab/p",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "ab",
//     "hostname": "ab",
//     "port": "",
//     "pathname": "/p",
//     "search": "",
//     "hash": ""
// }

//   "IDNA hostnames which get mapped to 'localhost'",
// #[test]
// fn uri_() {
//     "input": "file://loC𝐀𝐋𝐇𝐨𝐬𝐭/usr/bin",
//     "base": null,
//     "href": "file:///usr/bin",
//     "protocol": "file:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/usr/bin",
//     "search": "",
//     "hash": ""
// }

//   "Empty host after the domain to ASCII",
// #[test]
// fn uri_() {
//     let input = "file://\u00ad/p";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://%C2%AD/p";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "file://xn--/p";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "https://bugzilla.mozilla.org/show_bug.cgi?id=1647058",
// #[test]
// fn uri_() {
//     "input": "#link",
//     "base": "https://example.org/##link",
//     "href": "https://example.org/#link",
//     "protocol": "https:",
//     "username": "",
//     "password": "",
//     "host": "example.org",
//     "hostname": "example.org",
//     "port": "",
//     "pathname": "/",
//     "search": "",
//     "hash": "#link"
// }

//   "UTF-8 percent-encode of C0 control percent-encode set and supersets",
// #[test]
// fn uri_() {
//     "input": "non-special:cannot-be-a-base-url-\u0000\u0001\u001F\u001E\u007E\u007F\u0080",
//     "base": null,
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:cannot-be-a-base-url-%00%01%1F%1E~%7F%C2%80",
//     "origin": "null",
//     "password": "",
//     "pathname": "cannot-be-a-base-url-%00%01%1F%1E~%7F%C2%80",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://www.example.com/path{\u007Fpath.html?query'\u007F=query#fragment<\u007Ffragment",
//     "base": null,
//     "hash": "#fragment%3C%7Ffragment",
//     "host": "www.example.com",
//     "hostname": "www.example.com",
//     "href": "https://www.example.com/path%7B%7Fpath.html?query%27%7F=query#fragment%3C%7Ffragment",
//     "origin": "https://www.example.com",
//     "password": "",
//     "pathname": "/path%7B%7Fpath.html",
//     "port": "",
//     "protocol": "https:",
//     "search": "?query%27%7F=query",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://user:pass[\u007F@foo/bar",
//     "base": "http://example.org",
//     "hash": "",
//     "host": "foo",
//     "hostname": "foo",
//     "href": "https://user:pass%5B%7F@foo/bar",
//     "origin": "https://foo",
//     "password": "pass%5B%7F",
//     "pathname": "/bar",
//     "port": "",
//     "protocol": "https:",
//     "search": "",
//     "username": "user"
// }

//   "Tests for the distinct percent-encode sets",
// #[test]
// fn uri_() {
//     "input": "foo:// !\"$%&'()*+,-.;<=>@[\\]^_`{|}~@host/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "foo://%20!%22$%&'()*+,-.%3B%3C%3D%3E%40%5B%5C%5D%5E_%60%7B%7C%7D~@host/",
//     "origin": "null",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": "%20!%22$%&'()*+,-.%3B%3C%3D%3E%40%5B%5C%5D%5E_%60%7B%7C%7D~"
// }

// #[test]
// fn uri_() {
//     "input": "wss:// !\"$%&'()*+,-.;<=>@[]^_`{|}~@host/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "wss://%20!%22$%&'()*+,-.%3B%3C%3D%3E%40%5B%5D%5E_%60%7B%7C%7D~@host/",
//     "origin": "wss://host",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "wss:",
//     "search": "",
//     "username": "%20!%22$%&'()*+,-.%3B%3C%3D%3E%40%5B%5D%5E_%60%7B%7C%7D~"
// }

// #[test]
// fn uri_() {
//     "input": "foo://joe: !\"$%&'()*+,-.:;<=>@[\\]^_`{|}~@host/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "foo://joe:%20!%22$%&'()*+,-.%3A%3B%3C%3D%3E%40%5B%5C%5D%5E_%60%7B%7C%7D~@host/",
//     "origin": "null",
//     "password": "%20!%22$%&'()*+,-.%3A%3B%3C%3D%3E%40%5B%5C%5D%5E_%60%7B%7C%7D~",
//     "pathname": "/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": "joe"
// }

// #[test]
// fn uri_() {
//     "input": "wss://joe: !\"$%&'()*+,-.:;<=>@[]^_`{|}~@host/",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "wss://joe:%20!%22$%&'()*+,-.%3A%3B%3C%3D%3E%40%5B%5D%5E_%60%7B%7C%7D~@host/",
//     "origin": "wss://host",
//     "password": "%20!%22$%&'()*+,-.%3A%3B%3C%3D%3E%40%5B%5D%5E_%60%7B%7C%7D~",
//     "pathname": "/",
//     "port":"",
//     "protocol": "wss:",
//     "search": "",
//     "username": "joe"
// }

// #[test]
// fn uri_() {
//     "input": "foo://!\"$%&'()*+,-.;=_`{}~/",
//     "base": null,
//     "hash": "",
//     "host": "!\"$%&'()*+,-.;=_`{}~",
//     "hostname": "!\"$%&'()*+,-.;=_`{}~",
//     "href":"foo://!\"$%&'()*+,-.;=_`{}~/",
//     "origin": "null",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://!\"$&'()*+,-.;=_`{}~/",
//     "base": null,
//     "hash": "",
//     "host": "!\"$&'()*+,-.;=_`{}~",
//     "hostname": "!\"$&'()*+,-.;=_`{}~",
//     "href":"wss://!\"$&'()*+,-.;=_`{}~/",
//     "origin": "wss://!\"$&'()*+,-.;=_`{}~",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "wss:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://host/ !\"$%&'()*+,-./:;<=>@[\\]^_`{|}~",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "foo://host/%20!%22$%&'()*+,-./:;%3C=%3E@[\\]^_%60%7B|%7D~",
//     "origin": "null",
//     "password": "",
//     "pathname": "/%20!%22$%&'()*+,-./:;%3C=%3E@[\\]^_%60%7B|%7D~",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://host/ !\"$%&'()*+,-./:;<=>@[\\]^_`{|}~",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "wss://host/%20!%22$%&'()*+,-./:;%3C=%3E@[/]^_%60%7B|%7D~",
//     "origin": "wss://host",
//     "password": "",
//     "pathname": "/%20!%22$%&'()*+,-./:;%3C=%3E@[/]^_%60%7B|%7D~",
//     "port":"",
//     "protocol": "wss:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://host/dir/? !\"$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "foo://host/dir/?%20!%22$%&'()*+,-./:;%3C=%3E?@[\\]^_`{|}~",
//     "origin": "null",
//     "password": "",
//     "pathname": "/dir/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "?%20!%22$%&'()*+,-./:;%3C=%3E?@[\\]^_`{|}~",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://host/dir/? !\"$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
//     "base": null,
//     "hash": "",
//     "host": "host",
//     "hostname": "host",
//     "href": "wss://host/dir/?%20!%22$%&%27()*+,-./:;%3C=%3E?@[\\]^_`{|}~",
//     "origin": "wss://host",
//     "password": "",
//     "pathname": "/dir/",
//     "port":"",
//     "protocol": "wss:",
//     "search": "?%20!%22$%&%27()*+,-./:;%3C=%3E?@[\\]^_`{|}~",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "foo://host/dir/# !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
//     "base": null,
//     "hash": "#%20!%22#$%&'()*+,-./:;%3C=%3E?@[\\]^_%60{|}~",
//     "host": "host",
//     "hostname": "host",
//     "href": "foo://host/dir/#%20!%22#$%&'()*+,-./:;%3C=%3E?@[\\]^_%60{|}~",
//     "origin": "null",
//     "password": "",
//     "pathname": "/dir/",
//     "port":"",
//     "protocol": "foo:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "wss://host/dir/# !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
//     "base": null,
//     "hash": "#%20!%22#$%&'()*+,-./:;%3C=%3E?@[\\]^_%60{|}~",
//     "host": "host",
//     "hostname": "host",
//     "href": "wss://host/dir/#%20!%22#$%&'()*+,-./:;%3C=%3E?@[\\]^_%60{|}~",
//     "origin": "wss://host",
//     "password": "",
//     "pathname": "/dir/",
//     "port":"",
//     "protocol": "wss:",
//     "search": "",
//     "username": ""
// }

//   "Ensure that input schemes are not ignored when resolving non-special URLs",
// #[test]
// fn uri_() {
//     "input": "abc:rootless",
//     "base": "abc://host/path",
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href":"abc:rootless",
//     "password": "",
//     "pathname": "rootless",
//     "port":"",
//     "protocol": "abc:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "abc:rootless",
//     "base": "abc:/path",
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href":"abc:rootless",
//     "password": "",
//     "pathname": "rootless",
//     "port":"",
//     "protocol": "abc:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "abc:rootless",
//     "base": "abc:path",
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href":"abc:rootless",
//     "password": "",
//     "pathname": "rootless",
//     "port":"",
//     "protocol": "abc:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "abc:/rooted",
//     "base": "abc://host/path",
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href":"abc:/rooted",
//     "password": "",
//     "pathname": "/rooted",
//     "port":"",
//     "protocol": "abc:",
//     "search": "",
//     "username": ""
// }

//   "Empty query and fragment with blank should throw an error",
// #[test]
// fn uri_() {
//     "input": "#",
//     "base": null,
//     "failure": true,
//     "relativeTo": "any-base"
// }

// #[test]
// fn uri_() {
//     "input": "?",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

//   "Last component looks like a number, but not valid IPv4",
// #[test]
// fn uri_() {
//     "input": "http://1.2.3.4.5",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://1.2.3.4.5.",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     let input = "http://0..0x300/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://0..0x300./";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "http://256.256.256.256.256",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     "input": "http://256.256.256.256.256.",
//     "base": "http://other.com/",
//     "failure": true
// }

// #[test]
// fn uri_() {
//     let input = "http://1.2.3.08";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://1.2.3.08.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://1.2.3.09";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://09.2.3.4";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://09.2.3.4.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://01.2.3.4.5";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://01.2.3.4.5.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://0x100.2.3.4";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://0x100.2.3.4.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://0x1.2.3.4.5";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://0x1.2.3.4.5.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.1.2.3.4";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.1.2.3.4.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.2.3.4";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.2.3.4.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.09";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.09.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.0x4";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.0x4.";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "http://foo.09..",
//     "base": null,
//     "hash": "",
//     "host": "foo.09..",
//     "hostname": "foo.09..",
//     "href":"http://foo.09../",
//     "password": "",
//     "pathname": "/",
//     "port":"",
//     "protocol": "http:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     let input = "http://0999999999999999999/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.0x";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://foo.0XFfFfFfFfFfFfFfFfFfAcE123";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "http://💩.123/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "U+0000 and U+FFFF in various places",
// #[test]
// fn uri_() {
//     let input = "https://\u0000y";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "https://x/\u0000y",
//     "base": null,
//     "hash": "",
//     "host": "x",
//     "hostname": "x",
//     "href": "https://x/%00y",
//     "password": "",
//     "pathname": "/%00y",
//     "port": "",
//     "protocol": "https:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://x/?\u0000y",
//     "base": null,
//     "hash": "",
//     "host": "x",
//     "hostname": "x",
//     "href": "https://x/?%00y",
//     "password": "",
//     "pathname": "/",
//     "port": "",
//     "protocol": "https:",
//     "search": "?%00y",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://x/?#\u0000y",
//     "base": null,
//     "hash": "#%00y",
//     "host": "x",
//     "hostname": "x",
//     "href": "https://x/?#%00y",
//     "password": "",
//     "pathname": "/",
//     "port": "",
//     "protocol": "https:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     let input = "https://\uFFFFy";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "https://x/\uFFFFy",
//     "base": null,
//     "hash": "",
//     "host": "x",
//     "hostname": "x",
//     "href": "https://x/%EF%BF%BFy",
//     "password": "",
//     "pathname": "/%EF%BF%BFy",
//     "port": "",
//     "protocol": "https:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://x/?\uFFFFy",
//     "base": null,
//     "hash": "",
//     "host": "x",
//     "hostname": "x",
//     "href": "https://x/?%EF%BF%BFy",
//     "password": "",
//     "pathname": "/",
//     "port": "",
//     "protocol": "https:",
//     "search": "?%EF%BF%BFy",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://x/?#\uFFFFy",
//     "base": null,
//     "hash": "#%EF%BF%BFy",
//     "host": "x",
//     "hostname": "x",
//     "href": "https://x/?#%EF%BF%BFy",
//     "password": "",
//     "pathname": "/",
//     "port": "",
//     "protocol": "https:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:\u0000y",
//     "base": null,
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:%00y",
//     "password": "",
//     "pathname": "%00y",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:x/\u0000y",
//     "base": null,
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:x/%00y",
//     "password": "",
//     "pathname": "x/%00y",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:x/?\u0000y",
//     "base": null,
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:x/?%00y",
//     "password": "",
//     "pathname": "x/",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "?%00y",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:x/?#\u0000y",
//     "base": null,
//     "hash": "#%00y",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:x/?#%00y",
//     "password": "",
//     "pathname": "x/",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:\uFFFFy",
//     "base": null,
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:%EF%BF%BFy",
//     "password": "",
//     "pathname": "%EF%BF%BFy",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:x/\uFFFFy",
//     "base": null,
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:x/%EF%BF%BFy",
//     "password": "",
//     "pathname": "x/%EF%BF%BFy",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:x/?\uFFFFy",
//     "base": null,
//     "hash": "",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:x/?%EF%BF%BFy",
//     "password": "",
//     "pathname": "x/",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "?%EF%BF%BFy",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "non-special:x/?#\uFFFFy",
//     "base": null,
//     "hash": "#%EF%BF%BFy",
//     "host": "",
//     "hostname": "",
//     "href": "non-special:x/?#%EF%BF%BFy",
//     "password": "",
//     "pathname": "x/",
//     "port": "",
//     "protocol": "non-special:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "",
//     "base": null,
//     "failure": true,
//     "relativeTo": "non-opaque-path-base"
// }

// #[test]
// fn uri_() {
//     "input": "https://example.com/\"quoted\"",
//     "base": null,
//     "hash": "",
//     "host": "example.com",
//     "hostname": "example.com",
//     "href": "https://example.com/%22quoted%22",
//     "origin": "https://example.com",
//     "password": "",
//     "pathname": "/%22quoted%22",
//     "port": "",
//     "protocol": "https:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "input": "https://a%C2%ADb/",
//     "base": null,
//     "hash": "",
//     "host": "ab",
//     "hostname": "ab",
//     "href": "https://ab/",
//     "origin": "https://ab",
//     "password": "",
//     "pathname": "/",
//     "port": "",
//     "protocol": "https:",
//     "search": "",
//     "username": ""
// }

// #[test]
// fn uri_() {
//     "comment": "Empty host after domain to ASCII",
//     let input = "https://\u00AD/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://%C2%AD/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "https://xn--/";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

//   "Non-special schemes that some implementations might incorrectly treat as special",
// #[test]
// fn uri_() {
//     "input": "data://example.com:8080/pathname?search#hash",
//     "base": null,
//     "href": "data://example.com:8080/pathname?search#hash",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "example.com:8080",
//     "hostname": "example.com",
//     "port": "8080",
//     "pathname": "/pathname",
//     "search": "?search",
//     "hash": "#hash"
// }

// #[test]
// fn uri_() {
//     "input": "data:///test",
//     "base": null,
//     "href": "data:///test",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "data://test/a/../b",
//     "base": null,
//     "href": "data://test/b",
//     "origin": "null",
//     "protocol": "data:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/b",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "data://:443";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "data://test:test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "data://[:1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "javascript://example.com:8080/pathname?search#hash",
//     "base": null,
//     "href": "javascript://example.com:8080/pathname?search#hash",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "example.com:8080",
//     "hostname": "example.com",
//     "port": "8080",
//     "pathname": "/pathname",
//     "search": "?search",
//     "hash": "#hash"
// }

// #[test]
// fn uri_() {
//     "input": "javascript:///test",
//     "base": null,
//     "href": "javascript:///test",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "javascript://test/a/../b",
//     "base": null,
//     "href": "javascript://test/b",
//     "origin": "null",
//     "protocol": "javascript:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/b",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "javascript://:443";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "javascript://test:test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "javascript://[:1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "mailto://example.com:8080/pathname?search#hash",
//     "base": null,
//     "href": "mailto://example.com:8080/pathname?search#hash",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "example.com:8080",
//     "hostname": "example.com",
//     "port": "8080",
//     "pathname": "/pathname",
//     "search": "?search",
//     "hash": "#hash"
// }

// #[test]
// fn uri_() {
//     "input": "mailto:///test",
//     "base": null,
//     "href": "mailto:///test",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "mailto://test/a/../b",
//     "base": null,
//     "href": "mailto://test/b",
//     "origin": "null",
//     "protocol": "mailto:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/b",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "mailto://:443";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "mailto://test:test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "mailto://[:1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "intent://example.com:8080/pathname?search#hash",
//     "base": null,
//     "href": "intent://example.com:8080/pathname?search#hash",
//     "origin": "null",
//     "protocol": "intent:",
//     "username": "",
//     "password": "",
//     "host": "example.com:8080",
//     "hostname": "example.com",
//     "port": "8080",
//     "pathname": "/pathname",
//     "search": "?search",
//     "hash": "#hash"
// }

// #[test]
// fn uri_() {
//     "input": "intent:///test",
//     "base": null,
//     "href": "intent:///test",
//     "origin": "null",
//     "protocol": "intent:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "intent://test/a/../b",
//     "base": null,
//     "href": "intent://test/b",
//     "origin": "null",
//     "protocol": "intent:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/b",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "intent://:443";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "intent://test:test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "intent://[:1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "urn://example.com:8080/pathname?search#hash",
//     "base": null,
//     "href": "urn://example.com:8080/pathname?search#hash",
//     "origin": "null",
//     "protocol": "urn:",
//     "username": "",
//     "password": "",
//     "host": "example.com:8080",
//     "hostname": "example.com",
//     "port": "8080",
//     "pathname": "/pathname",
//     "search": "?search",
//     "hash": "#hash"
// }

// #[test]
// fn uri_() {
//     "input": "urn:///test",
//     "base": null,
//     "href": "urn:///test",
//     "origin": "null",
//     "protocol": "urn:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "urn://test/a/../b",
//     "base": null,
//     "href": "urn://test/b",
//     "origin": "null",
//     "protocol": "urn:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/b",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "urn://:443";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "urn://test:test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "urn://[:1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "turn://example.com:8080/pathname?search#hash",
//     "base": null,
//     "href": "turn://example.com:8080/pathname?search#hash",
//     "origin": "null",
//     "protocol": "turn:",
//     "username": "",
//     "password": "",
//     "host": "example.com:8080",
//     "hostname": "example.com",
//     "port": "8080",
//     "pathname": "/pathname",
//     "search": "?search",
//     "hash": "#hash"
// }

// #[test]
// fn uri_() {
//     "input": "turn:///test",
//     "base": null,
//     "href": "turn:///test",
//     "origin": "null",
//     "protocol": "turn:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "turn://test/a/../b",
//     "base": null,
//     "href": "turn://test/b",
//     "origin": "null",
//     "protocol": "turn:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/b",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "turn://:443";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "turn://test:test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "turn://[:1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "stun://example.com:8080/pathname?search#hash",
//     "base": null,
//     "href": "stun://example.com:8080/pathname?search#hash",
//     "origin": "null",
//     "protocol": "stun:",
//     "username": "",
//     "password": "",
//     "host": "example.com:8080",
//     "hostname": "example.com",
//     "port": "8080",
//     "pathname": "/pathname",
//     "search": "?search",
//     "hash": "#hash"
// }

// #[test]
// fn uri_() {
//     "input": "stun:///test",
//     "base": null,
//     "href": "stun:///test",
//     "origin": "null",
//     "protocol": "stun:",
//     "username": "",
//     "password": "",
//     "host": "",
//     "hostname": "",
//     "port": "",
//     "pathname": "/test",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "stun://test/a/../b",
//     "base": null,
//     "href": "stun://test/b",
//     "origin": "null",
//     "protocol": "stun:",
//     "username": "",
//     "password": "",
//     "host": "test",
//     "hostname": "test",
//     "port": "",
//     "pathname": "/b",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     let input = "stun://:443";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "stun://test:test";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     let input = "stun://[:1]";
//     let uri: Uri = Uri::try_from(input);
//     assert!(uri.is_err());
// }

// #[test]
// fn uri_() {
//     "input": "w://x:0",
//     "base": null,
//     "href": "w://x:0",
//     "origin": "null",
//     "protocol": "w:",
//     "username": "",
//     "password": "",
//     "host": "x:0",
//     "hostname": "x",
//     "port": "0",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }

// #[test]
// fn uri_() {
//     "input": "west://x:0",
//     "base": null,
//     "href": "west://x:0",
//     "origin": "null",
//     "protocol": "west:",
//     "username": "",
//     "password": "",
//     "host": "x:0",
//     "hostname": "x",
//     "port": "0",
//     "pathname": "",
//     "search": "",
//     "hash": ""
// }
