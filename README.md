# bitfield-rle
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

A run-length-encoder that compresses bitfields.

The encoder uses a compact format and will only run length encode sequences of
bits if it compresses the bitfield. The encoded bitfield should therefore always
be smaller or the same size as the original bitfield with the exception of a 1-6
byte header.

Since this uses run-length-encoding, you'll get the best compression results if
you have longer sequences of the same bit in your bitfield.

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate bitfield_rle as rle;
extern crate sparse_bitfield as bitfield;

use bitfield::Bitfield;

let mut bits = Bitfield::new(1024);
bits.set(400, true);

let len = bits.byte_len();
let mut bytes = Vec::with_capacity(len);
bits.to_bytes(bytes);

let len = rle::encode_len(bytes);
let enc = Vec::with_capacity(len);
rle::encode(bytes, enc);
assert_eq!(enc.len(), 6);

let len = rle::decode_len(enc);
let dec = Vec::with_capacity(len);
rle::decode(enc, dec);

let bits = Bitfield::from_bytes(dec);
assert_eq!(bits.get(400), true);
```

## Format
The encoded bitfield is a series of compressed and uncompressed bit sequences.
All sequences start with a header that is a varint.

If the last bit is set in the varint (it is an odd number) then a header
represents a compressed bit sequence.

```txt
S = varint([l << 2, b << 1, 1])

where
  S = compressed sequence
  l = byte length of sequence
  b = bit
```

If the last bit is *not* set then a header represents an uncompressed bit
sequence.

```txt
S = [varint([l << 1, 0]), b]

where
  S = uncompressed sequence
  l = byte length of bitfield
  b = bitfield
```

## Installation
```sh
$ cargo add bitfield-rle
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/bitfield-rle.svg?style=flat-square
[2]: https://crates.io/crates/bitfield-rle
[3]: https://img.shields.io/travis/datrs/bitfield-rle.svg?style=flat-square
[4]: https://travis-ci.org/datrs/bitfield-rle
[5]: https://img.shields.io/crates/d/bitfield-rle.svg?style=flat-square
[6]: https://crates.io/crates/bitfield-rle
[7]: https://docs.rs/bitfield-rle/badge.svg
[8]: https://docs.rs/bitfield-rle
