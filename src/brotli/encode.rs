use flate2::write::GzEncoder;
use flate2::Compression;
use std::borrow::Cow;
use std::io::prelude::*;

/// Compresses the given data using Gzip compression.
///
/// This function takes a byte slice of data and compresses it using the Gzip compression algorithm.
/// If the compression succeeds, the resulting compressed data is returned as a `Cow<Vec<u8>>`.
/// If any error occurs during the compression process, an empty `Vec<u8>` is returned.
///
/// # Parameters
/// - `data` - A reference to a byte slice (`&[u8]`) containing the data to be compressed.
///
/// # Returns
/// - `Cow<Vec<u8>>` - The compressed data as a `Cow<Vec<u8>>`. The compressed data is returned as an
///   owned `Vec<u8>`. If compression fails, an empty owned `Vec<u8>` is returned.
///
/// # Notes
/// - The compression process uses `GzEncoder` with the default compression level (`Compression::default()`).
/// - The use of `Cow` allows for optimization by avoiding unnecessary copying of data when not required.
#[inline]
pub fn encode(data: &[u8]) -> Cow<Vec<u8>> {
    let mut encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::default());
    if let Err(_) = encoder.write_all(data) {
        return Cow::Owned(Vec::new());
    }
    Cow::Owned(encoder.finish().unwrap_or_else(|_| Vec::new()))
}
