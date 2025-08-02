# 📦 DWUTIL

A Rust library for downloading, verifying, decompressing, and storing files in a concurrent and customizable way.

Supports:

* ✅ HTTP downloads (via [`ureq`](https://docs.rs/ureq))
* ✅ Custom progress indicators (e.g., [`indicatif`](https://docs.rs/indicatif))
* ✅ Hash validation (SHA, MD5, etc.)
* ✅ Decompression (ZIP, GZ, TAR, XZ)
* ✅ Content-addressable storage (CAS)
* ✅ Multithreaded downloading

---

## ✨ Features

* **Download files** from the internet with progress updates
* **Validate** integrity using hashes
* **Decompress** archives to custom destinations
* **Exclude files/folders** during decompression
* **Store files** using content-addressable storage
* **Download multiple files concurrently**

### Cargo Features
| Feature | Purpose                                    |
| ------- | ------------------------------------------ |
| `sha`   | Add support to sha1 and sha2 hashing       |
| `md5`   | Add support to md5 hashing                 |
| `zip`   | Add support to zip decompression           |
| `tar`   | Add support to tar decompression           |
| `targz` | Add support to tar gz decompression        |
| `tarxz` | Add support to tar xz decompression        |
| `gz`    | Add support to gzip decompression          |
| `xz`    | Add support to xz decompression            |
| `indicatif`    | Add indicatif indicator bar         |

---

## 📦 Installation

```toml
# Cargo.toml
[dependencies]
dwutil = "0.0.1"
```

---

## 🚀 Quick Start

```rust
use dwutil::{Downloader, File, Decompression};

fn main() -> Result<(), String> {
    let file = File::new("https://example.com/archive.tar.gz")
        .with_path("downloads/archive.tar.gz")
        .with_size(1024 * 1024) // Optional
        .with_decompression(Decompression::new::<TarGzFactory>() // Your custom decoder type
            .with_dst("extracted/")
            .with_exclude("README.txt")
        );

    Downloader::new(MyIndicatorFactory::default())
        .with_file(file)
        .start()
}
```

---

## 📁 File Configuration

The `File` struct represents a downloadable file.

```rust
use sha2::Sha256;

let file = File::new("https://example.com/file.zip")
    .with_path("files/file.zip")
    .with_size(1_000_000)
    .with_hash(Hash::new::<Sha256>("expected_hash"))
    .with_decompression(decompression)
    .with_store(store);
```

### Optional settings:

* `.with_size(size)` – expected file size
* `.with_hash(hash)` – expected hash for integrity check
* `.with_decompression(...)` – automatically extract after download
* `.with_store(...)` – store using content-addressable logic

---

## 📂 Decompression

Set how to decompress the file and where to extract it:

```rust
let decompression = Decompression::new::<ZipDecoder>()
    .with_dst("output/")
    .with_exclude("docs/README.md");
```

### Decompression options:

* **Supported formats**: `.zip`, `.tar.gz`, `.xz`, `.gz`
* Exclude files/folders by relative path

---

## 🔒 Hash Validation

You can validate files using several hash types (e.g., SHA1, SHA256, MD5):

```rust
use dwutil::hash::Hash;
use sha2::Sha256;

let file = file.with_hash(Hash::new::<Sha256>("hex_hash_string"));
```

If the downloaded file doesn't match the hash, it will return an error.

---

## 🧱 Content-Addressable Storage

Use a CAS to deduplicate and organize files by content:

```rust
use dwutil::cas::default::DefaultStore;

let store = Arc::new(DefaultStore::new("objects"));

let file = file.with_store(store.clone());
```

Files will be stored in the store instead of the provided `path`.

---

## 📊 Progress Indicators

You can implement your own progress UI using the `Indicator` and `IndicatorFactory` traits.

Example using `indicatif`:

```rust
use dwutil::indicator::indicatif::IndicatifFactory;

// Use the default style bar
let factory = IndicatifFactory::new();

let dw = Downloader::new(factory);

```

By default the crate provides, a `SilentFactory` indicator and a `LogFactory` indicator

---

## 🔁 Download Multiple Files

You can download several files at once with concurrency:

```rust
Downloader::new(SilentFactory::new())
    .with_files(vec![file1, file2, file3])
    .with_max_current_downloads(3)
    .start()?;
```

---

## 🧪 Testing

Unit tests can be added inside the `tests` module and will run with:

```bash
cargo test
```

---

## 📚 Modules Overview

| Module       | Purpose                                       |
| ------------ | --------------------------------------------- |
| `cas`        | Store files using content-addressable methods |
| `decompress` | Decode and extract various archive types      |
| `hash`       | File hashing (SHA1, SHA256, MD5, etc.)        |
| `indicator`  | Progress bars, logging, error reporting       |
| `utils`      | Internal tools for copying, paths, etc.       |

---

## ✅ Requirements

* Rust 1.70+
* Internet connection (for actual downloads)
* Supported archive formats (ZIP, TAR, etc.)

---

## 🛠 Dependencies

* [`ureq`](https://docs.rs/ureq)
* [`tempfile`](https://docs.rs/tempfile)
* [`tracing`](https://docs.rs/tracing)
* (optional) [`indicatif`](https://docs.rs/indicatif)

---

## 📄 License

MIT


