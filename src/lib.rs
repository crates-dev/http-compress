pub(crate) mod brotli;
pub(crate) mod cfg;
pub(crate) mod compress;
pub(crate) mod deflate;
pub(crate) mod gzip;

pub use ahash::AHasher;
pub use compress::r#type::*;
pub use dashmap::DashMap;

pub(crate) use ::brotli::Decompressor;
pub(crate) use flate2::{
    Compression,
    read::{DeflateDecoder, GzDecoder},
    write::{DeflateEncoder, GzEncoder},
};
pub(crate) use http_constant::*;
pub(crate) use rayon::prelude::*;
pub(crate) use std::{
    borrow::Cow,
    fmt,
    io::{BufReader, BufWriter, Read, prelude::*},
    str::FromStr,
};
