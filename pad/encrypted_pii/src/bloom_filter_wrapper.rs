use bloom::BloomFilter;

/// Wrapper for a Bloom filter with additional functionality.
pub struct BloomFilterWrapper(pub BloomFilter);

impl BloomFilterWrapper {
    /// Converts a Bloom filter to a byte vector.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the Bloom filter.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    /// Converts a byte vector to a Bloom filter.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte vector to convert.
    ///
    /// # Returns
    ///
    /// A `BloomFilterWrapper` created from the byte vector.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        BloomFilterWrapper(BloomFilter::from_bytes(bytes))
    }
}
