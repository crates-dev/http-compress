//! http-compress
//!
//! A high-performance async library for HTTP compression/decompression,
//! supporting Brotli, Deflate, and Gzip algorithms. Provides both compression
//! and decompression capabilities with optimized memory usage,
//! ideal for HTTP clients/servers and network programming.

pub(crate) mod brotli;
pub(crate) mod cfg;
pub(crate) mod compress;
pub(crate) mod deflate;
pub(crate) mod gzip;

pub use compress::r#type::*;
pub use twox_hash::XxHash3_64;

pub(crate) use ::brotli::Decompressor;
pub(crate) use compress::r#const::*;
pub(crate) use core::hash::BuildHasherDefault;
pub(crate) use flate2::{
    Compression,
    read::{DeflateDecoder, GzDecoder},
    write::{DeflateEncoder, GzEncoder},
};
pub(crate) use std::{
    borrow::Cow,
    collections::HashMap,
    fmt,
    io::{BufReader, BufWriter, Read, prelude::*},
    str::FromStr,
};
