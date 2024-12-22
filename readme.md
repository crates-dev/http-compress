## http-compress

[![](https://img.shields.io/crates/v/http-compress.svg)](https://crates.io/crates/http-compress)
[![](https://docs.rs/http-compress/badge.svg)](https://docs.rs/http-compress)
[![](https://img.shields.io/crates/l/http-compress.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/http-compress/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/http-compress/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/HTTP-COMPRESS/)

[Api Docs](https://docs.rs/http-compress/latest/http-compress/)

> A lightweight library for decompressing HTTP responses supporting Brotli, Deflate, and Gzip.

## Features

## Installation

To use this crate, you can run cmd:

```shell
cargo add http-compress
```

## Use

```rust
use http_compress::*;
use http_type::Header;
use std::collections::HashMap;
let headers: Header = HashMap::new();
let data: Vec<u8> = vec![];
let body: Vec<u8> = Compress::from(&headers).decode(&data, 1024);
assert_eq!(body, data);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
