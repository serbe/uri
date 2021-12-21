use std::str::FromStr;

use crate::{Uri, RangeUsize, Error, utils::is_valid_scheme, Authority, uri::Resource};

impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = s.to_string();
        let mut chunk = RangeUsize::new(0, s.len());
        let mut username = None;
        let mut password = None;
        let mut host = None;
        let mut port = None;

        let scheme = get_scheme(s, &mut chunk)?;
        let lower_scheme = &s[scheme].to_lowercase();
        inner.replace_range(scheme.range(), lower_scheme);

        let fragment = get_fragment(s, &mut chunk);
        // dbg!(&fragment);

        let query = get_query(s, &mut chunk);
        // dbg!(&query);

        let authority = get_authority(s, &mut chunk)?;
        // dbg!(&authority);

        let resource = if &inner[RangeUsize::new(scheme.len() + 1, scheme.len() + 3)] == "//" {
            let shift = scheme.len() + 3;
            if let Some(auth) = &authority {
                username = auth.username.map(|username| username.shift(shift));
                password = auth.password.map(|password| password.shift(shift));
                host = Some(auth.host.shift(shift));
                port = auth.port;
            }
            Resource::Url
        } else {
            Resource::Uri
        };
        // dbg!(&resource);

        let path = if chunk.is_empty() { None } else { Some(chunk) };
        // dbg!(&path);

        Ok(Uri {
            resource,
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

fn get_scheme(s: &str, chunk: &mut RangeUsize) -> Result<RangeUsize, Error> {
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

/// scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
fn check_scheme(scheme: &str) -> Result<(), Error> {
    if scheme.is_empty() {
        Err(Error::EmptyScheme)
    } else if is_valid_scheme(scheme) {
        Ok(())
    } else {
        Err(Error::InvalidScheme(scheme.to_string()))
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
fn get_authority(s: &str, chunk: &mut RangeUsize) -> Result<Option<Authority>, Error> {
    if !s[&chunk].starts_with("//") {
        return Ok(None);
    }
    let mut range = RangeUsize::new(chunk.start + 2, chunk.end);
    if let Some(pos) = s[range].find('/') {
        range.end(range.start + pos)
    }
    let authority = s[range].parse::<Authority>()?;
    chunk.start(range.end);
    Ok(Some(authority))
}

// fn get_path(s: &str, chunk: &mut RangeUsize) -> Option<RangeUsize> {
//     s[&chunk]
//         .find('/')
//         .map(|pos| RangeUsize::new(chunk.start + pos, chunk.end))
// }