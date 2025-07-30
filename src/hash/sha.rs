use sha1::{Digest, Sha1};
use sha2::{Sha224, Sha256, Sha512_224, Sha512_256};

use super::Hasher;

impl Hasher for Sha1 {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let mut sha = Sha1::new();
        sha.update(bytes);
        let hash = sha.finalize();
        Ok(hex::encode(hash))
    }
}
impl Hasher for Sha224 {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let mut sha = Sha224::new();
        sha.update(bytes);
        let hash = sha.finalize();
        Ok(hex::encode(hash))
    }
}
impl Hasher for Sha256 {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let mut sha = Sha256::new();
        sha.update(bytes);
        let hash = sha.finalize();
        Ok(hex::encode(hash))
    }
}
impl Hasher for Sha512_224 {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let mut sha = Sha512_224::new();
        sha.update(bytes);
        let hash = sha.finalize();
        Ok(hex::encode(hash))
    }
}
impl Hasher for Sha512_256 {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let mut sha = Sha512_256::new();
        sha.update(bytes);
        let hash = sha.finalize();
        Ok(hex::encode(hash))
    }
}
