use std::{fs, path::PathBuf};

use tempfile::tempdir;

use crate::cas::{Store, default::DefaultStore};

use super::init_tracing;

#[test]
fn write_file() {
    init_tracing();
    const CONTENT: &[u8] = b"Hello World!";
    let loc = tempdir().unwrap();
    let path = loc.path();
    let dst = path.join("file.txt");
    let objects = path.join("objects");
    let store = DefaultStore::new(objects);
    store
        .create(CONTENT.to_vec(), PathBuf::from(dst.clone()))
        .unwrap();

    let read = fs::read_to_string(dst).unwrap();
    assert_eq!(read, String::from_utf8(CONTENT.to_vec()).unwrap());
}
