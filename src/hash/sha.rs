use super::Hasher;

impl<T: sha2::Digest> Hasher for T {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let mut sha = T::new();
        sha.update(bytes);
        let hash = sha.finalize();
        Ok(hex::encode(hash))
    }
}
