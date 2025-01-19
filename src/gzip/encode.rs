use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::BufWriter;

/// Compresses the given data using Gzip compression.
///
/// This function takes a byte vector containing data to be compressed, compresses
/// it using the `flate2` crate's `GzEncoder`, and returns the result as a vector of bytes.
///
/// # Arguments
/// - `data` - A reference to a `Vec<u8>` containing the data to be compressed.
/// - `buffer_size` - The buffer size to use for the writer. A larger buffer size
///   can improve performance for larger datasets.
///
/// # Returns
/// - `Vec<u8>` - The compressed data as a vector of bytes. If compression fails,
///   an empty `Vec<u8>` is returned.
#[inline]
pub fn encode(data: &Vec<u8>, buffer_size: usize) -> Vec<u8> {
    let encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::default());
    let mut buffered_writer: BufWriter<GzEncoder<Vec<u8>>> =
        BufWriter::with_capacity(buffer_size, encoder);
    if let Err(_) = buffered_writer.write_all(data) {
        return Vec::new();
    }
    match buffered_writer.into_inner() {
        Ok(encoder) => encoder.finish().unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}
