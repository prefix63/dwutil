use std::io::Write;

use sha1::Sha1;
use tempfile::NamedTempFile;

use crate::{
    hash::{Hash, md5::Md5},
    tests::init_tracing,
};

#[test]
fn sha_check_bytes_test() {
    init_tracing();
    let hash = Hash::<Sha1>::new("430ce34d020724ed75a196dfc2ad67c77772d169".to_owned());
    assert_eq!(hash.check_bytes(b"hello world!".to_vec()), Some(()));
    assert_eq!(hash.check_bytes(b"world hello!".to_vec()), None);
}

#[test]
fn sha_check_file_test() {
    init_tracing();
    let mut file = NamedTempFile::new().unwrap();
    file.write(b"This is a example file\n").unwrap();

    let hash = Hash::<Sha1>::new("59773469265971aa2b3d70c939f7179ffcd95014".to_owned());
    assert_eq!(hash.check_file(file).unwrap(), Some(()));
}

#[test]
fn md5_check_bytes_test() {
    init_tracing();
    let hash = Hash::<Md5>::new("fc3ff98e8c6a0d3087d515c0473f8677".to_owned());
    assert_eq!(hash.check_bytes(b"hello world!".to_vec()), Some(()));
    assert_eq!(hash.check_bytes(b"world hello!".to_vec()), None);
}

#[test]
fn md5_check_file_test() {
    init_tracing();
    let mut file = NamedTempFile::new().unwrap();
    file.write(b"This is a example file\n").unwrap();

    let hash = Hash::<Md5>::new("2bdd613e96bcbbf006a7b9909979923f".to_owned());
    assert_eq!(hash.check_file(file).unwrap(), Some(()));
}
