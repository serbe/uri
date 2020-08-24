use std::fmt;
use std::net::{SocketAddr, ToSocketAddrs};
use std::ops::Range;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::convert::TryFrom;

use crate::addr::Addr;
use crate::authority::Authority;
use crate::error::{Error, Result};
use crate::range::{get_chunks, RangeUsize};

#[derive(Clone, PartialEq)]
pub struct Uri {
    inner: String,
    scheme: RangeUsize,
    authority: Authority,
    addr: Addr,
    path: Option<RangeUsize>,
    query: Option<RangeUsize>,
    fragment: Option<RangeUsize>,
}

impl TryFrom<String> for Uri {
    type Error = Error;

    fn try_from(value: String) -> Result<Uri> {
        Ok(value.parse()?)
    }
}

impl TryFrom<&String> for Uri {
    type Error = Error;

    fn try_from(value: &String) -> Result<Uri> {
        Ok(value.parse()?)
    }
}

impl TryFrom<&str> for Uri {
    type Error = Error;

    fn try_from(value: &str) -> Result<Uri> {
        Ok(value.parse()?)
    }
}

impl TryFrom<&Uri> for Uri {
    type Error = Error;

    fn try_from(value: &Uri) -> Result<Uri> {
        Ok(value.clone())
    }
}

impl Uri {
    fn new(
        scheme: &str,
        username: Option<&str>,
        password: Option<&str>,
        host: &str,
        port: Option<u16>,
        path: Option<&str>,
        query: Option<&str>,
        fragment: Option<&str>,
    ) -> Result<Self> {
        let mut uri = String::new();
        if !scheme.is_empty() {
            uri.push_str(scheme);
            uri.push_str("://");
        }
        match (username, password) {
            (Some(username), Some(password)) => {
                uri.push_str(username);
                uri.push_str(":");
                uri.push_str(password);
                uri.push_str("@");
            }
            (Some(username), None) => {
                uri.push_str(username);
                uri.push_str("@");
            }
            _ => (),
        }
        uri.push_str(host);
        if let Some(port) = port {
            uri.push_str(":");
            uri.push_str(&port.to_string());
        }
        if let Some(path) = path {
            uri.push_str(path);
        }
        if let Some(query) = query {
            uri.push_str("?");
            uri.push_str(query);
        }
        if let Some(fragment) = fragment {
            uri.push_str("#");
            uri.push_str(fragment);
        }
        uri.parse()
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn scheme(&self) -> &str {
        &self.inner[self.scheme]
    }

    pub fn user_info(&self) -> Option<&str> {
        self.authority.user_info()
    }

    pub fn host(&self) -> &str {
        self.authority.host()
    }

    pub fn host_header(&self) -> String {
        match self.default_port() {
            p if p == 80 || p == 8080 => self.host().to_string(),
            p => format!("{}:{}", self.host(), p),
        }
    }

    pub fn port(&self) -> Option<u16> {
        self.authority.port()
    }

    pub fn default_port(&self) -> u16 {
        let default_port = match self.scheme() {
            "https" => 443,
            "http" => 80,
            "socks5" | "socks5h" => 1080,
            _ => 80,
        };

        match self.authority.port() {
            Some(port) => port,
            None => default_port,
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.path.map(|r| &self.inner[r])
    }

    pub fn query(&self) -> Option<&str> {
        self.query.map(|r| &self.inner[r])
    }

    pub fn fragment(&self) -> Option<&str> {
        self.fragment.map(|r| &self.inner[r])
    }

    pub fn request_uri(&self) -> &str {
        let mut result = "/";

        for v in &[self.path, self.query, self.fragment] {
            if let Some(r) = v {
                result = &self.inner[r.start..];
                break;
            }
        }

        result
    }

    pub fn proxy_request_uri(&self) -> String {
        let mut result = "/";

        for v in &[self.path, self.query, self.fragment] {
            if let Some(r) = v {
                result = &self.inner[r.start..];
                break;
            }
        }

        format!("{}://{}{}", self.scheme(), self.host_port(), result)
    }

    pub fn origin(&self) -> String {
        format!("{}://{}", self.scheme(), self.host_port())
    }

    pub fn host_port(&self) -> String {
        format!("{}:{}", self.host(), self.default_port())
    }

    fn socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(self.host_port().to_socket_addrs()?.collect())
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Ok(self.socket_addrs()?[0])
    }

    pub fn is_ssl(&self) -> bool {
        self.scheme() == "https"
    }

    // pub fn to_vec(&self) -> Vec<u8> {
    //     let mut vec = Vec::new();
    //     vec.push(self.addr_type());
    //     match self.addr {
    //         Addr::Ipv4(_) => vec.append(&mut self.host_vec()),
    //         Addr::Ipv6(_) => vec.append(&mut self.host_vec()),
    //         Addr::Domain(_) => {
    //             let mut addr = self.host_vec();
    //             vec.push(addr.len() as u8);
    //             vec.append(&mut addr);
    //         }
    //     }
    //     vec.append(&mut self.addr_port());
    //     vec
    // }

    pub fn host_vec(&self) -> Vec<u8> {
        self.addr.to_vec()
    }

    pub fn socks_addr_type(&self) -> u8 {
        match self.addr {
            Addr::Ipv4(_) => 1u8,
            Addr::Ipv6(_) => 4u8,
            Addr::Domain(_) => 3u8,
        }
    }

    // fn addr_port(&self) -> Vec<u8> {
    //     let port = self.default_port();
    //     vec![((port >> 8) & 0xff) as u8, (port & 0xff) as u8]
    // }

    // pub fn check_supported_proxy(self) -> Result<Self> {
    //     match self.scheme() {
    //         "http" | "https" | "socks5" | "socks5h" => Ok(self),
    //         s => Err(Error::UnsupportedScheme(s.to_string())),
    //     }
    // }

    pub fn authority(&self) -> Authority {
        self.authority.clone()
    }

    pub fn addr(&self) -> Addr {
        self.addr.clone()
    }

    pub fn base64_auth(&self) -> Option<String> {
        if self.scheme() == "http" || self.scheme() == "https" {
            self.authority.base64_auth()
        } else {
            None
        }
    }

    pub fn set_authority(&self, username: &str, password: &str) -> Result<Self> {
        let username = if username.is_empty() {
            None
        } else {
            Some(username)
        };
        let password = if username.is_none() || password.is_empty() {
            None
        } else {
            Some(password)
        };
        Uri::new(
            self.scheme(),
            username,
            password,
            self.host(),
            self.port(),
            self.path(),
            self.query(),
            self.fragment(),
        )
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uri = self.inner.to_string();
        let auth = self.authority.to_string();
        let start = self.scheme.end + 3;
        uri.replace_range(start..(start + auth.len()), &auth);
        write!(f, "{}", uri)
    }
}

impl fmt::Debug for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uri = self.inner.to_string();
        let auth = self.authority.to_string();
        let start = self.scheme.end + 3;
        uri.replace_range(start..(start + auth.len()), &auth);
        write!(f, "{}", uri)
    }
}

impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.to_string();
        remove_spaces(&mut s);

        let (uri_part, fragment) =
            get_chunks(&s, Some(RangeUsize::new(0, s.len())), "#", true, true);
        let (uri_part, query) = get_chunks(&s, uri_part, "?", true, true);
        let (scheme, maybe_part) = get_chunks(&s, uri_part, ":", true, false);
        let (scheme, mut uri_part) = if let Some(scheme) = scheme {
            let range = Range::from(scheme);
            if s[range.clone()].chars().all(|c| c.is_alphanumeric()) {
                s.replace_range(range.clone(), &s[range].to_lowercase());
                (scheme, maybe_part)
            } else {
                return Err(Error::EmptyScheme);
            }
        } else {
            return Err(Error::EmptyScheme);
        };

        let authority = if let Some(u) = &uri_part {
            let (auth, part) = if s[*u].contains("//") {
                get_chunks(
                    &s,
                    Some(RangeUsize::new(u.start + 2, u.end)),
                    "/",
                    false,
                    false,
                )
            } else {
                get_chunks(&s, uri_part, "/", false, false)
            };
            if let Some(a) = auth {
                uri_part = part;
                s[a].parse::<Authority>()?
            } else {
                return Err(Error::EmptyAuthority);
            }
        } else {
            return Err(Error::EmptyAuthority);
        };

        let addr = authority.host().parse::<Addr>()?;

        let (path, _uri_part) = get_chunks(&s, uri_part, "?", false, false);

        Ok(Uri {
            inner: s,
            scheme,
            authority,
            addr,
            path,
            query,
            fragment,
        })
    }
}

fn remove_spaces(text: &mut String) {
    text.retain(|c| !c.is_whitespace());
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use crate::uri::Uri;

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
        // assert_eq!(s.decode_user(), Some("?am".to_owned()));
        // assert_eq!(s.decode_password(), Some("pa?sword".to_owned()));
    }

    #[test]
    fn host_t1() {
        let uri = "http://www.example.org".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "www.example.org");
    }

    #[test]
    fn host_t2() {
        let uri = "ftp://webmaster@www.example.org/".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "www.example.org");
    }

    #[test]
    fn host_t3() {
        let uri = "http://%3Fam:pa%3Fsword@google.com".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "google.com");
    }

    #[test]
    fn host_t4() {
        let uri = "http://192.168.0.1:8080/".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "192.168.0.1");
    }

    #[test]
    fn host_t5() {
        let uri = "http://[fe80::1]:8080/".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "[fe80::1]");
    }

    #[test]
    fn host_t6() {
        let uri = "mysql://a,b,c/bar".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "a,b,c");
    }

    #[test]
    fn host_t7() {
        let uri = "scheme://!$&'()*+,;=hello!:23/path".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "!$&'()*+,;=hello!");
    }

    #[test]
    fn host_t8() {
        let uri = "myscheme://authority<\"hi\">/foo".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "authority<\"hi\">");
    }

    #[test]
    fn host_t9() {
        let uri = "http://hello.世界.com/foo".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), "hello.世界.com");
    }

    #[test]
    fn host_header_t1() {
        let uri = "http://www.example.org:8080".parse::<Uri>().unwrap();
        assert_eq!(uri.host_header(), "www.example.org");
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
        assert_eq!(uri.default_port(), 443);
    }

    #[test]
    fn default_port_t2() {
        let uri = "https://www.example.org".parse::<Uri>().unwrap();
        assert_eq!(uri.default_port(), 443);
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
        // assert_eq!(u.decode_path(), Some("/file one&two".to_owned()));
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
        // assert_eq!(s.decode_path(), Some("/foo/bar/baz/quux".to_owned()));
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
        // assert_eq!(s.decode_path(), Some("/a b".to_owned()));
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
        assert_eq!(uri.request_uri(), "/foo%2fbar/baz%2Fquux?alt=media");
    }

    #[test]
    fn proxy_request_uri() {
        let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(
            uri.proxy_request_uri(),
            "http://rest.rsc.io:80/foo%2fbar/baz%2Fquux?alt=media"
        );
    }

    #[test]
    fn origin_t1() {
        let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
        assert_eq!(uri.origin(), "http://www.example.org:80");
    }

    #[test]
    fn host_port_t1() {
        let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
        assert_eq!(uri.host_port(), "www.example.org:80");
    }

    // #[test]
    // fn addr_port_t1() {
    //     let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
    //     assert_eq!(uri.addr_port(), vec![0u8, 80u8]);
    // }

    // #[test]
    // fn path_without_leading() {
    //         //     let uri = "http:%2f%2fwww.example.org/?q=rust+language".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     // Opaque:   "%2f%2fwww.example.org/",
    //     u.query = Some("q=rust+language");
    //         // }

    // #[test]
    // fn host_subcomponent3() {
    //         //     let uri = "http://[fe80::1%25en0]/".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     u.host = "[fe80::1%en0]";
    //     assert_eq!(uri.path(), Some("/"));
    //         // }

    // #[test]
    // fn host_and_port_subcomponents3() {
    //         //     let uri = "http://[fe80::1%25en0]:8080/".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     u.host = "[fe80::1%en0]";
    //     u.port = Some("8080");
    //     assert_eq!(uri.path(), Some("/"));
    //         // }

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
    //     assert_eq!(uri.host(), "[fe80::1%25%65%6e%301-._~]");
    //     // assert_eq!(uri.host(), "[fe80::1%en01-._~]");
    //     assert_eq!(uri.port(), Some(8080));
    //     assert_eq!(uri.path(), Some("/"));
    //         }

    //          //      let uri = "http://192.168.0.2:/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		u.host = "192.168.0.2:";
    //      		u.path = Some("/foo");
    //          // }

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
    //      		assert_eq!(uri.host(), "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080");
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
}
