use crate::{error::Error, uri::Uri};

pub trait IntoUri {
    fn into_uri(self) -> Result<Uri, Error>;
}

impl IntoUri for Uri {
    fn into_uri(self) -> Result<Uri, Error> {
        if self.has_authority() {
            Ok(self)
        } else {
            Err(Error::EmptyAuthority)
        }
    }
}

impl IntoUri for &Uri {
    fn into_uri(self) -> Result<Uri, Error> {
        if self.has_authority() {
            Ok(self.clone())
        } else {
            Err(Error::EmptyAuthority)
        }
    }
}

impl<'a> IntoUri for &'a str {
    fn into_uri(self) -> Result<Uri, Error> {
        Uri::parse(self)?.into_uri()
    }
}

impl<'a> IntoUri for &'a String {
    fn into_uri(self) -> Result<Uri, Error> {
        (&**self).into_uri()
    }
}

impl<'a> IntoUri for String {
    fn into_uri(self) -> Result<Uri, Error> {
        (&*self).into_uri()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_uri_file_scheme() {
        let result = "file:///abd/cef".into_uri();
        assert!(result.is_err());
    }
}
