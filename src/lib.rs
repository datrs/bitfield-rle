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

/// Encode a bitfield.
pub fn encode(reader: impl AsRef<Vec<u8>>) -> Vec<u8> {
  let offset = 0;
  encode_with_offset(reader, offset)
}

/// Encode a bitfield at a specific offset
pub fn encode_with_offset(
  _reader: impl AsRef<Vec<u8>>,
  _offset: usize,
) -> Vec<u8> {
  unimplemented!();
}

/// Decode an encoded bitfield.
pub fn decode(buf: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  let (bitfield, _) = decode_with_offset(&buf, 0)?;
  Ok(bitfield)
}

/// Decode an encoded bitfield, starting at a specific offset.
pub fn decode_with_offset(
  buf: impl AsRef<[u8]>,
  mut offset: usize,
) -> Result<(Vec<u8>, usize)> {
  let buf = buf.as_ref();
  let mut bitfield = vec![0; decode_len_with_offset(&buf, offset)?];
  let mut next = 0u64;
  let mut ptr = 0;

  while offset < buf.len() {
    offset += varint::decode_with_offset(buf, offset, &mut next);
    let repeat = next & 1;
    let len = if repeat > 0 {
      (next >> 2) as usize
    } else {
      (next >> 1) as usize
    };

    if repeat > 0 {
      if next & 2 > 0 {
        for i in 0..len {
          bitfield[ptr + i] = 255;
        }
      }
    } else {
      for i in 0..len {
        bitfield[ptr + i] = buf[offset + i];
      }
      offset += len;
    }

    ptr += len;
  }

  Ok((bitfield, buf.len() - offset))
}

/// Returns how many bytes a decoded bitfield will use.
pub fn decode_len(buf: impl AsRef<[u8]>) -> Result<usize> {
  decode_len_with_offset(&buf, 0)
}

/// Returns how many bytes a decoded bitfield will use, starting at a specific offset.
pub fn decode_len_with_offset(
  buf: impl AsRef<[u8]>,
  mut offset: usize,
) -> Result<usize> {
  let buf = buf.as_ref();
  let mut len = 0;
  let mut next = 0u64;

  while offset < buf.len() {
    offset += varint::decode_with_offset(buf, offset, &mut next);
    let repeat = next & 1;

    let slice = if repeat > 0 {
      (next >> 2) as usize
    } else {
      (next >> 1) as usize
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
