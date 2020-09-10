use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use crate::utils::is_valid_ups;
use crate::error::{Error, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Addr {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Domain(String),
}

impl Addr {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Addr::Ipv4(ipv4) => ipv4.octets().to_vec(),
            Addr::Ipv6(ipv6) => ipv6.octets().to_vec(),
            Addr::Domain(domain) => {
                let mut vec = Vec::new();
                let mut addr = domain.as_bytes().to_vec();
                vec.push(addr.len() as u8);
                vec.append(&mut addr);
                vec
            }
        }
    }
}

impl FromStr for Addr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with('[') {
            if !s.ends_with(']') {
                return Err(Error::ParseIPv6);
            }
            return s[1..s.len() - 1]
                .parse::<Ipv6Addr>()
                .map(Addr::Ipv6)
                .map_err(Error::StdParseAddr);
        }
        if let Ok(ipv6) = s.parse::<Ipv6Addr>() {
            Ok(Addr::Ipv6(ipv6))
        } else if let Ok(ipv4) = s.parse::<Ipv4Addr>() {
            Ok(Addr::Ipv4(ipv4))
        } else if is_valid_ups(s, false) {
            Ok(Addr::Domain(s.to_string()))
        } else {
            Err(Error::ParseAddr(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use crate::addr::Addr;

    #[test]
    fn addr_ipv4() {
        assert_eq!(
            "127.0.0.1".parse::<Addr>().unwrap(),
            Addr::Ipv4(Ipv4Addr::new(127, 0, 0, 1))
        );
    }

    #[test]
    fn addr_ipv6() {
        assert_eq!(
            "[2001:0db8:11a3:09d7:1f34:8a2e:07a0:765d]"
                .parse::<Addr>()
                .unwrap(),
            Addr::Ipv6(Ipv6Addr::new(
                0x2001, 0xdb8, 0x11a3, 0x9d7, 0x1f34, 0x8a2e, 0x7a0, 0x765d
            ))
        );
    }

    #[test]
    fn addr_domain() {
        assert_eq!(
            "test.com".parse::<Addr>().unwrap(),
            Addr::Domain("test.com".to_string())
        );
    }

    #[test]
    fn addr_err() {
        assert!("127.0.0.1:123".parse::<Addr>().is_err());
        assert!("test.com:80".parse::<Addr>().is_err());
    }
}
