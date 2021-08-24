use crate::{error::Error, uri::Uri};

pub trait IntoUri: IntoUriSealed {}

impl IntoUri for Uri {}
impl IntoUri for String {}
impl<'a> IntoUri for &'a str {}
impl<'a> IntoUri for &'a String {}

pub trait IntoUriSealed {
    fn into_uri(self) -> Result<Uri, Error>;

    fn as_str(&self) -> &str;
}

impl IntoUriSealed for Uri {
    fn into_uri(self) -> Result<Uri, Error> {
        if self.has_authority() {
            Ok(self)
        } else {
            Err(Error::EmptyAuthority)
        }
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl<'a> IntoUriSealed for &'a str {
    fn into_uri(self) -> Result<Uri, Error> {
        Uri::parse(self)?.into_uri()
    }

    fn as_str(&self) -> &str {
        self
    }
}

impl<'a> IntoUriSealed for &'a String {
    fn into_uri(self) -> Result<Uri, Error> {
        (&**self).into_uri()
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl<'a> IntoUriSealed for String {
    fn into_uri(self) -> Result<Uri, Error> {
        (&*self).into_uri()
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_uri_file_scheme() {
        let result = "file:///etc/fstab".into_uri();
        assert!(result.is_err());
    }
}
