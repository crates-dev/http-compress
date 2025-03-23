pub(crate) mod brotli;
pub(crate) mod cfg;
pub(crate) mod compress;
pub(crate) mod deflate;
pub(crate) mod gzip;

pub use compress::r#type::*;
pub use twox_hash::XxHash3_64;

pub(crate) use ::brotli::Decompressor;
pub(crate) use core::hash::BuildHasherDefault;
pub(crate) use flate2::{
    Compression,
    read::{DeflateDecoder, GzDecoder},
    write::{DeflateEncoder, GzEncoder},
};
pub(crate) use http_constant::*;
pub(crate) use std::{
    borrow::Cow,
    collections::HashMap,
    fmt,
    io::{BufReader, BufWriter, Read, prelude::*},
    str::FromStr,
};
