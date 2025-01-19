use flate2::read::GzDecoder;
use std::{
    borrow::Cow,
    io::{BufReader, Read},
};

/// Decompresses the given Gzip-compressed data.
///
/// This function takes a byte slice of Gzip-compressed data and decompresses it using the `GzDecoder`
/// from the `flate2` crate. The decompression is done in a buffered manner to optimize performance.
/// If decompression is successful, the result is returned as an owned `Vec<u8>`. If an error occurs
/// during decompression, an empty `Vec<u8>` is returned.
///
/// # Parameters
/// - `data` - A reference to a byte slice (`&[u8]`) containing the Gzip-compressed data.
/// - `buffer_size` - The buffer size to use for the buffered reader. A larger buffer size can improve
///   performance for larger datasets.
///
/// # Returns
/// - `Cow<Vec<u8>>` - The decompressed data as a `Cow<Vec<u8>>`. If decompression is successful, the
///   decompressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
///
/// # Notes
/// - The decompression is done using the `GzDecoder` from the `flate2` crate.
/// - A `BufReader` is used to buffer the data during decompression, which improves performance.
/// - If an error occurs during the decompression process, an empty `Vec<u8>` is returned to avoid
///   panics and to ensure the function always returns a valid value.
#[inline]
pub fn decode(data: &[u8], buffer_size: usize) -> Cow<Vec<u8>> {
    let decoder: GzDecoder<&[u8]> = GzDecoder::new(data);
    let mut buffered_reader: BufReader<GzDecoder<&[u8]>> =
        BufReader::with_capacity(buffer_size, decoder);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match buffered_reader.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
