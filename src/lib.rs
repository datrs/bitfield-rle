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

/// Encode a bitfield.
pub fn encode(buf: impl AsRef<[u8]>) -> Vec<u8> {
  let (enc, _) = encode_with_offset(&buf, 0);
  enc
}

/// Encode a bitfield at a specific offset
pub fn encode_with_offset(
  buf: impl AsRef<[u8]>,
  offset: usize,
) -> (Vec<u8>, usize) {
  let buf = buf.as_ref();
  let mut len = 0u64;
  let mut contiguous = false;
  let mut prev_bits = 0;
  let mut noncontiguous_bits = Vec::new();
  let mut enc = Vec::with_capacity(encode_len_with_offset(&buf, offset));

  for i in offset..buf.len() {
    if contiguous && buf[i] == prev_bits {
      len += 1;
      continue;
    } else if contiguous {
      write_contiguous(&mut enc, len, prev_bits);
    }

    if buf[i] == 0 || buf[i] == 255 {
      if !contiguous && i > offset {
        write_noncontiguous(&mut enc, &mut noncontiguous_bits);
      }
      len = 1;
      prev_bits = buf[i];
      contiguous = true;
    } else if !contiguous {
      noncontiguous_bits.push(buf[i]);
    } else {
      contiguous = false;
      noncontiguous_bits.push(buf[i]);
    }
  }

  if contiguous {
    write_contiguous(&mut enc, len, prev_bits);
  } else {
    write_noncontiguous(&mut enc, &mut noncontiguous_bits);
  }

  (enc, buf.len() - offset)
}

/// Writes a value for contiguous data to the encoded bitfield
fn write_contiguous(enc: &mut Vec<u8>, mut len: u64, prev_bits: u8) {
  len <<= 2;
  len += 1;
  if prev_bits == 255 {
    len += 2;
  }
  let mut varint = vec![0u8; varint::length(len)];
  varint::encode(len, &mut varint);
  enc.append(&mut varint);
}

/// Writes a value for noncontiguous data to the encoded bitfield
fn write_noncontiguous(enc: &mut Vec<u8>, noncontiguous_bits: &mut Vec<u8>) {
  let mut len = noncontiguous_bits.len() as u64;
  len <<= 1;
  let mut varint = vec![0u8; varint::length(len)];
  varint::encode(len, &mut varint);
  enc.append(&mut varint);
  enc.append(noncontiguous_bits);
}

/// Returns how many bytes a decoded bitfield will use.
pub fn encode_len(buf: impl AsRef<[u8]>) -> usize {
  encode_len_with_offset(&buf, 0)
}

/// Returns how many bytes an encoded bitfield will use, starting at a specific offset.
pub fn encode_len_with_offset(buf: impl AsRef<[u8]>, offset: usize) -> usize {
  let buf = buf.as_ref();
  let mut len = 0u64;
  let mut partial_len = 0u64;
  let mut contiguous = false;
  let mut prev_bits = 0;

  for i in offset..buf.len() {
    if contiguous && buf[i] == prev_bits {
      partial_len += 1;
      continue;
    } else if contiguous {
      len += varint::length(partial_len << 2) as u64;
    }

    if buf[i] == 0 || buf[i] == 255 {
      if !contiguous && i > offset {
        len += partial_len;
        len += varint::length(partial_len << 1) as u64;
      }
      partial_len = 1;
      prev_bits = buf[i];
      contiguous = true;
    } else if !contiguous {
      partial_len += 1;
    } else {
      partial_len = 1;
      contiguous = false;
    }
  }

  if contiguous {
    len += varint::length(partial_len << 2) as u64;
  } else {
    len += partial_len;
    len += varint::length(partial_len << 1) as u64;
  }

  len as usize
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
