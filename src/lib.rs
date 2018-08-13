#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate failure;
extern crate varinteger as varint;

use failure::Error;

use std::convert::AsRef;

/// Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Returns how many bytes are needed to encode the bitfield.
pub fn encode_len(_buf: &[u8]) -> usize {
  unimplemented!();
}

/// Returns how many bytes a decoded bitfield will use.
pub fn decode_len(_buf: &[u8]) -> usize {
  unimplemented!();
}

/// Encode a bitfield.
pub fn encode(reader: impl AsRef<Vec<u8>>) -> Vec<u8> {
  let offset = 0;
  encode_with_offset(reader, offset)
}

/// Encode a bitfield at a specific offset
pub fn encode_with_offset(
  reader: impl AsRef<Vec<u8>>,
  offset: usize,
) -> Vec<u8> {
  unimplemented!();
}

/// Decode an encoded bitfield.
pub fn decode(buf: impl AsRef<Vec<u8>>) -> Result<(Vec<u8>, usize)> {
  let _reader = buf.as_ref();
  let output = Vec::new();
  let len = decode_with_offset(&output, 0)?;
  Ok((output, len))
}

/// Decode an encoded bitfield at a specific offset.
pub fn decode_with_offset(
  buf: impl AsRef<Vec<u8>>,
  offset: usize,
) -> Result<usize> {
  let buf = buf.as_ref();
  let _len = decoding_len_with_offset(&buf, offset)?;
  unimplemented!();
}

/// Returns how many bytes a decoded bitfield will use.
pub fn decoding_len(buf: impl AsRef<Vec<u8>>) -> Result<usize> {
  decoding_len_with_offset(&buf, 0)
}

/// Returns how many bytes a decoded bitfield will use at a specific offset.
pub fn decoding_len_with_offset(
  buf: impl AsRef<Vec<u8>>,
  mut offset: usize,
) -> Result<usize> {
  let buf = buf.as_ref();
  let mut len = 0;
  let mut val = 0u64;

  while offset < buf.len() {
    let next = varint::decode(buf, &mut val);
    offset += next;
    let repeat = next & 1;

    let slice = if repeat > 0 {
      next - (next & 3) / 4
    } else {
      next / 2
    };

    len += slice;
    if repeat == 0 {
      offset += slice;
    }
  }

  ensure!(
    !offset > buf.len(),
    "Invalid RLE bitfield {} > {}",
    offset,
    buf.len()
  );

  Ok(len)
}
