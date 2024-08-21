use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use bloom::{BloomFilter, ASMS};
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};
use tokio_postgres::{Client, NoTls};

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=yourpassword dbname=yourdb",
        NoTls,
    )
    .await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let user_id = 1;
    let address = "123 Main St";

    // Generate asymmetric key pair for user
    let (private_key, public_key) = generate_key_pair();

    // Generate a symmetric key (randomly or derived from a passphrase)
    let symmetric_key = ring::rand::generate::<[u8; 32]>(&OsRng).unwrap();

    // Encrypt the address with the symmetric key
    let encrypted_address = encrypt_address(address, &symmetric_key);

    // Store encrypted address and bloom filter
    store_encrypted_address_with_bloom_filter(&client, user_id, &encrypted_address, address).await;

    // Search by partial address
    let partial_address = "Main";
    let found_user_ids = search_by_partial_address(&client, partial_address).await;
    println!("Found User IDs: {:?}", found_user_ids);

    Ok(())
}

fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);

    let private_key_pem = private_key.to_pkcs8().unwrap();
    let public_key_pem = public_key.to_pkcs1().unwrap();

    (private_key_pem, public_key_pem)
}

fn encrypt_address(address: &str, key: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(&[0u8; 12]); // Use a fixed nonce for deterministic encryption

    cipher
        .encrypt(nonce, address.as_bytes())
        .expect("Encryption failure!")
}

fn decrypt_address(encrypted_address: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(&[0u8; 12]);

    cipher
        .decrypt(nonce, encrypted_address)
        .expect("Decryption failure!")
}

fn create_bloom_filter(data: &str) -> BloomFilter {
    let mut bloom = BloomFilter::with_rate(0.01, data.len());
    for i in 0..data.len() {
        for j in i + 1..=data.len() {
            bloom.insert(&data[i..j]);
        }
    }
    bloom
}

fn bloom_to_bytes(bloom: &BloomFilter) -> Vec<u8> {
    bloom.to_bytes()
}

fn bytes_to_bloom(bytes: &[u8]) -> BloomFilter {
    BloomFilter::from_bytes(bytes)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let address = "123 Main";
        let key = [0u8; 32];
        let encrypted_address = encrypt_address(address, &key);
        let decrypted_address = decrypt_address(&encrypted_address, &key);
        assert_eq!(address, decrypted_address);
    }
}
