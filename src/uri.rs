// use percent_encoding::{percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::{
    convert::TryFrom,
    fmt,
    net::{SocketAddr, ToSocketAddrs},
    ops::Range,
    str,
    string::ToString,
};

use crate::{addr::Addr, authority::Authority, error::Error, range::RangeUsize, utils::decode};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Resource {
    Uri,
    Url,
    // URN,
}

#[derive(Clone, PartialEq)]
pub struct Uri {
    pub(crate) resource: Resource,
    pub(crate) inner: String,
    pub(crate) scheme: RangeUsize,
    pub(crate) username: Option<RangeUsize>,
    pub(crate) password: Option<RangeUsize>,
    pub(crate) host: Option<RangeUsize>,
    pub(crate) port: Option<u16>,
    pub(crate) path: Option<RangeUsize>,
    pub(crate) query: Option<RangeUsize>,
    pub(crate) fragment: Option<RangeUsize>,
    pub(crate) authority: Option<Authority>,
}

impl TryFrom<String> for Uri {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&String> for Uri {
    type Error = Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Uri {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&Uri> for Uri {
    type Error = Error;

    fn try_from(value: &Uri) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
}

impl Uri {
    pub fn parse(input: &str) -> Result<Uri, Error> {
        input.parse()
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn into_string(self) -> String {
        self.inner
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
        self.authority.clone()
    }

    pub fn user_info(&self) -> Option<&str> {
        match (self.username, self.password) {
            (Some(username), Some(password)) => Some(&self.inner[username + password]),
            (Some(username), None) => Some(&self.inner[username]),
            _ => None,
        }
    }

    pub fn decode_username(&self) -> Option<String> {
        self.username().and_then(decode)
    }

    pub fn decode_password(&self) -> Option<String> {
        self.password().and_then(decode)
    }

    pub fn host(&self) -> Option<&str> {
        self.host.map(|host| &self.inner[host])
    }

    pub fn host_str(&self) -> &str {
        self.host.map_or("", |host| &self.inner[host])
    }

    pub fn host_header(&self) -> &str {
        match (self.host, self.port) {
            (Some(host), Some(port)) if Some(port) == self.default_port() => &self.inner[host],
            (Some(host), Some(port)) => {
                &self.inner[host.start..host.end + format!("{port}").len() + 1]
            }
            (Some(host), None) => &self.inner[host],
            _ => "",
        }
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn default_port(&self) -> Option<u16> {
        match self.scheme() {
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
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.path.map(|r| &self.inner[r])
    }

    pub fn decode_path(&self) -> Option<String> {
        self.path().and_then(decode)
    }

    pub fn query(&self) -> Option<&str> {
        self.query.map(|range| &self.inner[range])
    }

    pub fn fragment(&self) -> Option<&str> {
        self.fragment.map(|range| &self.inner[range])
    }

    pub fn abs_path(&self) -> &str {
        match (self.path, self.query, self.fragment) {
            (Some(path), _, _) => &self.inner[path.range_from()],
            (None, Some(query), _) => &self.inner[query.range_from()],
            (None, None, Some(fragment)) => &self.inner[fragment.range_from()],
            _ => "/",
        }
    }

    pub fn absolute_uri(&self) -> String {
        match self.path() {
            Some(_) => self.inner.to_string(),
            None => format!("{}/", &self.inner),
        }
    }

    pub fn origin(&self) -> Option<String> {
        self.host_port()
            .map(|host_port| format!("{}://{}", self.scheme(), host_port))
    }

    pub fn host_port(&self) -> Option<&str> {
        match (self.host, self.port) {
            (Some(host), Some(port)) => {
                Some(&self.inner[host.start..host.end + format!("{port}").len() + 1])
            }
            (Some(host), None) => Some(&self.inner[host]),
            _ => None,
        }
    }

    pub fn host_with_port(&self) -> Option<String> {
        match (self.host, self.port) {
            (Some(host), Some(port)) => {
                Some(self.inner[host.start..host.end + format!("{port}").len() + 1].to_string())
            }
            (Some(host), None) => Some(format!(
                "{}:{}",
                &self.inner[host],
                self.default_port().map_or(80, |port| port)
            )),
            _ => None,
        }
    }

    pub fn socket_addrs(&self) -> Result<Vec<SocketAddr>, Error> {
        Ok(self
            .host_with_port()
            .ok_or(Error::EmptyHost)
            .and_then(|host_port| host_port.to_socket_addrs().map_err(Error::IO))?
            .collect())
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, Error> {
        self.socket_addrs()?
            .first()
            .cloned()
            .ok_or(Error::SocketAddr)
    }

    pub fn is_ssl(&self) -> bool {
        self.scheme() == "https"
    }

    pub fn has_authority(&self) -> bool {
        self.authority.is_some()
    }

    pub fn base64_auth(&self) -> Option<String> {
        if self.scheme() == "http" || self.scheme() == "https" {
            self.authority
                .as_ref()
                .and_then(|authority| authority.base64_auth())
        } else {
            None
        }
    }

    pub fn set_authority(&self, username: &str, password: &str) -> Result<Uri, Error> {
        self.set_username(username)?.set_password(password)
    }

    pub fn set_username(&self, username: &str) -> Result<Uri, Error> {
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

    pub fn set_password(&self, password: &str) -> Result<Uri, Error> {
        let mut uri = self.inner.clone();
        match (self.username, self.password, self.host) {
            (_, _, None) => Err(Error::EmptyHost),
            (None, _, Some(_)) => Err(Error::EmptyUsername),
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
                );
                Ok(())
            }
            (Some(_), Some(old_password), Some(_)) => {
                uri.replace_range(old_password.range(), password);
                Ok(())
            }
        }?;
        uri.parse()
    }

    pub fn normalize(&self) -> String {
        let mut uri = self.inner.to_string();
        if let Some(host_range) = self.host {
            let normalize_host = self.inner[host_range].to_lowercase();
            uri.replace_range(host_range.range(), &normalize_host);
        }
        if self.is_url() && self.path().is_none() {
            uri.push('/');
        }
        uri
    }

    pub fn is_url(&self) -> bool {
        self.resource == Resource::Url
    }

    pub fn addr(&self) -> Result<Addr, Error> {
        self.host_with_port().ok_or(Error::EmptyHostPort)?.parse()
    }

    pub fn request_uri(&self, proxy: bool) -> String {
        if proxy {
            self.absolute_uri()
        } else {
            self.abs_path().to_string()
        }
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uri = self.inner.to_string();
        if let Some(authority) = &self.authority {
            let auth = authority.to_string();
            let start = self.scheme.end + 3;
            uri.replace_range(start..(start + auth.len()), &auth);
        }
        write!(f, "{uri}")
    }
}

impl fmt::Debug for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uri = self.inner.to_string();
        if let Some(authority) = &self.authority {
            let auth = authority.to_string();
            let start = self.scheme.end + 3;
            uri.replace_range(start..(start + auth.len()), &auth);
        }
        write!(f, "{uri}")
    }
}
