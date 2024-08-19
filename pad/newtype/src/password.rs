use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// The `Password` struct encapsulates a hashed password value.
///
/// # Examples
///
/// ```
/// let value = b"hello"; // Password value as bytes
/// let salt = b"random_salt_value"; // Salt value as bytes
/// let pepper = Some(b"secret_pepper_value" as &[u8]); // Optional pepper value as bytes
/// let password_with_pepper = Password::new(value, salt, pepper).expect("Failed to create password");
/// let hash_value_with_pepper = password_with_pepper.get_hash();
/// println!("Salted and Peppered hash value: {}", hash_value_with_pepper);
/// ```
///
/// # Salt and Pepper
///
/// **Salt:** A salt should be a unique, random value for each password to ensure that identical passwords do not result in the same hash. It helps prevent rainbow table attacks. A good salt value is at least 16 bytes of cryptographic randomness.
///
/// **Pepper:** A pepper is a secret value that is added to the hashing process. Unlike a salt, the pepper should be kept secret and not stored with the password hashes. It adds an extra layer of security by making it harder for attackers to crack the hash even if they have the salt. A good pepper value is at least 16 bytes of cryptographic randomness and should be securely stored.

#[derive(Debug, Clone, PartialEq)]
pub struct Password(u64);

impl Password {
    /// Creates a new `Password` instance by hashing the given password value with the provided salt and optional pepper.
    ///
    /// # Parameters
    ///
    /// - `value`: The password value to be hashed, as a byte slice.
    /// - `salt`: The salt to be used in the hashing process, as a byte slice.
    /// - `pepper`: An optional pepper to be used in the hashing process, as a byte slice.
    ///
    /// # Returns
    ///
    /// A `Result<Password, &'static str>` containing the resulting hash or an error if the value is less than 8 bytes.
    fn new(value: &str, salt: &[u8], pepper: Option<&[u8]>) -> Result<Self, &'static str> {
        if value.len() <= 8 {
            return Err("Password value must be at least 8 bytes long");
        }
        let mut s = DefaultHasher::new();
        if let Some(pepper_value) = pepper {
            pepper_value.hash(&mut s);
        }
        salt.hash(&mut s);
        value.hash(&mut s);

        let hash = s.finish();
        Ok(Password(hash))
    }

    /// Verifies if the given password, salt, and optional pepper match the stored hash.
    ///
    /// # Parameters
    ///
    /// - `value`: The password value to be verified, as a byte slice.
    /// - `salt`: The salt used in the hashing process, as a byte slice.
    /// - `pepper`: An optional pepper used in the hashing process, as a byte slice.
    ///
    /// # Returns
    ///
    /// A `Result<(), &'static str>` indicating whether the provided value, salt, and pepper match the stored hash or an error if the value is less than 8 bytes.
    pub fn verify(
        &self,
        value: &str,
        salt: &[u8],
        pepper: Option<&[u8]>,
    ) -> Result<(), &'static str> {
        let other_hash = Password::new(value, salt, pepper)?.get_hash();
        if self.get_hash() == other_hash {
            Ok(())
        } else {
            Err("Password verification failed")
        }
    }

    /// Returns the hash value of the password.
    fn get_hash(&self) -> u64 {
        self.0
    }
}

impl Deref for Password {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<u64> for Password {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_with_pepper() {
        let value = "hellohello"; // Password value as bytes
        let salt = b"random_salt_value"; // Salt value as bytes
        let pepper = Some(b"secret_pepper_value" as &[u8]); // Pepper value as bytes

        let password_with_pepper =
            Password::new(value, salt, pepper).expect("Failed to create password");
        let hash_value_with_pepper = password_with_pepper.get_hash();

        println!("Salted and Peppered hash value: {}", hash_value_with_pepper);

        // Use assert to check if the hash is calculated correctly
        assert_eq!(
            hash_value_with_pepper,
            Password::new(value, salt, pepper).unwrap().get_hash()
        );
    }

    #[test]
    fn test_password_without_pepper() {
        let value = "hellohello"; // Password value as bytes
        let salt = b"random_salt_value"; // Salt value as bytes

        let password_without_pepper =
            Password::new(value, salt, None).expect("Failed to create password");

        assert!(password_without_pepper.verify(value, salt, None).is_ok());
    }

    #[test]
    fn test_deref() {
        let value = "hellohello"; // Password value as bytes
        let salt = b"random_salt_value"; // Salt value as bytes
        let pepper = Some(b"secret_pepper_value" as &[u8]); // Pepper value as bytes

        let password = Password::new(value, salt, pepper).expect("Failed to create password");
        let hash_value = *password; // Using deref to get the hash value
        println!("Dereferenced hash value: {}", hash_value);

        // Use assert to check if the dereferenced hash is correct
        assert_eq!(hash_value, password.get_hash());
    }

    #[test]
    fn test_as_ref() {
        let value = "hellohello"; // Password value as bytes
        let salt = b"random_salt_value"; // Salt value as bytes
        let pepper = Some(b"secret_pepper_value" as &[u8]); // Pepper value as bytes

        let password = Password::new(value, salt, pepper).expect("Failed to create password");
        let hash_value = password.as_ref(); // Using as_ref to get the hash value
        println!("AsRef hash value: {}", hash_value);

        // Use assert to check if the as_ref hash is correct
        assert_eq!(hash_value, &password.get_hash());
    }

    #[test]
    fn test_short_password_value() {
        let value = "short"; // Password value as bytes, less than 8 bytes
        let salt = b"random_salt_value"; // Salt value as bytes
        let result = Password::new(value, salt, None);

        assert!(
            result.is_err(),
            "Expected an error for short password value"
        );
        assert_eq!(
            result.unwrap_err(),
            "Password value must be at least 8 bytes long"
        );
    }

    #[test]
    fn test_verify_password() {
        let value = "hellohello"; // Password value as bytes
        let salt = b"random_salt_value"; // Salt value as bytes
        let pepper = Some(b"secret_pepper_value" as &[u8]); // Pepper value as bytes

        let password = Password::new(value, salt, pepper).expect("Failed to create password");

        // Verify incorrect password
        let incorrect_value = "wrongpassword";
        let verify_result = password.verify(incorrect_value, salt, pepper);
        assert!(
            verify_result.is_err(),
            "Expected password verification to fail"
        );
    }
}
