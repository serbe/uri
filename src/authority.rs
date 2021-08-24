use std::fmt;
use std::str::FromStr;

use base64::encode;

use crate::error::{Error, Result};
use crate::range::RangeUsize;
use crate::utils::{check_ups, decode};

/// authority = [ userinfo "@" ] host [ ":" port ]
#[derive(Clone, Debug, PartialEq)]
pub struct Authority {
    inner: String,
    pub(crate) username: Option<RangeUsize>,
    pub(crate) password: Option<RangeUsize>,
    pub(crate) host: RangeUsize,
    pub(crate) port: Option<u16>,
}

impl Default for Authority {
    fn default() -> Self {
        Self::new()
    }
}

impl Authority {
    pub fn new() -> Self {
        Authority {
            inner: String::new(),
            username: None,
            password: None,
            host: RangeUsize::new(0, 0),
            port: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn username(&self) -> Option<&str> {
        self.username.map(|username| &self.inner[username])
    }

    pub fn decode_username(&self) -> Option<String> {
        self.username().and_then(|username| decode(username))
    }

    pub fn password(&self) -> Option<&str> {
        self.password.map(|password| &self.inner[password])
    }

    pub fn decode_password(&self) -> Option<String> {
        self.password().and_then(|password| decode(password))
    }

    pub fn user_info(&self) -> Option<&str> {
        match (&self.username, &self.password) {
            (Some(_), Some(password)) => Some(&self.inner[password.range_to()]),
            (Some(username), None) => Some(&self.inner[username]),
            _ => None,
        }
    }

    pub fn host(&self) -> &str {
        &self.inner[self.host]
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn base64_auth(&self) -> Option<String> {
        match (self.decode_username(), self.decode_password()) {
            (Some(user), Some(pass)) => Some(encode(&format!("{}:{}", user, pass))),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

fn check_username(s: &str) -> Result<()> {
    if s.is_empty() {
        Err(Error::EmptyUsername)
    } else {
        check_ups(s, false, Error::InvalidUsername(s.to_string()))
    }
}

fn check_password(s: &str) -> Result<()> {
    check_ups(s, true, Error::InvalidPassword(s.to_string()))
}

fn check_user_info(s: &str) -> Result<()> {
    if s.is_empty() {
        Err(Error::EmptyUserInfo)
    } else if let Some(colon_pos) = s.find(':') {
        check_username(&s[..colon_pos]).and_then(|_| check_password(&s[colon_pos + 1..]))
    } else {
        check_username(s)
    }
}

/// userinfo = *( unreserved / pct-encoded / sub-delims / ":" )
fn get_user_info(s: &str, chunk: &mut RangeUsize) -> Option<RangeUsize> {
    s.find('@').map(|pos| {
        chunk.start(pos + 1);
        RangeUsize::new(0, pos)
    })
}

fn get_username(s: &str) -> RangeUsize {
    match s.find(':') {
        Some(pos) => RangeUsize::new(0, pos),
        None => RangeUsize::new(0, s.len()),
    }
}

fn get_password(s: &str) -> Option<RangeUsize> {
    s.find(':').map(|pos| RangeUsize::new(pos + 1, s.len()))
}

/// host = IP-literal / IPv4address / reg-name
/// IP-literal = "[" ( IPv6address / IPvFuture  ) "]"
/// IPvFuture  = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
///
///  IPv6address =                            6( h16 ":" ) ls32
///              /                       "::" 5( h16 ":" ) ls32
///              / [               h16 ] "::" 4( h16 ":" ) ls32
///              / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
///              / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
///              / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
///              / [ *4( h16 ":" ) h16 ] "::"              ls32
///              / [ *5( h16 ":" ) h16 ] "::"              h16
///              / [ *6( h16 ":" ) h16 ] "::"
///
///  ls32        = ( h16 ":" h16 ) / IPv4address
///              ; least-significant 32 bits of address
///
///  h16         = 1*4HEXDIG
///              ; 16 bits of address represented in hexadecimal
///
///  IPv4address = dec-octet "." dec-octet "." dec-octet "." dec-octet
///
///  dec-octet   = DIGIT                 ; 0-9
///              / %x31-39 DIGIT         ; 10-99
///              / "1" 2DIGIT            ; 100-199
///              / "2" %x30-34 DIGIT     ; 200-249
///              / "25" %x30-35          ; 250-255
///
///  reg-name    = *( unreserved / pct-encoded / sub-delims )
///
fn get_host(s: &str, chunk: &mut RangeUsize) -> Result<RangeUsize> {
    if s[&chunk].is_empty() {
        return Err(Error::EmptyHost);
    }

    let split_at = if s[&chunk].starts_with('[') && s[&chunk].contains(']') {
        "]:"
    } else {
        ":"
    };
    let start = chunk.start;
    let host = if let Some(pos) = s[&chunk].rfind(split_at) {
        chunk.start(chunk.start + pos + split_at.len());
        RangeUsize::new(start, chunk.start - 1)
    } else {
        chunk.start(chunk.end);
        RangeUsize::new(start, chunk.end)
    };
    Ok(host)
}

impl FromStr for Authority {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let inner = s.to_string();

        let mut chunk = RangeUsize::new(0, s.len());
        let mut username = None;
        let mut password = None;
        let mut port = None;
        let user_info = get_user_info(s, &mut chunk);
        if let Some(range) = user_info {
            check_user_info(&inner[range])?;
            username = Some(get_username(&inner[range]));
            password = get_password(&inner[range]);
        };
        let host = get_host(s, &mut chunk)?;
        if !chunk.is_empty() {
            port = Some(
                s[chunk]
                    .parse::<u16>()
                    .map_err(|_| Error::ParsePort(s[chunk].to_string()))?,
            );
        }

        Ok(Authority {
            inner,
            username,
            password,
            host,
            port,
        })
    }
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let auth = if let Some(pass) = self.password {
            let hidden_pass = "*".repeat(pass.len());
            let mut auth = self.inner.clone();
            auth.replace_range(pass.range(), &hidden_pass);

            auth
        } else {
            self.inner.clone()
        };

        write!(f, "{}", auth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn authority_new() {
        let authority = Authority {
            inner: String::new(),
            username: None,
            password: None,
            host: RangeUsize::new(0, 0),
            port: None,
        };
        assert_eq!(authority, Authority::new());
        let authority = Authority {
            inner: String::new(),
            username: Some(RangeUsize::new(0, 0)),
            password: None,
            host: RangeUsize::new(0, 0),
            port: None,
        };
        assert_ne!(authority, Authority::new());
    }

    #[test]
    fn authority_inner() {
        let authority = "user:password@host:80".parse::<Authority>().unwrap();
        assert_eq!(authority.inner, "user:password@host:80".to_string());
    }

    #[test]
    fn ui() {
        let ui_empty = "";
        assert_eq!(check_user_info(ui_empty), Err(Error::EmptyUserInfo));
        let ui_user_invalid = "user/ame";
        assert_eq!(
            check_user_info(ui_user_invalid),
            Err(Error::InvalidUsername("user/ame".to_string()))
        );
        let ui_good = "username";
        assert_eq!(check_user_info(ui_good), Ok(()));
        let ui_pass_empty = "username:";
        assert_eq!(check_user_info(ui_pass_empty), Ok(()));
        let ui_user_empty = ":password";
        assert_eq!(check_user_info(ui_user_empty), Err(Error::EmptyUsername));
        let ui_user_invalid2 = "user/ame:password";
        assert_eq!(
            check_user_info(ui_user_invalid2),
            Err(Error::InvalidUsername("user/ame".to_string()))
        );
        let ui_pass_invalid = "username:pass/ord";
        assert_eq!(
            check_user_info(ui_pass_invalid),
            Err(Error::InvalidPassword("pass/ord".to_string()))
        );
    }

    #[test]
    fn get_some_ui() {
        let mut range = RangeUsize::new(0, 26);
        let good_ui = "username:password@hostname";
        let expected = RangeUsize::new(0, 17);
        let expected_range = RangeUsize::new(18, 26);
        assert_eq!(get_user_info(good_ui, &mut range), Some(expected));
        assert_eq!(range, expected_range);
    }

    #[test]
    fn get_none_ui() {
        let mut range = RangeUsize::new(0, 28);
        let expected_range = RangeUsize::new(0, 28);
        let good_ui = "username:password%40hostname";
        assert_eq!(get_user_info(good_ui, &mut range), None);
        assert_eq!(range, expected_range);
    }

    #[test]
    fn get_ok_host() {
        let host = "hostname:123";
        let mut range = RangeUsize::new(0, 11);
        let expected = RangeUsize::new(0, 8);
        let expected_range = RangeUsize::new(9, 11);
        assert_eq!(get_host(host, &mut range), Ok(expected));
        assert_eq!(range, expected_range);
    }

    #[test]
    fn get_host_ipv6() {
        let host = "[hostname:123]";
        let mut range = RangeUsize::new(0, 14);
        assert_eq!(&host[range], host);
        assert!(host[range].starts_with('[') && host[range].contains(']'));
        let expected = RangeUsize::new(0, 14);
        let expected_range = RangeUsize::new(14, 14);
        assert_eq!(get_host(host, &mut range), Ok(expected));
        assert_eq!(&host[expected], host);
        assert_eq!(range, expected_range);
    }

    #[test]
    fn get_host_ipv6_v2() {
        let host = "[hostname]";
        let mut range = RangeUsize::new(0, 10);
        assert_eq!(&host[range], host);
        assert!(host[range].starts_with('[') && host[range].contains(']'));
        let expected = RangeUsize::new(0, 10);
        let expected_range = RangeUsize::new(10, 10);
        assert_eq!(get_host(host, &mut range), Ok(expected));
        assert_eq!(&host[expected], host);
        assert_eq!(range, expected_range);
    }
}
