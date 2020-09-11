use std::convert::TryFrom;
use std::fmt;
use std::net::{SocketAddr, ToSocketAddrs};
use std::ops::Range;
use std::str;
use std::str::FromStr;
use std::string::ToString;

use crate::addr::Addr;
use crate::authority::Authority;
use crate::error::{Error, Result};
use crate::range::RangeUsize;
use crate::utils::{decode, is_valid_scheme};

#[derive(Clone, PartialEq)]
pub struct Uri {
    pub(crate) inner: String,
    pub(crate) scheme: RangeUsize,
    pub(crate) username: Option<RangeUsize>,
    pub(crate) password: Option<RangeUsize>,
    pub(crate) host: Option<RangeUsize>,
    pub(crate) port: Option<u16>,
    pub(crate) path: Option<RangeUsize>,
    pub(crate) query: Option<RangeUsize>,
    pub(crate) fragment: Option<RangeUsize>,
    pub(crate) authority: Authority,
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
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn scheme(&self) -> &str {
        &self.inner[self.scheme]
    }

    pub fn username(&self) -> Option<&str> {
        self.username.map(|username| &self.inner[username])
    }

    pub fn password(&self) -> Option<&str> {
        self.password.map(|password| &self.inner[password])
    }

    pub fn authority(&self) -> Option<Authority> {
        if self.authority.is_empty() {
            None
        } else {
            Some(self.authority.clone())
        }
    }

    pub fn user_info(&self) -> Option<&str> {
        match (self.username, self.password) {
            (Some(username), Some(password)) => Some(&self.inner[username + password]),
            (Some(username), None) => Some(&self.inner[username]),
            _ => None,
        }
    }

    pub fn decode_username(&self) -> Option<String> {
        self.username().and_then(|username| decode(username))
    }

    pub fn decode_password(&self) -> Option<String> {
        self.password().and_then(|password| decode(password))
    }

    pub fn host(&self) -> Option<&str> {
        self.host.map(|host| &self.inner[host])
    }

    pub fn host_str(&self) -> &str {
        self.host.map_or("", |host| &self.inner[host])
    }

    pub fn host_header(&self) -> String {
        match (self.host(), self.default_port()) {
            (Some(host), Some(port)) if port == 80 || port == 8080 => host.to_string(),
            (Some(host), Some(port)) => format!("{}:{}", host, port),
            _ => String::new(),
        }
    }

    pub fn port(&self) -> Option<u16> {
        self.authority().and_then(|authority| authority.port())
    }

    pub fn default_port(&self) -> Option<u16> {
        match self.port() {
            Some(port) => Some(port),
            None => match self.scheme() {
                "acap" => Some(674),
                "afp" => Some(548),
                "dict" => Some(2628),
                "dns" => Some(53),
                "file" => None,
                "ftp" => Some(21),
                "git" => Some(9418),
                "gopher" => Some(70),
                "http" => Some(80),
                "https" => Some(443),
                "imap" => Some(143),
                "ipp" => Some(631),
                "ipps" => Some(631),
                "irc" => Some(194),
                "ircs" => Some(6697),
                "ldap" => Some(389),
                "ldaps" => Some(636),
                "mms" => Some(1755),
                "msrp" => Some(2855),
                "msrps" => None,
                "mtqp" => Some(1038),
                "nfs" => Some(111),
                "nntp" => Some(119),
                "nntps" => Some(563),
                "pop" => Some(110),
                "prospero" => Some(1525),
                "redis" => Some(6379),
                "rsync" => Some(873),
                "rtsp" => Some(554),
                "rtsps" => Some(322),
                "rtspu" => Some(5005),
                "sftp" => Some(22),
                "smb" => Some(445),
                "snmp" => Some(161),
                "socks5" => Some(1080),
                "socks5h" => Some(1080),
                "ssh" => Some(22),
                "steam" => None,
                "svn" => Some(3690),
                "telnet" => Some(23),
                "ventrilo" => Some(3784),
                "vnc" => Some(5900),
                "wais" => Some(210),
                "ws" => Some(80),
                "wss" => Some(443),
                _ => None,
            },
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.path.map(|r| &self.inner[r])
    }

    pub fn decode_path(&self) -> Option<String> {
        self.path().and_then(|path| decode(path))
    }

    pub fn query(&self) -> Option<&str> {
        self.query.map(|range| &self.inner[range])
    }

    pub fn fragment(&self) -> Option<&str> {
        self.fragment.map(|range| &self.inner[range])
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

    pub fn proxy_request_uri(&self) -> &str {
        // let mut result = "/";

        // for v in &[self.path, self.query, self.fragment] {
        //     if let Some(r) = v {
        //         result = &self.inner[r.start..];
        //         break;
        //     }
        // }

        // self.host_port()
        //     .map(|hp| format!("{}://{}{}", self.scheme(), hp, result).as_str())
        &self.inner
    }

    pub fn origin(&self) -> Option<String> {
        self.host_port()
            .map(|hp| format!("{}://{}", self.scheme(), hp))
    }

    pub fn host_port(&self) -> Option<String> {
        match (self.host(), self.default_port()) {
            (Some(host), Some(port)) => Some(format!("{}:{}", host, port)),
            (Some(host), None) => Some(host.to_string()),
            _ => None,
        }
    }

    pub fn socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(self
            .host_port()
            .ok_or(Error::EmptyHost)
            .and_then(|host_port| host_port.to_socket_addrs().map_err(Error::IO))?
            .collect())
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        self.socket_addrs()?
            .pop()
            .map_or(Err(Error::SocketAddr), Ok)
    }

    pub fn is_ssl(&self) -> bool {
        self.scheme() == "https"
    }

    // pub fn host_vec(&self) -> Vec<u8> {
    //     self.addr.to_vec()
    // }

    pub fn has_authority(&self) -> bool {
        !self.authority.is_empty()
    }

    pub fn addr(&self) -> Result<Addr> {
        self.host().ok_or(Error::EmptyHost)?.parse()
    }

    pub fn base64_auth(&self) -> Option<String> {
        if (self.scheme() == "http" || self.scheme() == "https") && self.has_authority() {
            self.authority.base64_auth()
        } else {
            None
        }
    }

    pub fn set_authority(&self, username: &str, password: &str) -> Result<Uri> {
        self.set_username(username)?.set_password(password)
    }

    pub fn set_username(&self, username: &str) -> Result<Uri> {
        let mut uri = self.inner.clone();
        match (self.username, self.host) {
            (_, None) => return Err(Error::EmptyHost),
            (None, Some(host)) => {
                uri.replace_range(
                    Range {
                        start: host.start - 1,
                        end: host.start - 1,
                    },
                    username,
                );
                uri.replace_range(
                    Range {
                        start: host.start - 1,
                        end: host.start - 1,
                    },
                    "@",
                )
            }
            (Some(old_username), Some(_)) => uri.replace_range(old_username.range(), username),
        };
        uri.parse()
    }

    pub fn set_password(&self, password: &str) -> Result<Uri> {
        let mut uri = self.inner.clone();
        match (self.username, self.password, self.host) {
            (_, _, None) => return Err(Error::EmptyHost),
            (None, _, Some(_)) => return Err(Error::EmptyUsername),
            (Some(username), None, Some(_)) => {
                uri.replace_range(
                    Range {
                        start: username.start + 1,
                        end: username.start + 1,
                    },
                    password,
                );
                uri.replace_range(
                    Range {
                        start: username.start + 1,
                        end: username.start + 1,
                    },
                    ":",
                )
            }
            (Some(_), Some(old_password), Some(_)) => {
                uri.replace_range(old_password.range(), password)
            }
        };
        uri.parse()
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
        let mut inner = s.to_string();
        let mut chunk = RangeUsize::new(0, s.len());

        let scheme = get_scheme(s, &mut chunk)?;
        let lower_scheme = &s[scheme].to_lowercase();
        inner.replace_range(scheme.range(), lower_scheme);

        let fragment = get_fragment(s, &mut chunk);

        let query = get_query(s, &mut chunk);

        let authority = get_authority(s, &mut chunk)?;
        let shift = scheme.len() + 3;

        let username = authority.username.map(|username| username.shift(shift));
        let password = authority.password.map(|password| password.shift(shift));
        let host = if authority.is_empty() {
            None
        } else {
            Some(authority.host.shift(shift))
        };
        let port = authority.port;

        let path = get_path(s, &mut chunk);

        Ok(Uri {
            inner,
            scheme,
            username,
            password,
            host,
            port,
            path,
            query,
            fragment,
            authority,
        })
    }
}

/// scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
fn check_scheme(scheme: &str) -> Result<()> {
    if scheme.is_empty() {
        Err(Error::EmptyScheme)
    } else if is_valid_scheme(scheme) {
        Ok(())
    } else {
        Err(Error::InvalidScheme(scheme.to_string()))
    }
}

fn get_scheme(s: &str, chunk: &mut RangeUsize) -> Result<RangeUsize> {
    match s.find(':') {
        Some(pos) => {
            let scheme = RangeUsize::new(0, pos);
            check_scheme(&s[scheme])?;
            chunk.start(scheme.end + 1);
            Ok(scheme)
        }
        None => Err(Error::EmptyScheme),
    }
}

fn get_fragment(s: &str, chunk: &mut RangeUsize) -> Option<RangeUsize> {
    s[&chunk]
        .find('#')
        .filter(|pos| pos <= &s.len())
        .map(|pos| {
            let end = chunk.end;
            chunk.end(chunk.start + pos);
            RangeUsize::new(chunk.start + pos + 1, end)
        })
}

fn get_query(s: &str, chunk: &mut RangeUsize) -> Option<RangeUsize> {
    s[&chunk]
        .find('?')
        .filter(|pos| pos <= &s.len())
        .map(|pos| {
            let end = chunk.end;
            chunk.end(chunk.start + pos);
            RangeUsize::new(chunk.start + pos + 1, end)
        })
}

/// authority = [ userinfo "@" ] host [ ":" port ]
fn get_authority(s: &str, chunk: &mut RangeUsize) -> Result<Authority> {
    if !s[&chunk].starts_with("//") {
        return Ok(Authority::new());
    }
    let mut range = RangeUsize::new(chunk.start + 2, chunk.end);
    s[range].find('/').map(|pos| range.end(range.start + pos));
    let authority = s[range].parse::<Authority>()?;
    chunk.start(range.end);
    Ok(authority)
}

fn get_path(s: &str, chunk: &mut RangeUsize) -> Option<RangeUsize> {
    s[&chunk]
        .find('/')
        .map(|pos| RangeUsize::new(chunk.start + pos, chunk.end))
}

#[cfg(test)]
mod tests {
    use crate::uri::Uri;
    use std::convert::TryFrom;

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
    fn host_t5() {
        let uri = "http://[fe80::1]:8080/".parse::<Uri>().unwrap();
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
        assert_eq!(uri.default_port(), Some(443));
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
        assert_eq!(uri.request_uri(), "/foo%2fbar/baz%2Fquux?alt=media");
    }

    #[test]
    fn proxy_request_uri() {
        let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(
            uri.proxy_request_uri(),
            "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
        );
    }

    #[test]
    fn origin_t1() {
        let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
        assert_eq!(uri.origin(), Some("http://www.example.org:80".to_string()));
    }

    #[test]
    fn host_port_t1() {
        let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
        assert_eq!(uri.host_port(), Some("www.example.org:80".to_string()));
    }

    // #[test]
    // fn addr_port_t1() {
    //     let uri = "http://www.example.org/foo.html".parse::<Uri>().unwrap();
    //     assert_eq!(uri.addr_port(), vec![0u8, 80u8]);
    // }

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
}
