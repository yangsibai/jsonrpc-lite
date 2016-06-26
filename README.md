jsonrpc-lite
====
JSON-RPC 2.0 Specification serialization for Rust.

[![Crates version][version-image]][version-url]
[![Build Status][travis-image]][travis-url]
[![Coverage Status][coveralls-image]][coveralls-url]
[![Crates downloads][downloads-image]][downloads-url]

Implementations:

- [redis-cli](https://github.com/iorust/redis-cli) redis CLI.

## API
### Documentation https://iorust.github.io/resp/resp

```Rust
extern crate resp;
use resp::{Value, encode, encode_slice, Decoder};
```

### RESP Values

```Rust
enum Value {
    /// Null bulk reply, $-1\r\n
    Null,
    /// Null array reply, *-1\r\n
    NullArray,
    /// For Simple Strings the first byte of the reply is "+"
    String(String),
    /// For Errors the first byte of the reply is "-"
    Error(String),
    /// For Integers the first byte of the reply is ":"
    Integer(i64),
    /// For Bulk Strings the first byte of the reply is "$"
    Bulk(String),
    /// For Bulk <binary> Strings the first byte of the reply is "$"
    BufBulk(Vec<u8>),
    /// For Arrays the first byte of the reply is "*"
    Array(Vec<Value>),
}
```

#### `value.is_null() -> bool`
#### `value.is_error() -> bool`
#### `value.encode() -> Vec<u8>`
#### `value.to_encoded_string() -> io::Result<String>`
#### `value.to_beautify_string() -> String`

### encode
#### `fn encode(value: &Value) -> Vec<u8>`
#### `fn encode_slice(array: &[&str]) -> Vec<u8>`

### Decoder
#### `Decoder.new() -> Self`
#### `Decoder.with_buf_bulk() -> Self`
#### `decoder.feed(buf: &[u8]) -> Result<(), io:Error>`
#### `decoder.read() -> Option<Value>`
#### `decoder.buffer_len() -> usize`
#### `decoder.result_len() -> usize`


[version-image]: https://img.shields.io/crates/v/resp.svg
[version-url]: https://crates.io/crates/resp

[travis-image]: http://img.shields.io/travis/iorust/resp.svg
[travis-url]: https://travis-ci.org/iorust/resp

[coveralls-image]: https://coveralls.io/repos/github/iorust/resp/badge.svg?branch=master
[coveralls-url]: https://coveralls.io/github/iorust/resp?branch=master

[downloads-image]: https://img.shields.io/crates/d/resp.svg
[downloads-url]: https://crates.io/crates/resp
