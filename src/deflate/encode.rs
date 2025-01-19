use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::BufWriter;

/// Compresses the given data using Deflate compression
///
/// # Parameters
/// - `data`: A reference to a `Vec<u8>` containing the data to be compressed.
/// - `buffer_size`: The buffer size to use for the buffered writer.
///
/// # Returns
/// A `Vec<u8>` containing the compressed data, or an empty `Vec<u8>` in case of error.
#[inline]
pub fn encode(data: &Vec<u8>, buffer_size: usize) -> Vec<u8> {
    let encoder: DeflateEncoder<Vec<u8>> = DeflateEncoder::new(Vec::new(), Compression::default());
    let mut buffered_writer: BufWriter<DeflateEncoder<Vec<u8>>> =
        BufWriter::with_capacity(buffer_size, encoder);
    if let Err(_) = buffered_writer.write_all(data) {
        return Vec::new();
    }
    match buffered_writer.into_inner() {
        Ok(encoder) => encoder.finish().unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}
