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
    #[error("Empty userinfo before @")]
    EmptyUserInfo,
    #[error("Empty host in authority")]
    EmptyHost,
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

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::IO(self_err), Error::IO(other_err)) => {
                self_err.to_string() == other_err.to_string()
            }
            (Error::Utf8Error(self_err), Error::Utf8Error(other_err)) => self_err == other_err,
            (Error::StdParseAddr(self_err), Error::StdParseAddr(other_err)) => {
                self_err == other_err
            }
            (Error::ParseIPv6, Error::ParseIPv6) => true,
            (Error::ParseAddr, Error::ParseAddr) => true,
            (Error::ParseHost, Error::ParseHost) => true,
            (Error::ParsePort(self_err), Error::ParsePort(other_err)) => self_err == other_err,
            (Error::EmptyScheme, Error::EmptyScheme) => true,
            (Error::EmptyUserInfo, Error::EmptyUserInfo) => true,
            (Error::EmptyHost, Error::EmptyHost) => true,
            (Error::EmptyAuthority, Error::EmptyAuthority) => true,
            (Error::EmptyUsername, Error::EmptyUsername) => true,
            (Error::SocketAddr, Error::SocketAddr) => true,
            (Error::InvalidScheme(self_err), Error::InvalidScheme(other_err)) => {
                self_err == other_err
            }
            (Error::InvalidUsername(self_err), Error::InvalidUsername(other_err)) => {
                self_err == other_err
            }
            (Error::InvalidPassword(self_err), Error::InvalidPassword(other_err)) => {
                self_err == other_err
            }
            _ => false,
        }
    }
}
