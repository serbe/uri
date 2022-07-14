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

impl<'a> IntoUri for &'a Uri {
    fn into_uri(self) -> Result<Uri, Error> {
        self.clone().into_uri()
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

impl IntoUri for String {
    fn into_uri(self) -> Result<Uri, Error> {
        (&*self).into_uri()
    }
}

pub fn into_uri<U: IntoUri>(u: U) -> Result<Uri, Error> {
    match u.into_uri() {
        Ok(uri) => Ok(uri),
        Err(_) => Err(Error::IntoUri),
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

    #[test]
    fn into_uri_fn() {
        let result = into_uri("file:///abd/cef");
        assert!(result.is_err());
    }
}
