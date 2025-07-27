<center>

## http-compress

[![](https://img.shields.io/crates/v/http-compress.svg)](https://crates.io/crates/http-compress)
[![](https://img.shields.io/crates/d/http-compress.svg)](https://img.shields.io/crates/d/http-compress.svg)
[![](https://docs.rs/http-compress/badge.svg)](https://docs.rs/http-compress)
[![](https://github.com/eastspire/http-compress/workflows/Rust/badge.svg)](https://github.com/eastspire/http-compress/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/http-compress.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/HTTP-COMPRESS/)

[Api Docs](https://docs.rs/http-compress/latest/http_compress/)

> A high-performance async library for HTTP compression/decompression, supporting Brotli, Deflate, and Gzip algorithms.

## Features

## Installation

To use this crate, you can run cmd:

```shell
cargo add http-compress
```

## Use

### Compress

```rust
use http_compress::*;
use core::hash::BuildHasherDefault;
use std::{borrow::Cow, collections::HashMap};

let headers: HashMap<_, _, BuildHasherDefault<XxHash3_64>> = HashMap::with_hasher(BuildHasherDefault::default());
let data: Vec<u8> = vec![];
let body: Cow<'_, Vec<u8>> = Compress::from(&headers).decode(&data, 1_024_000);
assert_eq!(*body, data);
```

### Encode

```rust
use http_compress::*;

let _ = Compress::Gzip.encode(&[], 1_024_000);
let _ = Compress::Deflate.encode(&[], 1_024_000);
let _ = Compress::Br.encode(&[], 1_024_000);
```

### Decode

```rust
use http_compress::*;

let _ = Compress::Gzip.decode(&[], 1_024_000);
let _ = Compress::Deflate.decode(&[], 1_024_000);
let _ = Compress::Br.decode(&[], 1_024_000);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
