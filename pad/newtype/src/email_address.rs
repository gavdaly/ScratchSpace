#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress(String);

// #[derive(Error, Debug, Clone, PartialEq)]
// #[error("{0} is not a valid email address")]
// pub struct EmailAddressError(String);

impl EmailAddress {
    pub fn new(email: &str) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Self(email.to_string()))
        } else {
            Err("Invalid email address".to_string())
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

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EmailAddress::new(value)
    }
}
