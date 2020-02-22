use std::{io, net, result, string};

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("string from utf8 error")]
    Utf8Error(#[from] string::FromUtf8Error),
    #[error("Net address parse")]
    StdParseAddr(#[from] net::AddrParseError),
    #[error("Parse ip version 6")]
    ParseIPv6,
    #[error("Parse address")]
    ParseAddr,
    #[error("Parse host")]
    ParseHost,
    #[error("Parse port: {0}")]
    ParsePort(String),
    // #[error("Unsupported scheme: {0}")]
    // UnsupportedScheme(String),
    #[error("Empty scheme")]
    EmptyScheme,
    #[error("Empty authority")]
    EmptyAuthority,
    // #[error("Username len more when 255: {0}")]
    // UnameLenOverflow(usize),
    // #[error("Password len more when 255: {0}")]
    // PasswdLenOverflow(usize),
    // #[error("Wrong status: {0}")]
    // WrongStatus(u8),
}
