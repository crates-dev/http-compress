pub(crate) mod brotli;
pub(crate) mod cfg;
pub(crate) mod compress;
pub(crate) mod deflate;
pub(crate) mod gzip;

pub use compress::r#type::*;

pub(crate) use ::brotli::Decompressor;
pub(crate) use flate2::{
    read::{DeflateDecoder, GzDecoder},
    write::{DeflateEncoder, GzEncoder},
    Compression,
};
pub(crate) use http_constant::*;
pub(crate) use std::{
    borrow::Cow,
    collections::HashMap,
    fmt,
    io::{prelude::*, BufReader, BufWriter, Read},
    str::FromStr,
};
