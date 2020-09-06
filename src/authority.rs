use std::fmt;
use std::str::FromStr;

use base64::encode;
use percent_encoding::percent_decode_str;

use crate::error::{Error, Result};
use crate::utils::{set_start, set_end, RangeUsize};

#[derive(Clone, Debug, PartialEq)]
pub struct Authority {
    inner: String,
    username: Option<RangeUsize>,
    password: Option<RangeUsize>,
    host: RangeUsize,
    port: Option<u16>,
}

impl Authority {
    pub fn username(&self) -> Option<&str> {
        self.username
            .map(|username| &self.inner[username])
    }

    pub fn decode_username(&self) -> Option<String> {
        self.username().and_then(|v| {
            percent_decode_str(v)
                .decode_utf8()
                .map_or(None, |op| Some(op.to_string()))
        })
    }

    pub fn password(&self) -> Option<&str> {
        self.password.map(|password| &self.inner[password])
    }

    pub fn decode_password(&self) -> Option<String> {
        self.password().and_then(|v| {
            percent_decode_str(v)
                .decode_utf8()
                .map_or(None, |op| Some(op.to_string()))
        })
    }

    pub fn user_info(&self) -> Option<&str> {
        match (&self.username_end, &self.password_end) {
            (Some(_), Some(password_end)) => Some(&self.inner[..*password_end]),
            (Some(username_end), None) => Some(&self.inner[..*username_end]),
            _ => None,
        }
    }

    pub fn host(&self) -> &str {
        // &self.inner[self.host]
    }

    pub fn port(&self) -> Option<u16> {
        match &self.port {
            Some(p) => Some(self.inner[*p].parse().unwrap()),
            None => None,
        }
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

impl FromStr for Authority {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let inner = s.to_string();

        let mut start = 0;
        let mut end = s.len();

        let username_end = get_username_end(s)?;
        let user_info_end = get_user_info_end(s)?;
    // start = set_start(start, user_info_end, 1);
    // let authority = s[start..end].parse::<Authority>()?;
    // let host_end = get_host_end(s, start, end)?;
    // start = set_end(start, Some(host_end), 0);
    // if end > start {
    //     let _ = check_port(s, start, end)?;
    // }

        let mut username = None;
        let mut password = None;

        let uri_part = if s.contains('@') {
            let (info, part) = get_chunks(&s, Some(RangeUsize::new(0, s.len())), "@", true, false);
            let (name, pass) = get_chunks(&s, info, ":", true, false);

            username = name;
            password = pass;

            part
        } else {
            Some(RangeUsize::new(0, s.len()))
        };

        let split_by = if s.contains(']') && s.contains('[') {
            "]:"
        } else {
            ":"
        };
        let (host, port) = get_chunks(&s, uri_part, split_by, true, false);
        let host = host.ok_or(Error::ParseHost)?;

        if let Some(p) = port {
            if inner[p].parse::<u16>().is_err() {
                return Err(Error::ParsePort(inner[p].to_string()));
            }
        }

        Ok(Authority {
            inner,
            username_end,
            password,
            host,
            port,
        })
    }
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let auth = if let Some(pass) = self.password {
            let range = Range::from(pass);

            let hidden_pass = "*".repeat(range.len());
            let mut auth = self.inner.to_string();
            auth.replace_range(range, &hidden_pass);

            auth
        } else {
            self.inner.to_string()
        };

        write!(f, "{}", auth)
    }
}

fn contain_reserver_char(s: &str) -> bool {
    s.chars()
        .any(|ch| [':', '/', '?', '#', '[', ']', '@'].contains(&ch))
}

fn check_user_info(s: &str) -> Result<()> {
    if let Some(colon_pos) = s.find(':') {
        if colon_pos == 0 {
            Err(Error::EmptyUsername)
        } else if contain_reserver_char(&s[..colon_pos]) {
            Err(Error::InvalidUsername(
                s[..colon_pos].to_string(),
            ))
        } else if contain_reserver_char(&s[colon_pos + 1..]) {
            Err(Error::InvalidPassword(
                s[colon_pos..].to_string(),
            ))
        } else {
            Ok(())
        }
    } else {
        if s.is_empty() {
            Err(Error::EmptyUsername)
        } else if contain_reserver_char(&s) {
            Err(Error::InvalidUsername(s.to_string()))
        } else {
            Ok(())
        }
    }?;
    Ok(())
}

fn get_user_info_end(s: &str) -> Result<Option<usize>> {
    if let Some(pos) = s.find('@') {
        let _ = check_user_info(&s[..pos])?;
        Ok(Some(pos))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::authority::Authority;

    #[test]
    fn authority_user() {
        let authority = "user:password@host:80".parse::<Authority>().unwrap();
        assert_eq!(authority.username(), Some("user"));
    }

    #[test]
    fn authority_password() {
        let authority = "user:password@host:80".parse::<Authority>().unwrap();
        assert_eq!(authority.password(), Some("password"));
    }

    #[test]
    fn authority_host() {
        let authority = "user:password@host:80".parse::<Authority>().unwrap();
        assert_eq!(authority.host(), "host");
    }

    #[test]
    fn authority_port() {
        let authority = "user:password@host:80".parse::<Authority>().unwrap();
        assert_eq!(authority.port(), Some(80));
    }

    #[test]
    fn authority_userinfo() {
        let authority = "user:password@host:80".parse::<Authority>().unwrap();
        assert_eq!(authority.user_info(), Some("user:password"));
    }

    #[test]
    fn authority_display() {
        let authority = "user:password@host:80".parse::<Authority>().unwrap();
        assert_eq!(&authority.to_string(), "user:********@host:80");
    }
}
