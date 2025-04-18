use crate::*;

/// Compresses the given data using the Deflate compression algorithm.
///
/// This function takes a byte slice of data and compresses it using the Deflate compression algorithm.
/// The data is written to a buffered writer to optimize performance. The compressed data is returned
/// as a `Cow<Vec<u8>>`. If compression succeeds, the resulting compressed data is returned as an
/// owned `Vec<u8>`. If any error occurs during the compression process, an empty `Vec<u8>` is returned.
///
/// # Parameters
/// - `data` - A reference to a byte slice (`&[u8]`) containing the data to be compressed.
/// - `buffer_size` - The buffer size to use for the buffered writer. A larger buffer size can
///   improve performance for larger datasets.
///
/// # Returns
/// - `Cow<Vec<u8>>` - The compressed data as a `Cow<Vec<u8>>`. If compression is successful, the
///   compressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
///
/// # Notes
/// - The compression process uses the `DeflateEncoder` with the default compression level
///   (`Compression::default()`).
/// - The use of a `BufWriter` optimizes the writing of data to the encoder by reducing the number
///   of write operations.
pub fn encode(data: &[u8], buffer_size: usize) -> Cow<Vec<u8>> {
    let encoder: DeflateEncoder<Vec<u8>> = DeflateEncoder::new(Vec::new(), Compression::default());
    let mut buffered_writer: BufWriter<DeflateEncoder<Vec<u8>>> =
        BufWriter::with_capacity(buffer_size, encoder);
    if let Err(_) = buffered_writer.write_all(data) {
        return Cow::Owned(Vec::new());
    }
    match buffered_writer.into_inner() {
        Ok(encoder) => Cow::Owned(encoder.finish().unwrap_or_else(|_| Vec::new())),
        Err(_) => Cow::Owned(Vec::new()),
    }
}
