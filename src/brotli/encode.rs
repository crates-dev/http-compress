use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;

/// Compresses the given data using Gzip compression
///
/// # Parameters
/// - `data`: A reference to a `Vec<u8>` containing the data to be compressed.
///
/// # Returns
/// A `Vec<u8>` containing the compressed data, or an empty `Vec<u8>` if compression fails.
#[inline]
pub fn encode(data: &Vec<u8>) -> Vec<u8> {
    let mut encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::default());
    if let Err(_) = encoder.write_all(data) {
        return Vec::new();
    }
    encoder.finish().unwrap_or_else(|_| Vec::new())
}
