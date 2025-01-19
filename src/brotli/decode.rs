use brotli::Decompressor;
use std::{borrow::Cow, io::Read};

#[inline]
pub fn decode(data: &[u8], buffer_size: usize) -> Cow<Vec<u8>> {
    let mut decompressor: Decompressor<&[u8]> = Decompressor::new(data, buffer_size);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match decompressor.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
