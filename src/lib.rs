#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate varinteger as varint;

/// Returns how many bytes are needed to encode the bitfield.
pub fn encode_len(_buf: &[u8]) -> usize {
  unimplemented!();
}

/// Returns how many bytes a decoded bitfield will use.
pub fn decode_len(_buf: &[u8]) -> usize {
  unimplemented!();
}

/// Encode a bitfield.
pub fn encode() {
  unimplemented!();
}

/// Decode an encoded bitfield.
pub fn decode(buf: &[u8]) {
  decode_with_offset(&buf, 0)
}

/// Decode an encoded bitfield at a specific offset.
pub fn decode_with_offset(buf: &[u8], mut offset: usize) {
  unimplemented!();
}

/// Returns how many bytes a decoded bitfield will use.
pub fn decoding_len(
  buf: &[u8]
) -> Result<usize, Box<std::error::Error + 'static>> {
  decoding_len_with_offset(&buf, 0)
}

/// Returns how many bytes a decoded bitfield will use at a specific offset.
// TODO: use failure::Error;
pub fn decoding_len_with_offset(
  buf: &[u8],
  mut offset: usize,
) -> Result<usize, Box<std::error::Error + 'static>> {
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

  // TODO: replace with ensure!() call.
  // ensure!(!offset > buffer.len(), "Invalid RLE bitfield {} > {}", offset, buffer.len());
  Ok(len)
}
