use super::Hasher;

pub struct Md5;
impl Hasher for Md5 {
    type Err = String;
    fn compute(bytes: &[u8]) -> Result<String, Self::Err> {
        let hash = md5::compute(bytes);
        Ok(hex::encode(hash.as_slice()))
    }
}
