#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress(String);

// #[derive(Error, Debug, Clone, PartialEq)]
// #[error("{0} is not a valid email address")]
// pub struct EmailAddressError(String);

type R<T> = Result<T, EmailError>;

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail,
}

impl EmailAddress {
    pub fn new(email: &str) -> R<Self> {
        if email.contains('@') {
            Ok(Self(email.to_string()))
        } else {
            Err(EmailError::InvalidEmail)
        }
    }
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, String> {
        let e = |_: EmailError| "Invalid email address".to_string();
        EmailAddress::new(value).map_err(e)
    }
}
