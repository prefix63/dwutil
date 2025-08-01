use super::Hasher;

pub struct Md5;
impl Hasher for Md5 {
    fn compute(bytes: &[u8]) -> Result<String, String> {
        let hash = md5::compute(bytes);
        Ok(hex::encode(hash.as_slice()))
    }
}
