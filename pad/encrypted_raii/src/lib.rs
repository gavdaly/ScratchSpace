pub mod address;
pub mod bloom_filter_wrapper;
pub mod encrypted;
use address::Address;
use encrypted::{Encrypted, Searchable};

#[cfg(test)]
mod tests {
    use super::address::Address;
    use super::bloom_filter_wrapper::BloomFilterWrapper;
    use rand::rngs::OsRng;
    use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{query, query_as, PgPool};
    use tokio_test::block_on;

    fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key =
            RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
        let public_key = RsaPublicKey::from(&private_key);

        let private_key_pem = private_key
            .to_pkcs8()
            .expect("Failed to convert private key");
        let public_key_pem = public_key.to_pkcs1().expect("Failed to convert public key");

        (private_key_pem, public_key_pem)
    }

    async fn store_encrypted_address_with_bloom_filter(
        pool: &PgPool,
        user_id: i32,
        encrypted_address: &[u8],
        address: &Address,
    ) {
        let bloom_filter = address.create_bloom_filter();
        let bloom_bytes = bloom_filter.to_bytes();

        query!(
            "INSERT INTO user_data (user_id, encrypted_address, bloom_filter) VALUES ($1, $2, $3)",
            user_id,
            encrypted_address,
            bloom_bytes
        )
        .execute(pool)
        .await
        .expect("Failed to store encrypted address and bloom filter");
    }

    async fn search_by_partial_address(pool: &PgPool, partial_address: &str) -> Vec<i32> {
        let partial_bloom = Address::new(partial_address.to_string()).create_bloom_filter();
        let partial_bloom_bytes = partial_bloom.to_bytes();

        let rows = query!(
            "SELECT user_id FROM user_data WHERE bloom_filter @> $1::bytea",
            partial_bloom_bytes
        )
        .fetch_all(pool)
        .await
        .expect("Failed to search by partial address");

        rows.into_iter().map(|row| row.user_id).collect()
    }

    #[tokio::test]
    async fn test_address_encryption_and_bloom_filter() {
        let database_url = "postgres://user:password@localhost/yourdb";
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
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
