use std::simd::ToBytes;

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use bloom::{BloomFilter, ASMS};
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};
use tokio_postgres::{Client, NoTls};

struct Encrypted<T>(T);

fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);

    let private_key_pem = private_key.to_pkcs8().unwrap();
    let public_key_pem = public_key.to_pkcs1().unwrap();

    (private_key_pem, public_key_pem)
}

impl Encrypted<T: ToBytes> {
    fn encrypt(data: T, key: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        let nonce = Nonce::from_slice(&[0u8; 12]); // Use a fixed nonce for deterministic encryption

        cipher
            .encrypt(nonce, address.as_bytes())
            .expect("Encryption failure!")
    }
    fn decrypt(encrypted_data: &[u8], key: &[u8]) -> T {
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        let nonce = Nonce::from_slice(&[0u8; 12]);

        cipher
            .decrypt(nonce, encrypted_data)
            .expect("Decryption failure!")
    }
}

trait Searchable<T: ToBytes> {
    fn create_bloom_filter(&self) -> BloomFilter;
    fn to_bytes(bloom: &BloomFilter) -> Vec<u8> {
        bloom.to_bytes()
    }

    fn to_bloom(bytes: &[u8]) -> BloomFilter {
        BloomFilter::from_bytes(bytes)
    }
}

impl Searchable<T> for Encrypted<T> {
    fn create_bloom_filter(data: T) -> BloomFilter {
        let data = data.to_bytes();
        let mut bloom = BloomFilter::with_rate(0.01, data.len());
        for i in 0..data.len() {
            for j in i + 1..=data.len() {
                bloom.insert(&data[i..j]);
            }
        }
        bloom
    }
}

async fn store_encrypted_address_with_bloom_filter(
    client: &Client,
    user_id: i32,
    encrypted_address: &[u8],
    address: &str,
) {
    let bloom_filter = create_bloom_filter(address);
    let bloom_bytes = bloom_to_bytes(&bloom_filter);

    client
        .execute(
            "INSERT INTO user_data (user_id, encrypted_address, bloom_filter) VALUES ($1, $2, $3)",
            &[&user_id, &encrypted_address, &bloom_bytes],
        )
        .await
        .expect("Failed to store encrypted address and bloom filter");
}

async fn search_by_partial_address(client: &Client, partial_address: &str) -> Vec<i32> {
    let partial_bloom = create_bloom_filter(partial_address);
    let partial_bloom_bytes = bloom_to_bytes(&partial_bloom);

    let rows = client
        .query(
            "SELECT user_id FROM user_data WHERE bloom_filter @> $1::bytea",
            &[&partial_bloom_bytes],
        )
        .await
        .expect("Failed to search by partial address");

    rows.into_iter().map(|row| row.get(0)).collect()
}
