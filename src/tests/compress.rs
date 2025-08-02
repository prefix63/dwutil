use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use tempfile::tempdir;
use zip::write::FileOptions;

use crate::{Decompression, decompress::DecoderFactory};
/// Crea un archivo `hello.txt` con contenido en una carpeta temporal
fn create_sample_file(dir: &Path) -> io::Result<std::path::PathBuf> {
    let path = dir.join("hello.txt");
    let mut f = File::create(&path)?;
    writeln!(f, "Hello, world!")?;
    Ok(path)
}

/// Empaqueta un archivo en un .tar
fn create_tar(source: &Path, dest: &Path) -> io::Result<()> {
    let tar_gz = File::create(dest)?;
    let mut tar = tar::Builder::new(tar_gz);
    tar.append_path_with_name(source, "hello.txt")?;
    Ok(())
}

/// Comprime un archivo con gzip
fn compress_gz(source: &Path, dest: &Path) -> io::Result<()> {
    let input = fs::read(source)?;
    let f = File::create(dest)?;
    let mut encoder = flate2::write::GzEncoder::new(f, flate2::Compression::default());
    encoder.write_all(&input)?;
    encoder.finish()?;
    Ok(())
}

/// Comprime un archivo con xz
fn compress_xz(source: &Path, dest: &Path) -> io::Result<()> {
    let input = fs::read(source)?;
    let f = File::create(dest)?;
    let mut encoder = xz2::write::XzEncoder::new(f, 6);
    encoder.write_all(&input)?;
    encoder.finish()?;
    Ok(())
}

#[test]
fn test_tar() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempdir()?;
    let file = create_sample_file(tmp.path())?;
    let archive = tmp.path().join("file.tar");

    create_tar(&file, &archive)?;

    let bytes = fs::read(&archive)?;
    let dst = tmp.path().join("out");
    let decompression = Decompression::new::<crate::decompress::tar::TarFactory>().with_dst(&dst);

    decompression.extract(bytes)?;

    assert_eq!(
        fs::read_to_string(dst.join("hello.txt"))?,
        "Hello, world!\n"
    );
    Ok(())
}

#[test]
fn test_gz() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempdir()?;
    let file = create_sample_file(tmp.path())?;
    let archive = tmp.path().join("file.txt.gz");

    compress_gz(&file, &archive)?;

    let bytes = fs::read(&archive)?;
    let dst = tmp.path().join("out");
    let decompression = Decompression::new::<crate::decompress::gz::GzFactory>().with_dst(&dst);

    decompression.extract(bytes)?;

    println!("ASER");
    assert_eq!(fs::read_to_string(dst.join("target"))?, "Hello, world!\n");
    Ok(())
}

#[test]
fn test_xz() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempdir()?;
    let file = create_sample_file(tmp.path())?;
    let archive = tmp.path().join("file.txt.xz");

    compress_xz(&file, &archive)?;

    let bytes = fs::read(&archive)?;
    let dst = tmp.path().join("out");
    let decompression = Decompression::new::<crate::decompress::xz::XzFactory>().with_dst(&dst);

    decompression.extract(bytes)?;

    assert_eq!(fs::read_to_string(dst.join("target"))?, "Hello, world!\n");
    Ok(())
}

#[test]
fn test_tar_gz() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempdir()?;
    let file = create_sample_file(tmp.path())?;
    let tar = tmp.path().join("file.tar");
    let targz = tmp.path().join("file.tar.gz");

    create_tar(&file, &tar)?;
    compress_gz(&tar, &targz)?;

    let bytes = fs::read(&targz)?;
    let dst = tmp.path().join("out");
    let decompression =
        Decompression::new::<crate::decompress::targz::TarGzFactory>().with_dst(&dst);

    decompression.extract(bytes)?;

    assert_eq!(
        fs::read_to_string(dst.join("hello.txt"))?,
        "Hello, world!\n"
    );
    Ok(())
}

#[test]
fn test_tar_xz() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempdir()?;
    let file = create_sample_file(tmp.path())?;
    let tar = tmp.path().join("file.tar");
    let tarxz = tmp.path().join("file.tar.xz");

    create_tar(&file, &tar)?;
    compress_xz(&tar, &tarxz)?;

    let bytes = fs::read(&tarxz)?;
    let dst = tmp.path().join("out");
    let decompression =
        Decompression::new::<crate::decompress::tarxz::TarXzFactory>().with_dst(&dst);

    decompression.extract(bytes)?;

    assert_eq!(
        fs::read_to_string(dst.join("hello.txt"))?,
        "Hello, world!\n"
    );
    Ok(())
}

#[test]
fn test_zip() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let zip_path = dir.path().join("file.zip");

    let zip_file = File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = FileOptions::default();

    zip.start_file("hello.txt", options)?;
    zip.write_all(b"Hello, world!")?;
    zip.finish()?;

    let mut decoder = crate::decompress::zip::ZipFactory::from_file(zip_path)?;

    let output_dir = tempdir()?;

    decoder.extract(output_dir.path().to_path_buf())?;

    // Check if the output contrains the file
    let path = output_dir.path();
    let hello = path.join("hello.txt");
    assert!(hello.exists());
    let content = fs::read_to_string(hello).unwrap();
    assert_eq!(content, "Hello, world!");
    Ok(())
}
