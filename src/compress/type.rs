/// Represents different compression algorithms supported by the library.
#[derive(Debug, PartialEq, Eq)]
pub enum Compress {
    /// Gzip compression algorithm.
    Gzip,
    /// Deflate compression algorithm.
    Deflate,
    /// Brotli compression algorithm.
    Br,
    /// Represents an unknown or unsupported compression algorithm.
    Unknown,
}
