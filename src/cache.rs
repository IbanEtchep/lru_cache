pub trait Cache<K, V> {
    /// Create a new cache with a maximum size.
    fn with_capacity(max_size: usize) -> Self;

    /// Get a reference to the value associated with the given key.
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Add the given key-value to the cache.
    fn put(&mut self, key: K, value: V);

    /// Remove the given key from the cache.
    fn invalidate(&mut self, key: &K);
}