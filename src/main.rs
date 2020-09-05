use std::str::FromStr;

use crate::error::{Error, Result};

// mod addr;
mod error;
// mod range;
// mod authority;
// mod uri;

#[derive(Debug, Clone, PartialEq)]
pub struct Uri {
    inner: String,
    scheme: usize,
    authority_end: Option<usize>,
    path_start: Option<usize>,
    query_start: Option<usize>,
    fragment_start: Option<usize>,
}

fn set_start(start: usize, new_start: Option<usize>, shift: usize) -> usize {
    if let Some(new_start) = new_start {
        new_start + shift
    } else {
        start
    }
}

fn set_end(end: usize, new_end: Option<usize>, shift: usize) -> usize {
    if let Some(new_end) = new_end {
        new_end - shift
    } else {
        end
    }
}

fn check_scheme(scheme: &str) -> Result<()> {
    if scheme.is_empty() {
        return Err(Error::EmptyScheme);
    }
    if scheme.char_indices().all(|(pos, ch)| {
        if pos == 0 {
            ch.is_alphabetic()
        } else {
            ch.is_alphanumeric() || ch == '+' || ch == '.' || ch == '-'
        }
    }) {
        Ok(())
    } else {
        Err(Error::InvalidScheme(scheme.to_string()))
    }
}

fn get_scheme(s: &str) -> Result<usize> {
    if let Some(pos) = s.find(':') {
        let (scheme, _) = s.split_at(pos);
        check_scheme(scheme)?;
        Ok(pos)
    } else {
        Err(Error::EmptyScheme)
    }
}

fn get_fragment(s: &str, start: usize) -> Option<usize> {
    if let Some(pos) = s[start..].find('#') {
        if pos <= s.len() {
        Some(start+pos +1)
        } else { None }
    } else {
        None
    }
}

fn get_query(s: &str, start: usize, end: usize) -> Option<usize> {
    if let Some(pos) = s[start..end].find('?') {
        if pos <= s.len() {
        Some(start+pos +1)
        } else { None }
    } else {
        None
    }
}

fn check_user_info(s: &str, start: usize, end: usize) -> Result<()> {
    if let Some(colon_pos) = s[start..end].find(':') {
        if colon_pos == start {
            Err(Error::EmptyUsername)
        } else if contain_reserver_char(&s[start..start+colon_pos]) {
            Err(Error::InvalidUsername(s[start..start+colon_pos].to_string()))
        } else if contain_reserver_char(&s[start+colon_pos+1..end]) {
            Err(Error::InvalidPassword(s[start+colon_pos..end].to_string()))
        } else {
            Ok(())
        }
    } else {
        if s.is_empty() {
            Err(Error::EmptyUsername)
        } else if contain_reserver_char(&s[start..end]) {
            Err(Error::InvalidUsername(s[start..end].to_string()))
        } else {
            Ok(())
        }
    }?;
    Ok(())
}

fn get_user_info_end(s: &str, start: usize, end: usize) -> Result<Option<usize>> {
    if let Some(pos) = s[start..end].find('@') {
        let _ = check_user_info(s, start, start+pos)?;
        Ok(Some(start+pos))
    } else {
        Ok(None)
    }
}

fn get_host_end(s: &str, start: usize, end: usize) -> Result<usize> {
    if s[start..end].is_empty() {
        Err(Error::EmptyHost)
    } else {
        let split_at = if s[start..end].starts_with('[') && s[start..end].contains(']') {
            "]:"
        } else {
            ":"
        };
        Ok(if let Some(pos) = s[start..end].rfind(split_at) {
            start+pos+split_at.len()
        } else {
            end
        })
    }
}

fn check_port(s: &str, start: usize, end: usize) -> Result<()> {
    s[start..end].parse::<u16>()
        .map(|_| ())
        .map_err(|_| Error::ParsePort(s[start..end].to_string()))
}

fn get_authority_end(s: &str, start: usize, end: usize) -> Result<Option<usize>> {
    if !s[start..end].starts_with("//") {
        return Ok(None)
    }

    let mut start = set_start(start, Some(start), 2);
    let mut end = end;
    let user_info_end = get_user_info_end(s, start, end)?;
    start = set_start(start, user_info_end, 1);
    if let Some(pos) = s[start..end].find('/') {
        end = set_end(end, Some(start+pos), 0);
    }
    let host_end = get_host_end(s, start, end)?;
    start = set_end(start, Some(host_end), 0);
    let _ = check_port(s, start, end)?;
    Ok(Some(end))
}

fn get_path(s: &str, start: usize, end: usize) -> Option<usize> {
    if let Some(pos) = s[start..end].find('/') {
        Some(start+pos)
    } else {
        None
    }
}

impl Uri {
    pub fn scheme(&self) -> &str {
        &self.inner[0..self.scheme]
    }
}

impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let inner = s.to_string();
        let mut start = 0;
        let mut end = s.len();
        let scheme = get_scheme(s)?;
        start = set_start(start, Some(scheme), 1);
        let fragment_start = get_fragment(s, start);
        if let Some(pos) = fragment_start {
            println!("fragment {:?} {:?}", fragment_start, s.get(pos .. end));
            assert_eq!(Some("skdjfnkjsdhfjskdf"), s.get(pos .. end));
        }
        end = set_end(end, fragment_start, 1);

        let query_start = get_query(s, start, end);
        if let Some(pos) = query_start {
            println!("query {:?} {:?}", query_start, s.get(pos .. end));
            assert_eq!(Some("asdew=12;asdwd=234&qwrc=1"), s.get(pos .. end));
        }
        end = set_end(end, query_start, 1);

        let authority_end = get_authority_end(s, start, end)?;
        if let Some(pos) = authority_end {
            println!("authority {:?} {:?}", authority_end, s.get(start+2 .. pos));
            assert_eq!(Some("webmaster:13z@www.example.org:234"), s.get(start+2 .. pos));
        }
        start = set_start(start, authority_end, 0);

        let path_start = get_path(s, start, end);

        Ok(Uri {
            inner,
            scheme,
            authority_end,
            path_start,
            query_start,
            fragment_start,
        })
    }
}

// fn get_range(s: &str, ch: char) {
//     println!("{:?}", s.contains(ch));
//     println!("{:?}", s.find(ch));
// }

// fn get_path_start(s: &str) -> Option<usize> {
//    s.find('/')
// }

fn contain_reserver_char(s: &str) -> bool {
    s.chars()
        .any(|ch| [':', '/', '?', '#', '[', ']', '@'].contains(&ch))
}

// fn check_host(s: &str) -> Result<()> {
//     Ok(())
// }


fn main() -> Result<()> {
    let s = "asd://webmaster:13z@www.example.org:234/sdfsdf/asd?asdew=12;asdwd=234&qwrc=1#skdjfnkjsdhfjskdf";
    let uri = s.parse::<Uri>()?;
    assert_eq!(uri.scheme(), "asd");
    println!("{:?}", uri);
    Ok(())
}
