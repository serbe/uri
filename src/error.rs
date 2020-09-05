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
    #[error("Empty scheme")]
    EmptyScheme,
    #[error("Empty authority")]
    EmptyAuthority,
    #[error("Empty username but have password")]
    EmptyUsername,
    #[error("No get Socket address")]
    SocketAddr,
    #[error("Scheme contains {0}")]
    InvalidScheme(String),
    #[error("Username contains reserver chars (use percent-encoded) {0}")]
    InvalidUsername(String),
    #[error("Password contains reserver chars (use percent-encoded) {0}")]
    InvalidPassword(String),
}
