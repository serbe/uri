use std::fmt;
use std::str::FromStr;

use base64::encode;

use crate::error::{Error, Result};
use crate::range::RangeUsize;
use crate::utils::decode;

#[derive(Clone, Debug, PartialEq)]
pub struct Authority {
    inner: String,
    pub(crate) username: Option<RangeUsize>,
    pub(crate) password: Option<RangeUsize>,
    pub(crate) host: RangeUsize,
    pub(crate) port: Option<u16>,
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

fn contain_reserver_char(s: &str) -> bool {
    s.chars()
        .any(|ch| [':', '/', '?', '#', '[', ']', '@'].contains(&ch))
}

fn check_user_info(s: &str) -> Result<()> {
    if let Some(colon_pos) = s.find(':') {
        if colon_pos == 0 {
            Err(Error::EmptyUsername)
        } else if colon_pos == s.len() {
            Ok(())
        } else if contain_reserver_char(&s[..colon_pos]) {
            Err(Error::InvalidUsername(s[..colon_pos].to_string()))
        } else if contain_reserver_char(&s[colon_pos + 1..]) {
            Err(Error::InvalidPassword(s[colon_pos + 1..].to_string()))
        } else {
            Ok(())
        }
    } else {
        if s.is_empty() {
            Err(Error::EmptyUserInfo)
        } else if contain_reserver_char(&s) {
            Err(Error::InvalidUsername(s.to_string()))
        } else {
            Ok(())
        }
    }?;
    Ok(())
}

fn get_user_info(s: &str, chunk: &mut RangeUsize) -> Option<RangeUsize> {
    s.find('@').map(|pos| {
        chunk.start(pos + 1);
        RangeUsize::new(0, pos)
    })
}

fn get_username(s: &str) -> Option<RangeUsize> {
    match s.find(':') {
        Some(pos) => Some(RangeUsize::new(0, pos)),
        None => Some(RangeUsize::new(0, s.len())),
    }
}

fn get_password(s: &str) -> Option<RangeUsize> {
    match s.find(':') {
        Some(pos) => Some(RangeUsize::new(pos + 1, s.len())),
        None => None,
    }
}

fn get_host(s: &str, chunk: &mut RangeUsize) -> Result<RangeUsize> {
    if s[&chunk].is_empty() {
        Err(Error::EmptyHost)
    } else {
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
            username = get_username(&inner[range]);
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
    fn reserver_char() {
        let bad_str = "myscheme://authority<\"hi\">/foo";
        assert!(contain_reserver_char(bad_str));
        let good_str = "myschemeauthority<\"hi\">foo";
        assert!(!contain_reserver_char(good_str));
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
    fn get_ui() {
        let mut range = RangeUsize::new(0, 26);
        let good_ui = "username:password@hostname";
        
        assert!(get_user_info(good_ui, &mut range).is_some());
    }
}
