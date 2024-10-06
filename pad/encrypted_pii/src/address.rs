use crate::bloom_filter_wrapper::BloomFilterWrapper;
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`

/// Represents an address with encryption and Bloom filter functionality.
pub struct Address(String);

impl Address {
    /// Creates a new Address instance.
    pub fn new(plaintext: String) -> Self {
        Address(plaintext)
    }

    /// Encrypts the address using AES-256-GCM with a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The symmetric key used for encryption.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the encrypted address.
    pub fn encrypt(&self, key: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        let nonce = Nonce::from_slice(&[0u8; 12]); // Use a fixed nonce for deterministic encryption
        cipher
            .encrypt(nonce, self.0.as_bytes())
            .expect("Encryption failure!")
    }

    /// Decrypts the address using AES-256-GCM with a given key.
    ///
    /// # Arguments
    ///
    /// * `encrypted_address` - The encrypted address to decrypt.
    /// * `key` - The symmetric key used for decryption.
    ///
    /// # Returns
    ///
    /// A string representing the decrypted address.
    pub fn decrypt(encrypted_address: &[u8], key: &[u8]) -> String {
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        let nonce = Nonce::from_slice(&[0u8; 12]);

        let decrypted_bytes = cipher
            .decrypt(nonce, encrypted_address)
            .expect("Decryption failure!");
        String::from_utf8(decrypted_bytes).expect("Failed to convert decrypted bytes to string")
    }

    /// Creates a Bloom filter from the address.
    ///
    /// # Returns
    ///
    /// A `BloomFilterWrapper` containing the Bloom filter for the address.
    pub fn create_bloom_filter(&self) -> BloomFilterWrapper {
        let mut bloom = BloomFilter::with_rate(0.01, self.0.len());
        for i in 0..self.0.len() {
            for j in i + 1..=self.0.len() {
                bloom.insert(&self.0[i..j]);
            }
        }
        BloomFilterWrapper(bloom)
    }
}
