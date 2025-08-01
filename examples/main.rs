use dwutil::{Decompression, Downloader, File, decompress::tarxz::TarXzFactory, hash::Hash};
use sha2::Sha256;
use tracing::Level;

fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    Downloader::new(dwutil::indicator::log::LogFactory::new())
        .with_file(File::new("https://github.com/zen-browser/desktop/releases/download/1.14.9b/zen.linux-x86_64.tar.xz")
            .with_path("zen.tar.xz")
            .with_hash(Hash::new::<Sha256>("3d5264cd45d41c074eb7c929fbc2a8eeae4f3beeef765ae8b3628c7a2b4423a1"))
            .with_decompression(Decompression::new::<TarXzFactory>().with_dst("./zen"))
            .with_size(81_407_468))
        .start()
        .unwrap();
}
