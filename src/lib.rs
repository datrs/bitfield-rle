#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

/// Returns how many bytes are needed to encode the bitfield.
pub fn encode_len(buf: &[u8]) -> usize {
  unimplemented!();
}

/// Returns how many bytes a decoded bitfield will use.
pub fn decode_len(buf: &[u8]) -> usize {
  unimplemented!();
}
