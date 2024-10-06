pub mod address;
pub mod bloom_filter_wrapper;
pub mod encrypted;
use address::Address;
use encrypted::{Encrypted, Searchable};

pub fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);

    let private_key_pem = private_key
        .to_pkcs8()
        .expect("Failed to convert private key");
    let public_key_pem = public_key.to_pkcs1().expect("Failed to convert public key");

    (private_key_pem, public_key_pem)
}

#[cfg(test)]
mod tests {
    use super::address::Address;
    use super::bloom_filter_wrapper::BloomFilterWrapper;
    use rand::rngs::OsRng;
    use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
    use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
    use sqlx::{query, query_as};
    use tokio_test::block_on;

    #[tokio::test]
    async fn test_address_encryption_and_bloom_filter() {
        // Use an in-memory SQLite database for testing
        let database_url = "sqlite::memory:";
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .unwrap();

        // Create necessary tables (you may need to adjust this based on your schema)
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS addresses (
                user_id INTEGER PRIMARY KEY,
                encrypted_address BLOB,
                bloom_filter BLOB
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        let user_id = 1;
        let address = Address::new("123 Main St".to_string());

        // Generate asymmetric key pair for user
        let (_private_key, _public_key) = generate_key_pair();

        // Generate a symmetric key (randomly or derived from a passphrase)
        let symmetric_key = ring::rand::generate::<[u8; 32]>(&OsRng).unwrap();

        // Encrypt the address with the symmetric key
        let encrypted_address = address.encrypt(&symmetric_key);

        // Store encrypted address and bloom filter
        store_encrypted_address_with_bloom_filter(&pool, user_id, &encrypted_address, &address)
            .await;

        // Search by partial address
        let partial_address = "Main";
        let found_user_ids = search_by_partial_address(&pool, partial_address).await;
        assert_eq!(found_user_ids, vec![user_id]);
    }
}
