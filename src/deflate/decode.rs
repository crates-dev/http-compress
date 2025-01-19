use flate2::read::DeflateDecoder;
use std::{
    borrow::Cow,
    io::{BufReader, Read},
};

#[inline]
pub fn decode(data: &[u8], buffer_size: usize) -> Cow<Vec<u8>> {
    let decoder: DeflateDecoder<&[u8]> = DeflateDecoder::new(data);
    let mut buffered_reader: BufReader<DeflateDecoder<&[u8]>> =
        BufReader::with_capacity(buffer_size, decoder);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match buffered_reader.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
