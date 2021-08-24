use std::{
    borrow::Cow,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
    str::FromStr,
};

use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Addr<'a> {
    SocketAddrV4(SocketAddrV4),
    SocketAddrV6(SocketAddrV6),
    Domain(Cow<'a, str>, u16),
}

impl<'a> Addr<'a> {
    pub fn to_owned(&self) -> Addr<'static> {
        match self {
            Addr::SocketAddrV4(addr) => Addr::SocketAddrV4(*addr),
            Addr::SocketAddrV6(addr) => Addr::SocketAddrV6(*addr),
            Addr::Domain(host, port) => Addr::Domain(String::from(host.clone()).into(), *port),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Addr::SocketAddrV4(addr) => addr.ip().octets().to_vec(),
            Addr::SocketAddrV6(addr) => addr.ip().octets().to_vec(),
            Addr::Domain(domain, _port) => {
                let mut vec = Vec::new();
                let mut addr = domain.as_bytes().to_vec();
                vec.push(addr.len() as u8);
                vec.append(&mut addr);
                vec
            }
        }
    }

    pub fn port(&self) -> u16 {
        match self {
            Addr::SocketAddrV4(addr) => addr.port(),
            Addr::SocketAddrV6(addr) => addr.port(),
            Addr::Domain(_host, port) => *port,
        }
    }

    pub fn host(&self) -> String {
        match self {
            Addr::SocketAddrV4(addr) => addr.ip().to_string(),
            Addr::SocketAddrV6(addr) => addr.ip().to_string(),
            Addr::Domain(host, _port) => host.to_string(),
        }
    }
}

impl<'a> FromStr for Addr<'a> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let rsplit: Vec<String> = s.rsplitn(2, ':').map(|x| x.to_string()).collect();
        if rsplit.len() != 2 {
            return Err(Error::ParseHost);
        }
        let (host, port) = (rsplit[1].clone(), rsplit[0].clone());
        let port = port.parse::<u16>().map_err(|_| Error::ParsePort(port))?;

        match s.parse::<SocketAddr>() {
            Ok(socket) => match socket {
                SocketAddr::V4(addr) => Ok(Addr::SocketAddrV4(addr)),
                SocketAddr::V6(addr) => Ok(Addr::SocketAddrV6(addr)),
            },
            Err(_) => Ok(Addr::Domain(host.into(), port)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

    use crate::addr::Addr;

    #[test]
    fn addr_ipv4() {
        assert_eq!(
            "127.0.0.1:8080".parse::<Addr>().unwrap(),
            Addr::SocketAddrV4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))
        );
    }

    #[test]
    fn addr_ipv6() {
        assert_eq!(
            "[2001:0db8:11a3:09d7:1f34:8a2e:07a0:765d]:129"
                .parse::<Addr>()
                .unwrap(),
            Addr::SocketAddrV6(SocketAddrV6::new(
                Ipv6Addr::new(0x2001, 0xdb8, 0x11a3, 0x9d7, 0x1f34, 0x8a2e, 0x7a0, 0x765d),
                129,
                0,
                0
            ))
        );
    }

    #[test]
    fn addr_domain() {
        assert_eq!(
            "test.com:123".parse::<Addr>().unwrap(),
            Addr::Domain("test.com".into(), 123)
        );
    }

    #[test]
    fn addr_err() {
        assert!("127.0.0.1".parse::<Addr>().is_err());
        assert!("test.com".parse::<Addr>().is_err());
    }
}
